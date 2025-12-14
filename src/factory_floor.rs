use itertools::Itertools;
use itertools::izip;
use ndarray::{Array1, Array2, Axis, Ix1, Slice, s, concatenate};
use ndarray_inverse::Inverse;
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
    joltage: Vec<u8>,
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
                    .map(|x| x.parse::<u8>().unwrap())
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
    pub fn optimal_joltage_pushing(&self)->usize{
        let c_n = vec![-1.0; self.button_configurations.len()];
        let c_b = vec![0.0; self.joltage.len()];
        let c = Array1::from_vec([&c_n[..], &c_b[..]].concat());
        let b = Array1::from_vec(self.joltage.clone()).mapv(|x| x as f64);

        let N = self.get_sparse_matrix().mapv(|x| x as f64);
        let B = Array2::<f64>::eye(self.joltage.len());

        println!("{:?} {:?}", N.shape(), B.shape());
        let A = concatenate(Axis(0), &[N.view(), B.view()]).unwrap();
        println!("{}", A);

        let solution = Self::revised_simplex(c, A.t().to_owned(), b);
        todo!()
    }

    fn revised_simplex(c: Array1<f64>, a: Array2<f64>, b: Array1<f64>) -> Array1<f64> {
        let n_index = c.len() - b.len();
        let mut n_slicer = Vec::from_iter(0..n_index);
        let mut b_slicer = Vec::from_iter(n_index..c.len());
        println!("{:?} {:?}", n_slicer, b_slicer);

        let mut s_n = c.select(Axis(0), n_slicer.as_slice());
        let x_n = vec![0f64; n_index];
        let x_b = b.to_vec();
        let mut x = Array1::from_vec([&x_n[..], &x_b[..]].concat());
        let N = a.select(Axis(1), &n_slicer);
        let B = a.select(Axis(1), &b_slicer);
        let b_inv = B.inv().unwrap();
        let c_b = c.select(Axis(0), b_slicer.as_slice());
        let c_n = c.select(Axis(0), n_slicer.as_slice());
        let lambda = b_inv.dot(&c_b);
        s_n = c_n - N.t().dot(&lambda);
        while s_n.iter().any(|x| x < &0f64) {
            let incomming_index = s_n.iter().position_min_by(|a, b| a.total_cmp(b)).unwrap();
            println!("{}, {}", s_n, incomming_index);
            let d = a.slice(s![.., incomming_index]);
            println!("{:?}", d);
            let x_b = x.select(Axis(0), &b_slicer);
            let coeff = &x_b / &d;
            println!("coeff {}", coeff);
            let outgoing_index = coeff.iter().position_min_by(|a, b| a.total_cmp(b)).unwrap();
            for (index, temp) in izip!(&b_slicer, d) {
                x[*index] -= temp * coeff[outgoing_index];
            }
            x[incomming_index] = coeff[outgoing_index];

            let n_removed = n_slicer.remove(incomming_index);
            let b_removed = b_slicer.remove(outgoing_index);
            println!("{}->{}", incomming_index, outgoing_index);

            println!("x : {}", x);

            n_slicer.push(b_removed);
            b_slicer.push(n_removed);

            let N = a.select(Axis(1), &n_slicer);
            let B = a.select(Axis(1), &b_slicer);
            let b_inv = B.inv().unwrap();
            let c_b = c.select(Axis(0), b_slicer.as_slice());
            let c_n = c.select(Axis(0), n_slicer.as_slice());
            let lambda = b_inv.dot(&c_b);
            s_n = c_n - N.t().dot(&lambda);

            println!("{}", B);
            println!("{}", N);
            println!("{}, {}, {}", x, lambda, s_n);
        }

        let result_index = Vec::from_iter(0..n_index);
        x.select(Axis(0), &result_index)
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

        //let joltage_pushes = machine.optimal_joltage_pushing();
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

    #[test]
    fn test_wiki_example() {
        let c = Array1::<f64>::from_vec(vec![-2.0, -3.0, -4.0, 0.0, 0.0]);
        let a = Array2::<f64>::from_shape_vec(
            (2, 5),
            vec![3.0, 2.0, 1.0, 1.0, 0.0, 2.0, 5.0, 3.0, 0.0, 1.0],
        )
        .unwrap();
        let b = Array1::from_vec(vec![10.0, 15.0]);
        let x = Machine::revised_simplex(c, a, b);

        assert_eq!(x, Array1::from_vec(vec![0.0, 0.0, 5.0]));
    }

}
