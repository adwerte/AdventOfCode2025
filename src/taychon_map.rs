use ndarray::{ArcArray2, Array2};
use threadpool::ThreadPool;
use std::sync::Arc;
use std::{clone, collections::HashMap, fs, io::Cursor, rc::Rc, sync::mpsc, thread};

pub fn main() {
    let bank = fs::read_to_string("./input/7_taychon_map.txt").unwrap();
    let bank = bank.trim();
    let mut map = TaychonMap::new();
    let mut splits = bank.split('\n');
    let depth = splits.clone().count();
    let mut rows = String::new();
    let mut ringer = splits.next().unwrap().to_string();
    let width = ringer.len();
    for split in splits {
        //println!("{}", ringer);
        rows.push_str(&ringer);
        ringer = map.fill_row(&ringer, split);
    }
    rows.push_str(&ringer);

    println!("Day 7: Splits: {}", map.splits);

    let paths = TaychonMap::get_paths_threads(&rows, depth, width);

    println!("Day 7: Paths {}", paths);
    // 3334
}

struct TaychonMap {
    splits: usize,
    paths: usize,
}

impl TaychonMap {
    pub fn new() -> Self {
        TaychonMap {
            splits: 0,
            paths: 1,
        }
    }

    pub fn fill_row(&mut self, above_row: &str, current_row: &str) -> String {
        let mut above_chars = above_row.chars();
        let mut current_chars = current_row.chars();
        let mut new_row = String::new();
        loop {
            let above_char = above_chars.next();
            let current_char = current_chars.next();
            let Some(above_char) = above_char else {
                break;
            };
            let Some(current_char) = current_char else {
                break;
            };
            if above_char == 'S' || above_char == '|' {
                if current_char == '.' {
                    new_row.push('|');
                } else if current_char == '^' {
                    new_row.pop().unwrap();
                    new_row.push('|');
                    new_row.push('^');
                    new_row.push('|');
                    self.splits += 1;
                    above_chars.next();
                    current_chars.next();
                }
            } else {
                new_row.push('.');
            }
        }

        new_row
    }
    pub fn get_paths(bank: &str, depth: usize, width: usize) -> usize {
        let bank = bank.chars().collect();
        let array = Array2::from_shape_vec((depth, width), bank).unwrap();
        let mut targets = vec![[0usize, width / 2]];
        let mut paths: usize = 0;
        let mut depest = 0usize;
        loop {
            // println!("tagerts len {}", targets.len());
            let cursor_option = targets.pop();
            let Some(mut cursor) = cursor_option else {
                break;
            };
            cursor[0] += 1;
            if cursor[0] >= depth {
                paths += 1;
                continue;
            }
            if depest < cursor[0] {
                println!("{}/{}", cursor[0] + 1, depth);
                depest = cursor[0]
            }
            let current_char = array[cursor];
            if current_char == '|' {
                targets.push(cursor);
            } else if current_char == '^' {
                cursor[1] -= 1;
                targets.push(cursor);
                cursor[1] += 2;
                targets.push(cursor);
            }
        }

        paths
    }

    pub fn get_paths_threads(bank: &str, depth: usize, width: usize) -> usize {
        let bank = bank.chars().collect();
        let array = Arc::new(Array2::from_shape_vec((depth, width), bank).unwrap());
        let mut targets = vec![[0usize, width / 2]];
        let mut paths = 1;
        while !targets.is_empty() {
            let (tx, rx) = mpsc::channel();
            let pool = ThreadPool::new(32);
            while let Some(start) = targets.pop() {
                let clone_tx = tx.clone();
                let clone_array = array.clone();
                pool.execute(move || {
                    let output = Self::get_rightmost_path(&clone_array, start);
                    for starts in output {
                        clone_tx.send(starts).unwrap()
                    }
                });
            }
            drop(tx);
            for recieved in rx {
                targets.push(recieved);
            }
            println!("Threads will be started: {}", targets.len());
            paths += targets.len();
        }

        paths
    }

    fn get_rightmost_path(array: &Arc<Array2<char>>, start: [usize; 2]) -> Vec<[usize; 2]> {
        let mut cursor = start;
        let mut new_starts = Vec::new();
        loop {
            cursor[0] += 1;
            if cursor[0] >= array.shape()[0] {
                break;
            }
            let current_char = array[cursor];
            if current_char == '|' {
                continue;
            } else if current_char == '^' {
                new_starts.push([cursor[0], cursor[1] + 1]);
                cursor[1] -= 1;
                continue;
            } else {
                panic!("Fuck are you doing here?")
            }
        }

        new_starts
    }
}

#[cfg(test)]
mod test {

    use crate::taychon_map::TaychonMap;

    #[test]
    fn test_example() {
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let depth = splits.clone().count();
        let mut ringer = splits.next().unwrap().to_string();
        let width = ringer.len();
        let mut rows = String::new();
        for split in splits {
            println!("{}", ringer);
            rows.push_str(&ringer);
            ringer = map.fill_row(&ringer, split);
        }
        println!("{}", ringer);
        rows.push_str(&ringer);

        assert_eq!(map.splits, 21);

        println!("depth: {}, {}", depth, width);

        let paths = TaychonMap::get_paths_threads(&rows, depth, width);
        assert_eq!(paths, 40);
    }
    #[test]
    fn test_sub_example() {
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let depth = splits.clone().count();
        let mut ringer = splits.next().unwrap().to_string();
        let width = ringer.len();
        let mut rows = String::new();
        for split in splits {
            println!("{}", ringer);
            rows.push_str(&ringer);
            ringer = map.fill_row(&ringer, split);
        }
        println!("{}", ringer);
        rows.push_str(&ringer);

        assert_eq!(map.splits, 6);

        println!("depth: {}, {}", depth, width);

        let paths = TaychonMap::get_paths_threads(&rows, depth, width);
        assert_eq!(paths, 8);
    }

    #[test]
    fn test_sub_example_2() {
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let depth = splits.clone().count();
        let mut ringer = splits.next().unwrap().to_string();
        let width = ringer.len();
        let mut rows = String::new();
        for split in splits {
            println!("{}", ringer);
            rows.push_str(&ringer);
            ringer = map.fill_row(&ringer, split);
        }
        println!("{}", ringer);
        rows.push_str(&ringer);

        assert_eq!(map.splits, 9);

        println!("depth: {}, {}", depth, width);

        let paths = TaychonMap::get_paths_threads(&rows, depth, width);
        assert_eq!(paths, 11);
    }
}
