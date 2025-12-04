use std::fs;


pub fn main() {
    let input = fs::read_to_string("./input/3_joltage.txt").unwrap();
    let output = interpet_input(&input, 2);
    let sum : usize = output.iter().sum();
    println!("Day 3: 2 joltage sum = {}", sum);
    let output = interpet_input(&input, 12);
    let sum : usize = output.iter().sum();
    println!("Day 3: 12 joltage sum = {}", sum)
}

fn interpet_input(input: &str, n : usize) -> Vec<usize> {
    let mut output = Vec::new();
    let banks = input.split("\n");
    for bank in banks {
        let joltage = get_battery_joltage_concated(bank, n);
        output.push(joltage);
    }
    output
}

fn get_battery_joltage_concated(bank :&str, n : usize)-> usize{
    let vec = get_battery_joltage(bank, n);
    vec.iter().fold(0, |acc, elem| acc * 10 + *elem as usize)
}


fn get_battery_joltage(bank : &str, n : usize) -> Vec<u8> {
    let mut digits = vec![0; n];
    let iterator = bank.chars();

    for (bank_index, char) in iterator.enumerate(){
        let digit = char.to_digit(10).unwrap() as u8;
        let remained = bank.len() - bank_index;
        let mut positions = Vec::new();
        for (digit_index, old_digit) in digits.iter().enumerate(){
            let required = n - digit_index;
            if required <= remained && *old_digit < digit{
                positions.push(digit_index);
            }
        }
        if let Some(index) = positions.iter().min(){
            digits[*index] = digit;
            for temp in digits.iter_mut().skip(index + 1){
                *temp = 0;
            }
        }
    }

    digits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_from_987654321111111(){
        let number = get_battery_joltage_concated("987654321111111", 2);
        assert_eq!(number, 98)
    }
    
    #[test]
    fn test_2_from_811111111111119(){
        let number = get_battery_joltage_concated("811111111111119", 2);
        assert_eq!(number, 89);
    }

    #[test]
    fn test_2_from_234234234234278(){
        let number = get_battery_joltage_concated("234234234234278", 2);
        assert_eq!(number, 78)
    }

    #[test]
    fn test_2_from_818181911112111(){
        let number = get_battery_joltage_concated("818181911112111", 2);
        assert_eq!(number, 92)
    }

    #[test]
    fn test_12_from_987654321111111(){
        let number = get_battery_joltage_concated("987654321111111", 12);
        assert_eq!(number, 987654321111)
    }
    
    #[test]
    fn test_12_from_811111111111119(){
        let number = get_battery_joltage_concated("811111111111119", 12);
        assert_eq!(number, 811111111119);
    }

    #[test]
    fn test_12_from_234234234234278(){
        let number = get_battery_joltage_concated("234234234234278", 12);
        assert_eq!(number, 434234234278)
    }

    #[test]
    fn test_12_from_818181911112111(){
        let number = get_battery_joltage_concated("818181911112111", 12);
        assert_eq!(number, 888911112111)
    }

}