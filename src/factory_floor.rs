use itertools::Itertools;
use ndarray::{Array1, Array2};
use regex::Regex;
use std::fs;

pub fn main() {
    let bank = fs::read_to_string("./input/10_factory_floor.txt").unwrap();
    let bank = bank.trim();
    let mut presses = 0;
    for line in bank.split('\n') {
        let machine = Machine::new(line);
        presses += machine.optimal_pushing_iterative();
    }

    println!("Day 10: Total Pressese {}.", presses)
    // 491
}

#[derive(Debug)]
struct Machine {
    target_indicators: Vec<bool>,
    button_configurations: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    pub fn new(bank: &str) -> Self {
        let indicator_finder = Regex::new(r"\[(.*?)\]|\((.*?)\)|\{(.*?)\}").unwrap();
        let mut result = indicator_finder
            .captures_iter(bank)
            .map(|x| x.extract::<1>());
        let indicator_str = result.next().unwrap().1[0];
        let target_indicators: Vec<bool> = indicator_str.chars().map(|x| x == '#').collect();

        let mut button_configurations: Vec<Vec<usize>> = Vec::new();
        let mut joltage = Vec::new();
        for button in result {
            if button.0.ends_with('}') {
                joltage = button.1[0]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
            } else {
                let connections = button.1[0]
                    .split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                button_configurations.push(connections);
            }
        }

        Self {
            target_indicators,
            button_configurations,
            joltage,
        }
    }

    fn get_sparse_matrix(&self) -> Array2<u8> {
        let mut full_vec: Vec<u8> = Vec::new();
        for button in &self.button_configurations {
            let mut button_vec = vec![0; self.target_indicators.len()];
            for index in button {
                button_vec[*index] = 1;
            }
            full_vec.extend(button_vec);
        }
        //println!("{}->{:?}", full_vec.len(), (self.button_configurations.len(), self.target_indicators.len()));

        Array2::from_shape_vec(
            (
                self.button_configurations.len(),
                self.target_indicators.len(),
            ),
            full_vec,
        )
        .unwrap()
    }

    pub fn optimal_pushing_iterative(&self) -> usize {
        // Use a linear algebra solution that has the button configs as a matrix that connects an input of button precesses to a series of lights on-or-off.
        let matrix = self.get_sparse_matrix();
        let mut min = None;
        for i in 1..10 {
            let combinations = (0..self.button_configurations.len() as u8)
                .combinations(i)
                .unique();
            for combination in combinations {
                let mut combi = vec![0; self.button_configurations.len()];
                for index in &combination {
                    combi[*index as usize] = 1u8;
                }
                let domain = Array1::from_vec(combi.clone());

                let projection = matrix.t().dot(&domain);
                let current_indicators: Vec<bool> = projection.iter().map(|x| x % 2 == 1).collect();
                //println!("{:?} -> {:?} -> {:?} == {:?}", combination, combi, current_indicators, self.target_indicators);

                if current_indicators == self.target_indicators {
                    min = Some(i);
                    break;
                }
            }
            if min.is_some() {
                break;
            }
        }

        min.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_machine_1() {
        let bank = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        let machine = Machine::new(bank);
        let pushes = machine.optimal_pushing_iterative();

        assert_eq!(pushes, 2);
    }

    #[test]
    fn test_machine_2() {
        let bank = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = Machine::new(bank);
        let pushes = machine.optimal_pushing_iterative();

        assert_eq!(pushes, 3);
    }

    #[test]
    fn test_machine_3() {
        let bank = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = Machine::new(bank);
        let pushes = machine.optimal_pushing_iterative();

        assert_eq!(pushes, 2);
    }
}
