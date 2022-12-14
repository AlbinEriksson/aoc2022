use std::{ops::{Index, IndexMut}, collections::BinaryHeap, cmp::Reverse};

use crate::util::{intset::IntSet, pos::Pos2d};

use super::Solver;

type Cell = i8;
type Pos = Pos2d<i8>;

impl Index<(usize, usize)> for Day12 {
    type Output = Cell;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.grid.index(index.1 * self.width as usize + index.0)
    }
}

impl IndexMut<(usize, usize)> for Day12 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.grid.index_mut(index.1 * self.width as usize + index.0)
    }
}

#[derive(PartialEq, Eq)]
struct Cost {
    num_steps: usize,
    rem_dist: u8,
    pos: Pos
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Reverse(self.num_steps + self.rem_dist as usize).partial_cmp(
            &Reverse(other.num_steps + other.rem_dist as usize)
        )
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Reverse(self.num_steps + self.rem_dist as usize).cmp(
            &Reverse(other.num_steps + other.rem_dist as usize)
        )
    }
}

pub struct Day12 {
    grid: Vec<Cell>,
    width: i8,
    height: i8,
    start: Pos,
    end: Pos
}

impl Day12 {
    fn check_elevation(&self, from: &Pos, to: &Pos) -> bool {
        let from = self[(from.x as usize, from.y as usize)];
        let to = self[(to.x as usize, to.y as usize)];
        to - from <= 1
    }

    fn is_in_grid(&self, pos: &Pos) -> bool {
        pos.x >= 0 && (pos.x as isize) < (self.width as isize) &&
        pos.y >= 0 && (pos.y as isize) < (self.height as isize)
    }

    fn visit(&self, from: &Cost, visited: &mut IntSet, queue: &mut BinaryHeap<Cost>) {
        visited.add(from.pos.y as isize * self.width as isize + from.pos.x as isize);
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let pos = Pos { x: from.pos.x + dx, y: from.pos.y + dy };
            if self.is_in_grid(&pos) && self.check_elevation(&from.pos, &pos) &&
                !visited.contains(pos.y as isize * self.width as isize + pos.x as isize)
            {
                queue.push(self.get_cost(from.num_steps + 1, pos));
            }
        }
    }

    fn a_star(&self, visited: &mut IntSet, queue: &mut BinaryHeap<Cost>) -> Option<usize> {
        while let Some(to_visit) = queue.pop() {
            if to_visit.pos == self.end {
                return Some(to_visit.num_steps);
            }
            self.visit(&to_visit, visited, queue);
        }
        None
    }

    fn get_cost(&self, num_steps: usize, pos: Pos) -> Cost {
        Cost {
            num_steps,
            rem_dist: pos.x.abs_diff(self.end.x) + pos.y.abs_diff(self.end.y),
            pos: pos
        }
    }
}

const INPUT: &str = include_str!("../../input/day12");

impl Solver for Day12 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day12 {
            grid: vec!(),
            width: 0,
            height: 0,
            start: Pos { x: 0, y: 0 },
            end: Pos { x: 0, y: 0 }
        }
    }

    fn reset(&mut self) {
        self.grid.clear();
        self.width = 0;
        self.height = 0;
        self.start.x = 0;
        self.start.y = 0;
        self.end.x = 0;
        self.end.y = 0;
    }

    fn parse_input(&mut self) {
        self.width = INPUT.find('\n').unwrap() as i8;
        self.height = INPUT.chars().filter(|c| *c == '\n').count() as i8;

        self.grid = vec![0; self.width as usize * self.height as usize];
        for (y, line) in INPUT.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let height = match ch {
                    'S' => {
                        self.start.x = x as i8;
                        self.start.y = y as i8;
                        0
                    },
                    'E' => {
                        self.end.x = x as i8;
                        self.end.y = y as i8;
                        25
                    },
                    'a'..='z' => ch as i8 - 'a' as i8,
                    _ => panic!()
                };
                self[(x, y)] = height;
            }
        }
    }

    fn solve_part1(&self) -> usize {
        let mut visited = IntSet::new(0, self.width as isize * self.height as isize);
        let mut queue: BinaryHeap<Cost> = BinaryHeap::new();
        self.visit(&self.get_cost(0, self.start), &mut visited, &mut queue);
        self.a_star(&mut visited, &mut queue).unwrap()
    }

    fn solve_part2(&self) -> usize {
        let mut visited = IntSet::new(0, self.width as isize * self.height as isize);
        let mut queue: BinaryHeap<Cost> = BinaryHeap::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if self[(x as usize, y as usize)] != 0 {
                    continue
                }
                self.visit(&self.get_cost(0, Pos::new(x, y)), &mut visited, &mut queue);
            }
        }
        self.a_star(&mut visited, &mut queue).unwrap()
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Fewest steps from start to best signal: {part1}");
        println!("Fewest steps from low elevation to best signal: {part2}");
    }
}
