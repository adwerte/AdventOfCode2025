use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input/2_product_ids.txt").unwrap();
    let temp = input.trim();
    let output = interpet_input(temp);
    println!("day 2: output {:?}", output);
    let asum: usize = output.iter().sum();
    println!("Day 2: the sum is {}", asum)
}

fn interpet_input(input: &str) -> Vec<usize> {
    let mut output_total = Vec::new();
    let splits = input.split(',');
    for split in splits {
        let range: Vec<&str> = split.split('-').collect();
        //println!("{:?}", range);
        let output = interpet_range_bruteforce_part_2(range[0], range[1]);
        output_total.extend(output);
    }

    output_total
}
fn interpet_range_bruteforce(start: &str, end: &str) -> Vec<usize> {
    let mut start_number: usize = start.parse().unwrap();
    let end_number: usize = end.parse().unwrap();
    let mut output = Vec::new();
    while start_number <= end_number {
        let astring = start_number.to_string();
        if astring.len().is_multiple_of(2) {
            let repeater = astring.len() / 2;
            let first_digits = &astring[..repeater];
            let second_digits = &astring[repeater..];
            if first_digits == second_digits {
                output.push(start_number);
            }
        }
        start_number += 1
    }
    output
}
fn interpet_range_bruteforce_part_2(start: &str, end: &str) -> Vec<usize> {
    let mut start_number: usize = start.parse().unwrap();
    let end_number: usize = end.parse().unwrap();
    let mut output = Vec::new();
    while start_number <= end_number {
        let astring = start_number.to_string();
        let max_index = astring.len() / 2;
        for i in 1..max_index + 1 {
            let target: Vec<char> = astring[..i].chars().collect();
            let chunks = astring
                .chars()
                .collect::<Vec<char>>()
                .chunks(i)
                .all(|x| x == target);
            if chunks {
                output.push(start_number);
                //println!("{}", start_number);
                break;
            }
        }
        start_number += 1
    }
    output
}

fn interpet_range(start: &str, end: &str) -> Vec<usize> {
    let first_number_len = start.len() / 2;
    let start_number: usize = start.parse().unwrap();
    let end_number: usize = end.parse().unwrap();
    let mut change = end_number - start_number;
    let subtractor = 10usize.pow(first_number_len as u32);
    let mut first_number: usize = if first_number_len != 0 {
        start[..first_number_len].parse().unwrap()
    } else {
        start.chars().next().unwrap().to_digit(10).unwrap() as usize
    };
    let mut output = Vec::new();

    //println!("subtractor {}", subtractor);
    //println!("first number len {}", first_number_len);
    //println!("change {}", change);

    loop {
        let digits = first_number.to_string() + &first_number.to_string();
        let number: usize = digits.parse().unwrap();
        println!("testing digit {}", digits);
        if number <= end_number && number >= start_number {
            output.push(number);
        }

        if change < subtractor {
            break;
        }
        first_number += 1;
        change -= subtractor;
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_95_115() {
        let result = interpet_range_bruteforce("95", "115");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 99);

        let result = interpet_range_bruteforce_part_2("95", "115");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 99);
        assert_eq!(result[1], 111);
    }

    #[test]
    fn test_11_22() {
        let result = interpet_range_bruteforce_part_2("11", "22");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 11);
        assert_eq!(result[1], 22);
    }

    #[test]
    fn test_998_1012() {
        let result = interpet_range_bruteforce("998", "1012");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1010);

        let result = interpet_range_bruteforce_part_2("998", "1012");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], 999);
        assert_eq!(result[1], 1010);
    }

    #[test]
    fn test_1_20() {
        let result = interpet_range_bruteforce_part_2("1", "20");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 11);
    }

    #[test]
    fn test_9987328_10008767() {
        let result = interpet_range_bruteforce("9987328", "10008767");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 10001000);
    }

    #[test]
    fn example() {
        let result = interpet_input(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let asum: usize = result.iter().sum();
        assert_eq!(asum, 4174379265);
        //assert_eq!(result.len(), 8);
    }
}
