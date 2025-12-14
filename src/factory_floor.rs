use good_lp::Expression;
use good_lp::ProblemVariables;
use good_lp::variable;
use good_lp::{Solution, SolverModel, default_solver};
use itertools::Itertools;
use ndarray::Array;
use ndarray::{Array1, Array2};
use regex::Regex;
use std::fs;

pub fn main() {
    let bank = fs::read_to_string("./input/10_factory_floor.txt").unwrap();
    let bank = bank.trim();
    let mut presses = 0;
    let mut joltage = 0;
    for line in bank.split('\n') {
        let machine = Machine::new(line);
        presses += machine.optimal_pushing_iterative();
        joltage += machine.optimal_joltage_pushing();
    }

    println!("Day 10: Total Pressese {}.", presses);
    // 491

    println!("Day 10: Joltage presses {}", joltage);
    // 19669
    // 20582
    // 20615
    // 20617
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
    pub fn optimal_joltage_pushing(&self) -> usize {
        let c = Array1::from_vec(vec![1.0; self.button_configurations.len()]);
        let matrix = self.get_sparse_matrix().mapv(|x| x as f64);
        let b = Array::from_vec(self.joltage.iter().map(|x| *x as f64).collect());

        let solution = Self::revised_simplex(c, matrix.t().to_owned(), b.clone());
        solution.iter().sum::<f64>().round() as usize
    }

    fn revised_simplex(c: Array1<f64>, a: Array2<f64>, b: Array1<f64>) -> Array1<f64> {
        let mut variables = ProblemVariables::new();
        let mut expression = Expression::with_capacity(c.len());
        let mut vec_variables = Vec::new();
        for index in 0..c.len() {
            let variable = variables.add(variable().min(0).integer());
            vec_variables.push(variable);
            expression += variable * c[index];
        }
        let mut constraints = Vec::new();
        for (row, b_sub) in a.rows().into_iter().zip(b.iter()) {
            let mut row_exp = Expression::with_capacity(row.len());
            for (coeff, var) in row.iter().zip(vec_variables.iter()) {
                row_exp += *var * *coeff;
            }
            let constraint = row_exp.eq(*b_sub);
            constraints.push(constraint)
        }
        let solution = variables
            .minimise(expression)
            .using(default_solver)
            .with_all(constraints)
            .solve()
            .unwrap();
        let mut vec_solution = Vec::new();
        for variable in vec_variables {
            vec_solution.push(solution.value(variable))
        }

        Array1::from_vec(vec_solution)
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

        let joltage_pushes = machine.optimal_joltage_pushing();
        assert_eq!(joltage_pushes, 10)
    }

    #[test]
    fn test_machine_2() {
        let bank = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        let machine = Machine::new(bank);
        let pushes = machine.optimal_pushing_iterative();

        assert_eq!(pushes, 3);

        let joltage_pushes = machine.optimal_joltage_pushing();
        assert_eq!(joltage_pushes, 12)
    }

    #[test]
    fn test_machine_3() {
        let bank = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let machine = Machine::new(bank);
        let pushes = machine.optimal_pushing_iterative();

        assert_eq!(pushes, 2);

        let joltage_pushes = machine.optimal_joltage_pushing();
        assert_eq!(joltage_pushes, 11)
    }

    // #[test]
    // fn test_wiki_example() {
    //     let c = Array1::<f64>::from_vec(vec![-2.0, -3.0, -4.0]);
    //     let a = Array2::<f64>::from_shape_vec(
    //         (2, 3),
    //         vec![3.0, 2.0, 1.0, 2.0, 5.0, 3.0,],
    //     )
    //     .unwrap();
    //     let b = Array1::from_vec(vec![10.0, 15.0]);
    //     let x = Machine::revised_simplex(c, a, b);

    //     assert_eq!(x, Array1::from_vec(vec![0.0, 0.0, 5.0]));
    // }
}
