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
    let mut array = Array2::from_shape_vec(shape, vec_of_vec).unwrap();
    let start: usize = array.iter().sum();

    let mut change = 1;
    while change > 0 {
        let removed = moving_paperoll_weights(&array, required_weight);
        change = removed.iter().sum();
        for (remove, value) in izip!(removed.iter(), array.iter_mut()) {
            *value -= remove;
        }
    }

    let end: usize = array.iter().sum();
    start - end
}

fn moving_paperoll_weights(array: &Array2<usize>, required_weight: usize) -> Array2<usize> {
    let kernel: Vec<usize> = vec![1, 1, 1, 1, 0, 1, 1, 1, 1];
    let kernel = Array2::from_shape_vec([3, 3], kernel).unwrap();

    let weights = convolve(array, &kernel, BorderMode::Constant(0), 0);

    let removed = vec![0; array.iter().count()];
    let mut removed = Array2::from_shape_vec(array.raw_dim(), removed).unwrap();
    for (value, weight, removed) in izip!(array.iter(), weights.iter(), removed.iter_mut()) {
        if value == &1 && weight < &required_weight {
            *removed = 1;
        }
    }

    removed
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let count = moving_paperoll_count(input, 4);
        assert_eq!(count, 43)
    }
}
