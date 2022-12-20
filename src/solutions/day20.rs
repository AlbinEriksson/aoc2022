use std::collections::VecDeque;

use super::Solver;

pub struct Day20 {
    numbers: Vec<i64>
}

fn mix(numbers: &Vec<i64>, num_mixes: usize) -> i64 {
    let mut mix: VecDeque<(usize, i64)> = numbers.iter().map(|v| *v).enumerate().collect();
    for _ in 0..num_mixes {
        for i in 0..mix.len() {
            // Cycle numbers around until the ith number is at index 0
            while mix[0].0 != i {
                let number = mix.pop_front().unwrap();
                mix.push_back(number);
            }
            let number = mix.pop_front().unwrap();
            let move_by = number.1.rem_euclid(mix.len() as i64);
            // Cycle numbers around until the 0th position is where we need to insert the number
            for _ in 0..move_by {
                let number = mix.pop_front().unwrap();
                mix.push_back(number);
            }
            mix.push_back(number)
        }
    }
    let zero = mix.iter().enumerate().find(|(_, num)| num.1 == 0).unwrap().0;
    mix[(zero + 1000) % mix.len()].1 as i64 +
    mix[(zero + 2000) % mix.len()].1 as i64 +
    mix[(zero + 3000) % mix.len()].1 as i64
}

const INPUT: &str = include_str!("../../input/day20");

impl Solver for Day20 {
    type Solution1 = i64;
    type Solution2 = i64;

    fn new() -> Self {
        Day20 { numbers: vec!() }
    }

    fn reset(&mut self) {
        self.numbers.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            self.numbers.push(line.parse().unwrap());
        }
    }

    fn solve_part1(&self) -> i64 {  
        mix(&self.numbers, 1)
    }

    fn solve_part2(&self) -> i64 {
        let numbers: Vec<i64> = self.numbers.iter().map(|v| v * 811589153).collect();
        mix(&numbers, 10)
    }

    fn print_solutions(&self, part1: i64, part2: i64) {
        println!("Sum of three 1000th numbers after 0: {part1}");
        println!("Sum of three 1000th decrypted numbers after 0: {part2}");
    }
}
