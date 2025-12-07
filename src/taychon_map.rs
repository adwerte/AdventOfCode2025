use std::{collections::HashMap, fs, rc::Rc};

pub fn main(){
    let bank = fs::read_to_string("./input/7_taychon_map.txt").unwrap();
    let bank = bank.trim();
    let mut map = TaychonMap::new();
    let mut splits = bank.split('\n');
    let mut ringer = splits.next().unwrap().to_string();
    for split in splits{
        //println!("{}", ringer);
        ringer = map.fill_row(&ringer, split);
    }
    println!("Day 7: Splits: {}", map.splits);
}

struct TaychonPath{
    pub parents : Vec<Rc<Self>>,
}


struct TaychonMap{
    splits : usize,
}

impl TaychonMap{
    pub fn new() -> Self {
        TaychonMap { splits: 0 }
    }

    pub fn fill_row_rc(&mut self, above_row : HashMap<usize, Rc<TaychonPath>>, current_row : &str) -> HashMap<usize, Rc<TaychonPath>> {
        let mut current_chars = current_row.chars();
        let mut new_row: HashMap<usize, Rc<TaychonPath>> = HashMap::new();
        let mut index = 0usize;
        loop{
            let parent = above_row.get(&index);
            let current_char = current_chars.next();
            let Some(current_char) = current_char else {
                break;
            };
            let Some(parent) = parent else {
                index += 1;
                continue;
            };
            if current_char == '.' {

                new_row.insert(index, Rc::new(TaychonPath { parents:  vec![parent.clone()]}));
            } else if current_char == '^' {
                let temp_rc = new_row.get(& (index - 1));
                if let Some( mut temp) = temp_rc {
                    todo!()
                } else {
                    new_row.insert(index - 1, Rc::new(TaychonPath { parents: vec![parent.clone()]}));
                };
            }
        }
        todo!()
    }


    pub fn fill_row(&mut self, above_row : &str, current_row : &str) -> String{
        let mut above_chars = above_row.chars();
        let mut current_chars = current_row.chars();
        let mut new_row = String::new();
        loop{
            let above_char = above_chars.next();
            let current_char = current_chars.next();
            let Some(above_char) = above_char else {
                break;
            };
            let Some(current_char) = current_char else {
                break;
            };
            if above_char == 'S' || above_char == '|'{
                if current_char == '.'{
                    new_row.push('|');
                } else if current_char == '^'{
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
}


#[cfg(test)]
mod test{
    use crate::taychon_map::TaychonMap;

    #[test]
    fn test_example(){
        let bank = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";
        let bank = bank.trim();
        let mut map = TaychonMap::new();
        let mut splits = bank.split('\n');
        let mut ringer = splits.next().unwrap().to_string();
        for split in splits{
            println!("{}", ringer);
            ringer = map.fill_row(&ringer, split);
        }

        assert_eq!(map.splits, 22)
    }
}