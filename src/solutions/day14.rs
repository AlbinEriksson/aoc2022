use std::{iter, mem::swap};

use crate::util::{intset::IntSet, pos::Pos2d, set::Set};

use super::Solver;

type Pos = Pos2d<u16>;
type Path = Vec<Pos>;

pub struct Day14 {
    paths: Vec<Path>
}

impl Day14 {
    fn get_spanning_rect(&self) -> (Pos, Pos) {
        self.paths
            .iter()
            .flatten()
            .chain(iter::once(&Pos::new(500, 0)))
            .fold((Pos::new(u16::MAX, u16::MAX), Pos::new(0, 0)), |(mut min, mut max), point| {
                min.x = min.x.min(point.x);
                min.y = min.y.min(point.y);
                max.x = max.x.max(point.x + 1);
                max.y = max.y.max(point.y + 1);
                (min, max)
            })
    }

    fn create_grid(&self, min: &Pos, max: &Pos) -> IntSet {
        let width = (max.x - min.x) as isize;
        let mut grid = IntSet::new(min.y as isize * width + min.x as isize, max.y as isize * width + max.x as isize);
        for path in &self.paths {
            for pair in path.windows(2) {
                if pair[0].x == pair[1].x {
                    let mut y0 = pair[0].y;
                    let mut y1 = pair[1].y;
                    if y1 < y0 {
                        swap(&mut y0, &mut y1);
                    }
                    for y in y0 ..= y1 {
                        grid_add(&mut grid, Pos::new(pair[0].x, y), width);
                    }
                } else {
                    let mut x0 = pair[0].x;
                    let mut x1 = pair[1].x;
                    if x1 < x0 {
                        swap(&mut x0, &mut x1);
                    }
                    for x in x0 ..= x1 {
                        grid_add(&mut grid, Pos::new(x, pair[0].y), width);
                    }
                }
            }
        }
        grid
    }

    fn simulate_sand(&self, grid: &mut IntSet, width: isize, height: isize) -> usize {
        let mut sand_count = 0;
        loop {
            let mut pos = Pos { x: 500, y: 0 };
            if grid_contains(grid, pos, width) {
                return sand_count;
            }
            loop {
                if pos.y as isize >= height {
                    return sand_count;
                } else if !grid_contains(&grid, Pos::new(pos.x, pos.y + 1), width) {
                    pos.y += 1;
                } else if !grid_contains(&grid, Pos::new(pos.x - 1, pos.y + 1), width) {
                    pos.x -= 1;
                    pos.y += 1;
                } else if !grid_contains(&grid, Pos::new(pos.x + 1, pos.y + 1), width) {
                    pos.x += 1;
                    pos.y += 1;
                } else {
                    grid_add(grid, pos, width);
                    sand_count += 1;
                    break;
                }
            }
        }
    }
}

fn grid_contains(grid: &IntSet, pos: Pos, width: isize) -> bool {
    grid.contains(pos.y as isize * width + pos.x as isize)
}

fn grid_add(grid: &mut IntSet, pos: Pos, width: isize) {
    grid.add(pos.y as isize * width + pos.x as isize);
}

const INPUT: &str = include_str!("../../input/day14");

impl Solver for Day14 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day14 { paths: vec!() }
    }

    fn reset(&mut self) {
        self.paths.clear();
    }

    fn parse_input(&mut self) {
        self.paths = INPUT
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|point| {
                        let comma = point.find(',').unwrap();
                        let x: u16 = point[..comma].parse().unwrap();
                        let y: u16 = point[comma+1..].parse().unwrap();
                        Pos { x, y }
                    })
                    .collect::<Path>()
            })
            .collect()
    }

    fn solve_part1(&self) -> usize {
        let (min, max) = self.get_spanning_rect();
        let width = (max.x - min.x) as isize;
        let height = (max.y - min.y) as isize;
        let mut grid = self.create_grid(&min, &max);
        self.simulate_sand(&mut grid, width, height)
    }

    fn solve_part2(&self) -> usize {
        let (mut min, mut max) = self.get_spanning_rect();
        max.y += 2;
        min.x = min.x.min(500 - max.y);
        max.x = max.x.max(500 + max.y + 1);
        let width = (max.x - min.x) as isize;
        let height = (max.y - min.y) as isize;

        let mut grid = self.create_grid(&min, &max);
        for x in min.x .. max.x {
            grid_add(&mut grid, Pos::new(x, max.y - 1), width);
        }

        self.simulate_sand(&mut grid, width, height)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Sand units before overflow: {part1}");
        println!("Sand units before sand source blockage: {part2}");
    }
}
