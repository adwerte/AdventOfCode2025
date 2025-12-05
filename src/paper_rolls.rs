use itertools::izip;
use ndarray::Array2;
use ndarray_ndimage::{BorderMode, convolve};
use std::fs;

pub fn main() {
    let input = fs::read_to_string("./input/4_paper.txt").unwrap();
    let sum = moving_paperoll_count(input.trim(), 4);
    println!("Day 4: sum = {}", sum);
}

fn moving_paperoll_count(bank: &str, required_weight: usize) -> usize {
    let splits = bank.split("\n");
    let mut vec_of_vec = Vec::new();
    let mut shape = (0, 0);

    for split in splits {
        let vec: Vec<usize> = split
            .chars()
            .map(|x| if x == '@' { 1 } else { 0 })
            .collect();
        shape.0 += 1;
        shape.1 = vec.len();
        vec_of_vec.extend_from_slice(&vec);
    }
    let array = Array2::from_shape_vec(shape, vec_of_vec).unwrap();

    let kernel: Vec<usize> = vec![1, 1, 1, 1, 0, 1, 1, 1, 1];
    let kernel = Array2::from_shape_vec([3, 3], kernel).unwrap();

    let weights = convolve(&array, &kernel, BorderMode::Constant(0), 0);

    let removed = vec![0; array.iter().count()];
    let mut removed = Array2::from_shape_vec(shape, removed).unwrap();
    for (value, weight, removed) in izip!(array.iter(), weights.iter(), removed.iter_mut()) {
        if value == &1 && weight < &required_weight {
            *removed = 1;
        }
    }

    removed.iter().sum()
}

fn moving_paperoll_weights(bank: &str, n: usize) -> Vec<isize> {
    let mut weights = vec![0; bank.len()];
    for (index, weight) in weights.iter_mut().enumerate() {
        let start = index.checked_sub(n);
        let cur_char = bank.chars().nth(index);
        if cur_char == Some('.') || cur_char.is_none() {
            continue;
        } else if cur_char == Some('@') {
            *weight -= 1;
        }
        let end = index + n + 1;
        let end = if end <= bank.len() { Some(end) } else { None };
        // fuck, the one which are removed should not be counted.
        // AND it is a 2D problem.
        let area = if let Some(start) = start
            && let Some(end) = end
        {
            let mut neighbors = 0;
            for r in index + 1..end {
                let char = bank.chars().nth(r);
                if char == Some('@') {
                    neighbors += 1
                } else {
                    break;
                }
            }
            for l in (start..index).rev() {
                let char = bank.chars().nth(l);
                if char == Some('@') {
                    neighbors += 1
                } else {
                    break;
                }
            }

            println!("{}", neighbors);

            &bank[start..end]
        } else if let Some(start) = start {
            &bank[start..]
        } else if let Some(end) = end {
            &bank[..end]
        } else {
            ""
        };
        for char in area.chars() {
            if char == '@' {
                *weight += 1;
            }
        }
        println!("{}, {}, {:?}-{}-{:?}", area, weight, start, index, end);
    }
    weights
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let count = moving_paperoll_count(input, 4);
        assert_eq!(count, 13)
    }
}
