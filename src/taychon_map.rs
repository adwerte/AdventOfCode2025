use ndarray::Array2;
use threadpool::ThreadPool;
use std::sync::Arc;
use std::{fs, sync::mpsc};

pub fn main() {
    let bank = fs::read_to_string("./input/7_taychon_map.txt").unwrap();
    let bank = bank.trim();
    let mut splits = bank.split('\n');
    let top_row = splits.next().unwrap();
    let mut ringer: Vec<usize> = top_row.chars().map(|x| if x == 'S' {1} else {0}).collect();
    let mut map = TaychonMap::new();
    for split in splits {
        //println!("{}", ringer);
        ringer = map.fill_row(ringer, split);
    }

    println!("Day 7: Splits: {}", map.splits);

    let paths : usize = ringer.iter().sum();
    println!("Day 7: Paths {}", paths);
    // 3334
}

struct TaychonMap {
    splits: usize,
}

impl TaychonMap {
    pub fn new() -> Self {
        TaychonMap {
            splits: 0,
        }
    }

    pub fn fill_row(&mut self, above_row: Vec<usize>, current_row: &str) -> Vec<usize> {
        let mut current_chars = current_row.chars();
        let mut new_row = vec![0; above_row.len()];
        for (index, above) in above_row.iter().enumerate() {
            let current_char = current_chars.next();
            let Some(current_char) = current_char else {
                break;
            };
            if above > &0 {
                if current_char == '.' {
                    new_row[index] += above;
                } else if current_char == '^' {
                    new_row[index - 1] += above;
                    new_row[index + 1] += above;
                    self.splits += 1;
                }
            }
        }

        new_row
    }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
        let top_row = splits.next().unwrap();
        let mut ringer: Vec<usize> = top_row.chars().map(|x| if x == 'S' {1} else {0}).collect();
        for split in splits {
            println!("{:?}", ringer);
            ringer = map.fill_row(ringer, split);
        }
        println!("{:?}", ringer);

        assert_eq!(map.splits, 21);


        let paths : usize = ringer.iter().sum();
        assert_eq!(paths, 40);
    }
    #[test]
    fn test_sub_example() {
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let top_row = splits.next().unwrap();
        let mut ringer: Vec<usize> = top_row.chars().map(|x| if x == 'S' {1} else {0}).collect();
        for split in splits {
            println!("{:?}", ringer);
            ringer = map.fill_row(ringer, split);
        }
        println!("{:?}", ringer);

        assert_eq!(map.splits, 6);

        let paths : usize = ringer.iter().sum();
        assert_eq!(paths, 8);
    }

    #[test]
    fn test_sub_example_2() {
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let top_row = splits.next().unwrap();
        let mut ringer: Vec<usize> = top_row.chars().map(|x| if x == 'S' {1} else {0}).collect();
        for split in splits {
            println!("{:?}", ringer);
            ringer = map.fill_row(ringer, split);
        }
        println!("{:?}", ringer);

        assert_eq!(map.splits, 9);

        let paths : usize = ringer.iter().sum();
        assert_eq!(paths, 13);
    }
}
