use std::fs;

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


struct TaychonMap{
    splits : usize,
}

impl TaychonMap{
    pub fn new() -> Self {
        TaychonMap { splits: 0 }
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