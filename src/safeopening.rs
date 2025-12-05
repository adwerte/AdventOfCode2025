use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input/1_safecode.txt").unwrap();
    interpet_string(input);
}

fn interpet_string(input: String) -> (usize, usize) {
    let codes = input.split("\n");
    let mut dial = SafeDial::new(50);
    for code in codes {
        if code.starts_with("R") {
            let number: i64 = code.to_string()[1..].parse().unwrap();
            dial.right(number);
        } else if code.starts_with("L") {
            let number: i64 = code.to_string()[1..].parse().unwrap();
            dial.left(number);
        }
    }
    let number = dial.code_positions.iter().filter(|x| **x == 0).count();
    println!("Your number is {}", number);
    let ticks = dial.turns + number;
    println!("Your ticks are {}", ticks);
    (number, ticks)
}

struct SafeDial {
    current_position: i64,
    turns: usize,
    code_positions: Vec<u8>,
}

impl SafeDial {
    pub fn new(start_position: i64) -> Self {
        Self {
            current_position: start_position,
            turns: 0,
            code_positions: Vec::new(),
        }
    }

    fn right(&mut self, number: i64) {
        self.current_position += number;
        while self.current_position > 99 {
            self.current_position -= 100;
            if self.current_position != 0 {
                self.turns += 1;
            }
        }
        self.code_positions.push(self.current_position as u8)
    }

    fn left(&mut self, number: i64) {
        let mut iterations: i64 = 0;
        if self.current_position == 0 {
            iterations -= 1i64;
        }
        self.current_position -= number;
        while self.current_position < 0 {
            self.current_position += 100;
            iterations += 1i64;
        }
        self.turns += iterations as usize;
        self.code_positions.push(self.current_position as u8)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82".to_string();
        let number = interpet_string(input);
        assert_eq!(number.0, 3);
        assert_eq!(number.1, 6);
    }

    #[test]
    fn test_example_part() {
        let input = "L68\nL30\nR48\nL5".to_string();
        let number = interpet_string(input);
        assert_eq!(number.0, 1);
        assert_eq!(number.1, 2);
    }
}
