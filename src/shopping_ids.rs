use std::fs;

use itertools::Itertools;

pub fn main() {
    let bank = fs::read_to_string("./input/5_shopping_ids.txt").unwrap();

    let (range, bank) = interpet_input(&bank);

    let mut fresh: usize = 0;
    for astring in bank.split('\n') {
        let number: usize = astring.parse().unwrap();
        if range.value_in(number) {
            fresh += 1;
        }
    }

    println!("Day 5: Fresh produce {}", fresh);

    let sum = range.get_full_range_sum();

    println!("Day 5: Fresh produce ids {}", sum);
    // guess 1: 353084569925896
    // guess 2: 352716206375547
}

fn interpet_input(bank: &str) -> (Ranger, &str) {
    let bank = bank.trim();
    let mut split = bank.split("\n\n");

    let ranges = split.next().unwrap();
    let ranges = Ranger::new(ranges);
    let ids = split.next().unwrap();

    (ranges, ids)
}

#[derive(Debug)]
struct Ranger {
    ranges: Vec<[usize; 2]>,
}

impl Ranger {
    pub fn new(range_bank: &str) -> Self {
        let mut ranges = Vec::new();
        for text_range in range_bank.split('\n') {
            let mut split = text_range.split('-');
            let start: usize = split.next().unwrap().parse().unwrap();
            let end: usize = split.next().unwrap().parse().unwrap();
            ranges.push([start, end])
        }
        let ranges = ranges
            .into_iter()
            .sorted_by(|x, y| x[0].cmp(&y[0]))
            .collect();

        Self { ranges }
    }

    pub fn value_in(&self, value: usize) -> bool {
        let mut test = false;
        for range in &self.ranges {
            if range[0] <= value && value <= range[1] {
                test = true;
                break;
            }
        }
        test
    }

    pub fn get_full_range_sum(&self) -> usize {
        let mut sum = 0;
        let mut max = 0;
        for range in &self.ranges {
            if max < range[0] {
                max = range[0];
            }
            let end = range[1];
            let diff = if let Some(temp) = end.checked_sub(max) {
                max = end + 1;
                temp + 1
            } else {
                0
            };
            //println!("{:?}, {}, {}", range, max, diff);
            sum += diff;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let bank = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

        let (range, bank) = interpet_input(bank);
        println!("{:?}", range);

        let mut fresh: usize = 0;
        for astring in bank.split('\n') {
            let number: usize = astring.parse().unwrap();
            if range.value_in(number) {
                fresh += 1;
            }
        }

        assert_eq!(fresh, 3);

        let sum = range.get_full_range_sum();

        assert_eq!(sum, 14);
    }
}
