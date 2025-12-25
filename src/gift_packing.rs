use std::rc::Rc;
use std::{fs, str::Split};

use ndarray::Array2;

pub fn main() {
    let bank = fs::read_to_string("./input/12_gift_packing.txt").unwrap();
    let bank = bank.trim();
    let mut lines = bank.split('\n');
    let mut gifts = Vec::new();
    let mut trees: Vec<Tree> = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with(':') {
            gifts.push(Rc::new(Gift::new(&mut lines)));
        } else {
            trees.push(Tree::new(line, &gifts));
        }
    }
    let fitting_trees: Vec<&Tree> = trees.iter().filter(|x| x.get_simple_fits()).collect();
    println!(
        "Day 12: {} Trees where gifts might fit out of {}",
        fitting_trees.len(),
        trees.len(),
    );
    // 497
}
#[derive(Debug)]
struct Gift {
    shape: Array2<bool>,
}

impl Gift {
    pub fn new<'a>(lines: &mut Split<'a, char>) -> Self {
        let mut data = Vec::new();
        let mut depth = 0usize;
        let mut width = 0usize;
        while let Some(line) = lines.next()
            && !line.is_empty()
        {
            for char in line.chars() {
                data.push(char == '#')
            }
            depth += 1;
            width = line.len();
        }
        Self {
            shape: Array2::from_shape_vec((depth, width), data).unwrap(),
        }
    }
    pub fn get_gift_area(&self) -> usize {
        self.shape.iter().filter(|x| **x).count()
    }
}

#[derive(Debug)]
struct Tree {
    shape: (usize, usize),
    counts: Vec<(Rc<Gift>, usize)>,
}

impl Tree {
    pub fn new(bank: &str, gifts: &[Rc<Gift>]) -> Self {
        let mut parts = bank.split(':');
        let mut shape = parts.next().unwrap().split('x');
        let height = shape.next().unwrap().parse::<usize>().unwrap();
        let width = shape.next().unwrap().parse::<usize>().unwrap();
        let shape = (height, width);

        let counts = parts.next().unwrap().trim().split(' ');
        let mut sized_gifts = Vec::new();
        for (gift, count) in gifts.iter().zip(counts.into_iter()) {
            sized_gifts.push((gift.clone(), count.parse::<usize>().unwrap()))
        }

        Self {
            shape,
            counts: sized_gifts,
        }
    }

    pub fn get_simple_fits(&self) -> bool {
        let tree_area = self.shape.0 * self.shape.1;
        let mut gift_area = 0usize;
        for (gift, count) in self.counts.iter() {
            gift_area += gift.get_gift_area() * count;
        }
        gift_area < tree_area
    }
}
