use std::{collections::HashSet, iter};

use super::Solver;

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl From<char> for Dir {
    fn from(ch: char) -> Self {
        match ch {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!()
        }
    }
}

struct Motion {
    dir: Dir,
    dist: usize
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i16,
    y: i16
}

impl Pos {
    fn move_in_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1
        }
    }

    fn follow(&mut self, pos: &Pos) {
        match (pos.x - self.x, pos.y - self.y) {
            (x, y) if x.abs() == 2 && y.abs() == 2 => {
                self.x += x.signum();
                self.y += y.signum();
            },
            (x, y) if x.abs() < 2 && y.abs() == 2 => {
                self.x += x;
                self.y += y.signum();
            },
            (x, y) if x.abs() == 2 && y.abs() < 2 => {
                self.x += x.signum();
                self.y += y;
            }
            (x, y) if x.abs() < 2 && y.abs() < 2 => (),
            (x, y) => panic!("Invalid pos difference {x}, {y}")
        }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

pub struct Day9 {
    motions: Vec<Motion>
}

const INPUT: &str = include_str!("../../input/day9");

impl Solver for Day9 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day9 { motions: vec!() }
    }

    fn reset(&mut self) {
        self.motions.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let mut words = line.split(' ');
            let dir: char = words.next().unwrap().parse().unwrap();
            let dist: usize = words.next().unwrap().parse().unwrap();
            self.motions.push(Motion {
                dir: dir.into(),
                dist
            });
        }
    }

    fn solve_part1(&self) -> usize {
        self.motions
            .iter()
            .flat_map(|motion| {
                iter::repeat(motion.dir).take(motion.dist)
            })
            .scan(Pos::default(), |head, dir| {
                head.move_in_dir(dir);
                Some(*head)
            })
            .scan(Pos::default(), |tail, head| {
                tail.follow(&head);
                Some(*tail)
            })
            .collect::<HashSet<Pos>>()
            .len()
    }

    fn solve_part2(&self) -> usize {
        self.motions
            .iter()
            .flat_map(|motion| {
                iter::repeat(motion.dir).take(motion.dist)
            })
            .scan(Pos::default(), |head, dir| {
                head.move_in_dir(dir);
                Some(*head)
            })
            .scan([Pos::default(); 9], |knots, head| {
                knots[0].follow(&head);
                for i in 1..knots.len() {
                    knots[i].follow(&knots[i - 1].clone());
                }
                Some(*knots.last().unwrap())
            })
            .collect::<HashSet<Pos>>()
            .len()
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Number of visited tail positions: {part1}");
        println!("Number of visited 10th knot positions: {part2}");
    }
}
