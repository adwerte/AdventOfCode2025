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

fn get_12_battery_joltage(bank : &str) -> usize {
    let mut digits: [usize; 12] = [0; 12];
    digits[0] = bank.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;

    todo!()


}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_2_from_987654321111111(){
        let number = get_2_battery_joltage("987654321111111");
        assert_eq!(number, 98)
    }
    
    #[test]
    fn test_2_from_811111111111119(){
        let number = get_2_battery_joltage("811111111111119");
        assert_eq!(number, 89)
    }

    #[test]
    fn test_2_from_234234234234278(){
        let number = get_2_battery_joltage("234234234234278");
        assert_eq!(number, 78)
    }

    #[test]
    fn test_2_from_818181911112111(){
        let number = get_2_battery_joltage("818181911112111");
        assert_eq!(number, 92)
    }
}