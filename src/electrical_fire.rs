use ndarray::Array2;
use std::fs;

pub fn main() {
    let bank = fs::read_to_string("./input/8_electrical_fire.txt").unwrap();
    let boxes = JunctionBox::news(&bank);
    let mut switch = JunctionNetwork::new(boxes.iter().collect());

    for _ in 0..1000 {
        switch.iter(&boxes);
    }

    let mut networks = switch.networks.clone();
    networks.sort_by_key(|b| b.len());
    networks.reverse();

    let mut product: usize = 1;
    for network in networks.iter().take(3) {
        product *= network.len();
    }
    println!("Day 8: Network = {}", product);

    while networks.len() > 1 {
        switch.iter(&boxes);
        networks = switch.networks.clone();
    }

    let boxes = switch.latest.unwrap();
    let total = boxes[0].coordinate[0] * boxes[1].coordinate[0];

    println!("Day 8: Last connection = {}", total);
}

#[derive(Debug, PartialEq)]
struct JunctionBox {
    coordinate: [isize; 3],
}

impl JunctionBox {
    fn new(row: &str) -> Self {
        let splits = row.split(',');
        let mut coordinate: [isize; 3] = [0; 3];
        for (index, split) in splits.enumerate() {
            coordinate[index] = split.parse().unwrap();
        }

        Self { coordinate }
    }

    pub fn news(bank: &str) -> Vec<Self> {
        let bank = bank.trim();
        let rows = bank.split('\n');
        rows.map(Self::new).collect()
    }
}

struct JunctionNetwork<'a> {
    networks: Vec<Vec<&'a JunctionBox>>,
    distances: Array2<f64>,
    latest: Option<[&'a JunctionBox; 2]>,
}

impl<'a> JunctionNetwork<'a> {
    pub fn new(boxes: Vec<&'a JunctionBox>) -> Self {
        let mut distances = Array2::zeros((boxes.len(), boxes.len()));
        for (j, box_j) in boxes.iter().enumerate() {
            for (i, box_i) in boxes.iter().enumerate() {
                if i == j {
                    distances[[i, j]] = f64::INFINITY;
                } else {
                    distances[[i, j]] = ((box_j.coordinate[0] - box_i.coordinate[0]).pow(2)
                        + (box_j.coordinate[1] - box_i.coordinate[1]).pow(2)
                        + (box_j.coordinate[2] - box_i.coordinate[2]).pow(2))
                        as f64;
                    distances[[i, j]] = distances[[i, j]].sqrt();
                }
            }
        }
        let networks = boxes.iter().map(|x| vec![*x]).collect();

        Self {
            networks,
            distances,
            latest: None,
        }
    }

    pub fn iter(&mut self, boxes: &'a [JunctionBox]) {
        let min = self.distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let position = self.distances.iter().position(|x| x == &min).unwrap();
        let j = position / boxes.len();
        let i = position % boxes.len();
        self.distances[[i, j]] = f64::INFINITY;
        self.distances[[j, i]] = f64::INFINITY;
        let box_j = &boxes[j];
        let box_i = &boxes[i];
        let mut network_i = None;
        let mut network_j = None;
        for (network_id, network) in self.networks.iter().enumerate() {
            if network.contains(&box_i) && network.contains(&box_j) {
                network_i = None;
                network_j = None;
            } else if network.contains(&box_i) {
                network_i = Some(network_id);
            } else if network.contains(&box_j) {
                network_j = Some(network_id);
            }
        }

        if let Some(network_i) = network_i
            && let Some(network_j) = network_j
        {
            let network_old = self.networks.remove(network_j);
            let new_index = if network_j < network_i {
                network_i - 1
            } else {
                network_i
            };
            self.networks[new_index].extend(network_old.iter());
            self.latest = Some([box_i, box_j])
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_example() {
        let bank = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689\n";
        let boxes = JunctionBox::news(bank);
        let mut switch = JunctionNetwork::new(boxes.iter().collect());

        for _ in 0..10 {
            switch.iter(&boxes);
        }

        let mut networks = switch.networks.clone();
        networks.sort_by_key(|b| b.len());
        networks.reverse();

        let mut product: usize = 1;
        for network in networks.iter().take(3) {
            product *= network.len();
        }

        assert_eq!(product, 40);

        while networks.len() > 1 {
            switch.iter(&boxes);
            networks = switch.networks.clone();
        }

        let boxes = switch.latest.unwrap();
        let total = boxes[0].coordinate[0] * boxes[1].coordinate[0];
        assert_eq!(total, 25272)
    }
}
