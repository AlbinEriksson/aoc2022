use std::collections::HashSet;

use crate::util::pos::Pos2d;

use super::Solver;

type Pos = Pos2d<i16>;

#[derive(Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East
}

impl Dir {
    fn offset(&self, by: usize) -> Self {
        (<Self as Into<usize>>::into(*self) + by).into()
    }

    fn movement(&self) -> Pos {
        match self {
            Self::North => Pos::new(0, -1),
            Self::South => Pos::new(0, 1),
            Self::West => Pos::new(-1, 0),
            Self::East => Pos::new(1, 0)
        }
    }

    fn adjacents(&self) -> [Pos; 3] {
        match self {
            Self::North => [Pos::new(-1, -1), Pos::new(0, -1), Pos::new(1, -1)],
            Self::South => [Pos::new(-1, 1), Pos::new(0, 1), Pos::new(1, 1)],
            Self::West => [Pos::new(-1, -1), Pos::new(-1, 0), Pos::new(-1, 1)],
            Self::East => [Pos::new(1, -1), Pos::new(1, 0), Pos::new(1, 1)]
        }
    }
}

impl Into<usize> for Dir {
    fn into(self) -> usize {
        match self {
            Self::North => 0,
            Self::South => 1,
            Self::West => 2,
            Self::East => 3
        }
    }
}

impl From<usize> for Dir {
    fn from(value: usize) -> Self {
        match value % 4 {
            0 => Self::North,
            1 => Self::South,
            2 => Self::West,
            3 => Self::East,
            _ => unreachable!()
        }
    }
}

pub struct Day23 {
    elves: Vec<Pos>
}

fn simulate(elves: &mut HashSet<Pos>, limit: Option<usize>) -> usize {
    let limit = limit.unwrap_or(usize::MAX);
    let mut round = 0;
    while round < limit {
        let mut new_elves: HashSet<Pos> = HashSet::new();
        let mut all_happy = true;
        'elves: for elf in elves.iter() {
            let mut happy = true;
            for neighbor in [
                Pos::new(-1, -1), Pos::new(0, -1), Pos::new(1, -1),
                Pos::new(-1, 0), Pos::new(1, 0),
                Pos::new(-1, 1), Pos::new(0, 1), Pos::new(1, 1)]
            {
                let neighbor = *elf + neighbor;
                if elves.contains(&neighbor) {
                    happy = false;
                    all_happy = happy;
                    break;
                }
            }
            if happy {
                new_elves.insert(*elf);
                continue;
            }
            'dirs: for dir in [Dir::North, Dir::South, Dir::West, Dir::East] {
                let dir = dir.offset(round);
                for adjacent in dir.adjacents() {
                    let adjacent = *elf + adjacent;
                    if elves.contains(&adjacent) {
                        continue 'dirs;
                    }
                }
                let new_pos = *elf + dir.movement();
                if new_elves.contains(&new_pos) {
                    new_elves.remove(&new_pos);
                    new_elves.insert(*elf);
                    new_elves.insert(new_pos + dir.movement());
                } else {
                    new_elves.insert(new_pos);
                }
                continue 'elves;
            }
            new_elves.insert(*elf);
        }
        *elves = new_elves;
        round += 1;
        if all_happy {
            break;
        }
    }
    round
}

const INPUT: &str = include_str!("../../input/day23");

impl Solver for Day23 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day23 { elves: vec!() }
    }

    fn reset(&mut self) {
        self.elves.clear();
    }

    fn parse_input(&mut self) {
        for (y, line) in INPUT.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    self.elves.push(Pos::new(x as i16, y as i16));
                }
            }
        }
    }

    fn solve_part1(&self) -> usize {
        let mut elves: HashSet<Pos> = self.elves.iter().map(|v| *v).collect();
        simulate(&mut elves, Some(10));

        let (min, max) = elves.iter()
            .fold((Pos::new(i16::MAX, i16::MAX), Pos::new(i16::MIN, i16::MIN)), |(min, max), elf| {
                (
                    Pos::new(min.x.min(elf.x), min.y.min(elf.y)),
                    Pos::new(max.x.max(elf.x + 1), max.y.max(elf.y + 1))
                )
            });
        let size = max - min;
        size.x as usize * size.y as usize - elves.len()
    }

    fn solve_part2(&self) -> usize {
        let mut elves: HashSet<Pos> = self.elves.iter().map(|v| *v).collect();
        simulate(&mut elves, None)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Number of empty tiles after 10 rounds: {part1}");
        println!("Number of rounds until the elves stop: {part2}");
    }
}
