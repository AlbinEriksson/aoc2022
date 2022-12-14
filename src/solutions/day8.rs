use crate::util::intset::IntSet;

use super::Solver;

pub struct Day8 {
    trees: Vec<Vec<i8>>,
    rows: u8,
    cols: u8
}

impl Day8 {

}

const INPUT: &str = include_str!("../../input/day8");

impl Solver for Day8 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day8 { trees: vec!(), rows: 0, cols: 0 }
    }

    fn reset(&mut self) {
        self.trees.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let mut row = vec![0i8; line.len()];
            for (index, ch) in line.chars().enumerate() {
                row[index] = ch as i8 - '0' as i8;
            }
            self.trees.push(row);
        }
        self.rows = self.trees.len() as u8;
        self.cols = self.trees[0].len() as u8;
    }

    fn solve_part1(&self) -> usize {
        let mut visible_trees = IntSet::new(0, self.rows as isize * self.cols as isize);

        fn check_visibility(day8: &Day8, cols: u8, row: u8, col: u8, index: u8, max_heights: &mut Vec<i8>, visible_trees: &mut IntSet) {
            let height = day8.trees[row as usize][col as usize];
            if height > max_heights[index as usize] {
                max_heights[index as usize] = height;
                visible_trees.add(row as isize * cols as isize + col as isize);
            }
        }

        fn check_visible_columns(day8: &Day8, cols: u8, row: u8, max_heights: &mut Vec<i8>, visible_trees: &mut IntSet) {
            for col in 0..day8.cols {
                check_visibility(day8, cols, row, col, col, max_heights, visible_trees);
            }
        }

        fn check_visible_rows(day8: &Day8, cols: u8, col: u8, max_heights: &mut Vec<i8>, visible_trees: &mut IntSet) {
            for row in 0..day8.rows {
                check_visibility(day8, cols, row, col, row, max_heights, visible_trees);
            }
        }

        // Top to bottom
        let mut max_heights = vec![-1i8; self.cols as usize];
        for row in 0..self.rows {
            check_visible_columns(self, self.cols, row, &mut max_heights, &mut visible_trees);
        }

        // Bottom to top
        max_heights.fill(-1);
        for row in (0..self.rows).rev() {
            check_visible_columns(self, self.cols, row, &mut max_heights, &mut visible_trees);
        }

        // Left to right
        max_heights.fill(-1);
        for col in 0..self.cols {
            check_visible_rows(self, self.cols, col, &mut max_heights, &mut visible_trees);
        }

        // Right to left
        max_heights.fill(-1);
        for col in (0..self.cols).rev() {
            check_visible_rows(self, self.cols, col, &mut max_heights, &mut visible_trees);
        }
        
        visible_trees.count()
    }

    fn solve_part2(&self) -> usize {
        let tree = self.trees.iter().enumerate().skip(1).take(self.rows as usize - 2).map(|(row, trees)| {
            trees.iter().enumerate().skip(1).take(self.cols as usize - 2).map(move |(col, tree)| {
                let left = self.trees[row].iter().take(col).rev().enumerate().find_map(|(left_col, left_tree)| {
                    if left_tree >= tree {
                        Some(left_col + 1)
                    } else {
                        None
                    }
                }).unwrap_or(col);
                let right = self.trees[row].iter().skip(col + 1).enumerate().find_map(|(right_col, right_tree)| {
                    if right_tree >= tree {
                        Some(right_col + 1)
                    } else {
                        None
                    }
                }).unwrap_or(self.cols as usize - col - 1);
                let up = self.trees.iter().take(row).rev().map(|up_trees| {
                    &up_trees[col]
                }).enumerate().find_map(|(up_row, up_tree)| {
                    if up_tree >= tree {
                        Some(up_row + 1)
                    } else {
                        None
                    }
                }).unwrap_or(row);
                let down = self.trees.iter().skip(row + 1).map(|down_trees| {
                    &down_trees[col]
                }).enumerate().find_map(|(down_row, down_tree)| {
                    if down_tree >= tree {
                        Some(down_row + 1)
                    } else {
                        None
                    }
                }).unwrap_or(self.rows as usize - row - 1);
                (left * right * up * down, row, col)
            }).max_by_key(|tree| tree.0).unwrap_or((0, 0, 0))
        }).max_by_key(|tree| tree.0).unwrap_or((0, 0, 0));
        tree.0
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Number of visible trees: {part1}");
        println!("Highest scenic score: {part2}");
    }
}
