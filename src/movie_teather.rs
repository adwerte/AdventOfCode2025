use itertools::Itertools;
use std::fs;

pub fn main() {
    let bank = fs::read_to_string("./input/9_movie_teather.txt").unwrap();
    let bank = bank.trim();
    let splits = bank.split('\n');
    let tiles: Vec<[isize; 2]> = splits
        .map(|x| {
            let mut numbers = x.split(',');
            [
                numbers.next().unwrap().parse::<isize>().unwrap(),
                numbers.next().unwrap().parse::<isize>().unwrap(),
            ]
        })
        .collect();
    let combinations: Vec<(&[isize; 2], &[isize; 2])> = tiles
        .iter()
        .tuple_combinations::<(&[isize; 2], &[isize; 2])>()
        .collect();
    let combinations = combinations.iter().map(|x| {
        let width = (x.1[0] - x.0[0]).unsigned_abs() + 1;
        let height = (x.1[1] - x.0[1]).unsigned_abs() + 1;
        (x.0, x.1, width * height)
    });
    let sorted_combos = combinations.sorted_by_key(|x| x.2);

    let mut area = 0usize;
    for (i, box_points) in sorted_combos.rev().enumerate() {
        if i == 0 {
            println!("Day 9: Biggest Tile Combo = {}", box_points.2);
            // 4786902990
        }
        //println!("{:?}-{:?}", box_points.0, box_points.1);
        let abox = Rectangle::new(box_points.0, box_points.1);
        let mut contains = false;
        for (i, point) in tiles.iter().enumerate() {
            let point1 = if i == 0 {
                tiles.last().unwrap()
            } else {
                &tiles[i - 1]
            };
            contains = abox.line_crosses(point1, point);
            if contains {
                break;
            }
        }
        if !contains {
            area = box_points.2;
            break;
        }
    }

    println!("Day 9: Max Tile within Polygon size {}", area)
    // 4644339888
    // 4595816092
    // 1571016172
}

struct Rectangle<'a> {
    top_point: &'a [isize; 2],
    bot_point: &'a [isize; 2],
    left_point: &'a [isize; 2],
    right_point: &'a [isize; 2],
}

impl<'a> Rectangle<'a> {
    pub fn new(point_1: &'a [isize; 2], point_2: &'a [isize; 2]) -> Self {
        let (top_point, bot_point) = if point_1[1] < point_2[1] {
            (point_2, point_1)
        } else {
            (point_1, point_2)
        };
        let (left_point, right_point) = if point_1[0] < point_2[0] {
            (point_1, point_2)
        } else {
            (point_2, point_1)
        };
        Self {
            top_point,
            bot_point,
            left_point,
            right_point,
        }
    }

    pub fn point_in(&self, point: &[isize; 2]) -> bool {
        let y_between = point[1] >= self.bot_point[1] && point[1] <= self.top_point[1];
        let x_between = point[0] >= self.left_point[0] && point[0] <= self.right_point[0];
        //println!("x: {:?}-{}-{:?}={}", self.bot_point[0], point[0], self.top_point[0], x_between);
        //println!("y: {:?}-{}-{:?}={}", self.bot_point[1], point[1], self.top_point[1], y_between);
        y_between && x_between
    }
    pub fn strictly_point_in(&self, point: &[isize; 2]) -> bool {
        let y_between = point[1] > self.bot_point[1] && point[1] < self.top_point[1];
        let x_between = point[0] > self.left_point[0] && point[0] < self.right_point[0];
        //println!("x: {:?}-{}-{:?}={}", self.bot_point[0], point[0], self.top_point[0], x_between);
        //println!("y: {:?}-{}-{:?}={}", self.bot_point[1], point[1], self.top_point[1], y_between);
        y_between && x_between
    }

    pub fn l_in(&self, point1: &[isize; 2], point2: &[isize; 2], point3: &[isize; 2]) -> bool {
        let bool_1 = self.point_in(point1);
        let bool_2 = self.point_in(point2);
        let bool_3 = self.point_in(point3);
        let self_part_1 = [point1, point2, point3].contains(&self.top_point);
        let self_part_2 = [point1, point2, point3].contains(&self.bot_point);
        let l_complete_in = bool_1 && bool_2 && bool_3;
        let l_part_off = self_part_1 && self_part_2;
        l_complete_in && !l_part_off
    }

    pub fn line_crosses(&self, point_1: &[isize; 2], point_2: &[isize; 2]) -> bool {
        let x_between = point_1[0] > self.left_point[0] && point_1[0] < self.right_point[0];
        let y_between = point_1[1] > self.bot_point[1] && point_1[1] < self.top_point[1];
        if point_1[0] == point_2[0] && x_between {
            let points = [point_1, point_2];
            let mut ordered_points = points.iter().sorted_by_key(|x| x[1]);
            let first = ordered_points.next().unwrap();
            let second = ordered_points.next().unwrap();
            let s_1 = first[1] <= self.bot_point[1];
            let s_2 = second[1] >= self.top_point[1];

            let first = self.bot_point[1] < first[1] && first[1] < self.top_point[1];
            let second = self.bot_point[1] < second[1] && second[1] < self.top_point[1];

            (s_1 && s_2) || first || second
        } else if point_1[1] == point_2[1] && y_between {
            let points = [point_1, point_2];
            let mut ordered_points = points.iter().sorted_by_key(|x| x[0]);
            let first = ordered_points.next().unwrap();
            let second = ordered_points.next().unwrap();
            let s_1 = first[0] <= self.left_point[0];
            let s_2 = second[0] >= self.right_point[0];
            //println!("{}-{}&&{}-{}", self.left_point[0], first[0], second[0], self.right_point[0]);

            let first = self.left_point[0] < first[0] && first[0] < self.right_point[0];
            let second = self.left_point[0] < second[0] && second[0] < self.right_point[0];

            (s_1 && s_2) || first || second
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let bank = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";
        let bank = bank.trim();
        let splits = bank.split('\n');
        let tiles: Vec<[isize; 2]> = splits
            .map(|x| {
                let mut numbers = x.split(',');
                [
                    numbers.next().unwrap().parse::<isize>().unwrap(),
                    numbers.next().unwrap().parse::<isize>().unwrap(),
                ]
            })
            .collect();
        let combinations: Vec<(&[isize; 2], &[isize; 2])> = tiles
            .iter()
            .tuple_combinations::<(&[isize; 2], &[isize; 2])>()
            .collect();
        let combinations = combinations.iter().map(|x| {
            let width = (x.1[0] - x.0[0]).unsigned_abs() + 1;
            let height = (x.1[1] - x.0[1]).unsigned_abs() + 1;
            (x.0, x.1, width * height)
        });
        let sorted_combos = combinations.sorted_by_key(|x| x.2);

        let mut area = 0usize;
        for (i, box_points) in sorted_combos.rev().enumerate() {
            if i == 0 {
                assert_eq!(box_points.2, 50);
            }
            println!("[{:?},{:?}]", box_points.0, box_points.1);
            let abox = Rectangle::new(box_points.0, box_points.1);
            let mut contains = false;
            for (i, point) in tiles.iter().enumerate() {
                let point1 = if i == 0 {
                    tiles.last().unwrap()
                } else {
                    &tiles[i - 1]
                };
                contains = abox.line_crosses(point1, point);
                println!("{:?}-{:?} = {}", point1, point, contains);
                if contains {
                    break;
                }
            }
            if !contains {
                area = box_points.2;
                break;
            }
        }

        assert_eq!(area, 24)
    }
}
