use std::fs;


pub fn main() {
    let input = fs::read_to_string("./input/3_joltage.txt").unwrap();
    let output = interpet_input(&input);
    let sum : usize = output.iter().sum();
    println!("Day 3: joltage sum = {}", sum)
}

fn interpet_input(input: &str) -> Vec<usize> {
    let mut output = Vec::new();
    let banks = input.split("\n");
    for bank in banks {
        let joltage = get_2_battery_joltage(bank);
        output.push(joltage);
    }
    output
}
fn get_2_battery_joltage(bank : &str)-> usize{
    let mut largest_number: usize = 0;
    for first_index in 0..bank.len(){
        for second_index in first_index + 1..bank.len(){
            let digits : String = [
                bank.chars().nth(first_index).unwrap(),
                bank.chars().nth(second_index).unwrap(),
            ].iter().collect();
            let number:usize = digits.parse().unwrap();
            largest_number = largest_number.max(number)
        }
    }


    largest_number
}

fn get_battery_joltage_concated(bank :&str, n : usize)-> usize{
    let vec = get_battery_joltage(bank, n);
    vec.iter().fold(0, |acc, elem| acc * 10 + *elem as usize)
}


fn get_battery_joltage(bank : &str, n : usize) -> Vec<u8> {
    let mut digits = vec![0; n];
    let mut iterator = bank.chars();
    let mut current_digit = 1;
    digits[0] = iterator.next().unwrap().to_digit(10).unwrap() as u8;


    for (index, char) in iterator.enumerate(){
        let digit = char.to_digit(10).unwrap() as u8;
        let value = 't: { for old_index in (1..digits.len() +1).rev(){
            println!("found {} {}, {}, {}", digits[old_index -1], digit, bank.len() - index, digits.len() - old_index);
            if digits[old_index - 1] < digit && bank.len() - index >= digits.len() - old_index{
                digits[old_index - 1] = digit;
                break 't Some(old_index - 1);
            }
        }
        
        digits.iter().position(|x|* x == 0u8)
        
        };
        println!("{:?}", digits);
        if let Some(index) = value{
            digits[index] = digit
        }
        
    }
    println!("{:?}", digits);
    digits
}

fn update_last_smallest_digit(digits : &Vec<u8>, updator : u8, index : usize) -> usize {
    let new_option = index.checked_sub(1);
    if let Some(new_index) = new_option && digits[new_index] > updator && index <= digits.len(){
        return index;
    }

    let value = if let Some(new_index) = new_option{
        update_last_smallest_digit(digits, updator, new_index)
    } else {
        index
    };
    value
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
        let number = get_2_battery_joltage("818181911112111");
        assert_eq!(number, 92)
    }

    #[test]
    fn test_fuck(){
        for i in (0..2).rev() {
            println!("{}", i)
        }
        assert!(false)
    }
}