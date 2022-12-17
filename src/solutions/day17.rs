use std::collections::HashMap;

use crate::util::pos::Pos2d;

use super::Solver;

#[derive(Default, PartialEq)]
enum Tile {
    #[default] Empty,
    Rock
}

type Row = [Tile; 7];

type Pos = Pos2d<i32>;

struct Rock {
    width: u8,
    height: u8,
    pieces: Vec<Pos>
}

impl Rock {
    fn check_collision(&self, pos: Pos, chamber: &Vec<Row>) -> bool {
        self.pieces
            .iter()
            .map(|piece| Pos::new(pos.x + piece.x, pos.y - piece.y))
            .any(|pos| {
                if (pos.y as usize) < chamber.len() {
                    match chamber[pos.y as usize][pos.x as usize] {
                        Tile::Empty => false,
                        Tile::Rock => true
                    }
                } else {
                    false
                }
            })
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Memo {
    peaks: [u16; 7],
    jet_index: u16,
    rock_type: u8,
}

struct MemoValue {
    rock_index: u32,
    chamber_height: u32
}

enum Jet {
    Left,
    Right
}

pub struct Day17 {
    jets: Vec<Jet>
}

impl Day17 {
    fn simulate(&self, num_rocks: usize) -> usize {
        let rocks = [
            // '-' shape
            Rock {
                width: 4,
                height: 1,
                pieces: vec![Pos::new(0, 0), Pos::new(1, 0), Pos::new(2, 0), Pos::new(3, 0)]
            },
            // '+' shape
            Rock {
                width: 3,
                height: 3,
                pieces: vec![Pos::new(1, 0), Pos::new(0, 1), Pos::new(1, 1), Pos::new(2, 1), Pos::new(1, 2)]
            },
            // backwards 'L' shape
            Rock {
                width: 3,
                height: 3,
                pieces: vec![Pos::new(2, 0), Pos::new(2, 1), Pos::new(0, 2), Pos::new(1, 2), Pos::new(2, 2)]
            },
            // '|' shape
            Rock {
                width: 1,
                height: 4,
                pieces: vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(0, 2), Pos::new(0, 3)]
            },
            // 'O' shape
            Rock {
                width: 2,
                height: 2,
                pieces: vec![Pos::new(0, 0), Pos::new(0, 1), Pos::new(1, 0), Pos::new(1, 1)]
            }
        ];

        let mut jet_index = 0;

        let mut chamber: Vec<Row> = vec!();
        let mut memos: HashMap<Memo, MemoValue> = HashMap::new();
        let mut highest_rocks = [0u32; 7];
        let mut floor = 0usize;
        let mut rock_number = 0usize;
        while rock_number < num_rocks {
            let rock_type = rock_number % rocks.len();
            let rock = &rocks[rock_type];
            let mut x = 2i32;
            let mut y = (chamber.len() + rock.height as usize + 2) as i32;

            loop {
                let new_x = match self.jets[jet_index] {
                    Jet::Left => (x - 1).max(0),
                    Jet::Right => (x + 1).min(7 - rock.width as i32),
                };
                jet_index = (jet_index + 1) % self.jets.len();
                if !rock.check_collision(Pos::new(new_x, y), &chamber) {
                    x = new_x
                }

                if (y - rock.height as i32) < 0 ||
                    rock.check_collision(Pos::new(x, y - 1), &chamber)
                {
                    if y as usize >= chamber.len() {
                        chamber.resize_with((y + 1) as usize, Default::default);
                    }
                    rock.pieces
                        .iter()
                        .map(|piece| Pos::new(x + piece.x, y - piece.y))
                        .for_each(|pos| {
                            chamber[pos.y as usize][pos.x as usize] = Tile::Rock;
                            if pos.y as u32 >= highest_rocks[pos.x as usize] {
                                highest_rocks[pos.x as usize] = pos.y as u32 + 1;
                            }
                        });
                    let mut peaks = [0u16; 7];
                    for col in 0..7 {
                        peaks[col] = (chamber.len() - highest_rocks[col] as usize).try_into().unwrap();
                    }
                    let memo = Memo {
                        peaks,
                        jet_index: jet_index as u16,
                        rock_type: rock_type as u8
                    };
                    if memos.contains_key(&memo) {
                        let memo_value = &memos[&memo];
                        let rocks_remaining = num_rocks - rock_number;
                        let loop_length = rock_number - memo_value.rock_index as usize;
                        // Subtracting 1 here avoids an unlikely edge case where the number of
                        // remaining rocks is a multiple of the loop's length. Since the loop may
                        // overlap itself, we want to add at least one more rock to the chamber, so
                        // that the overlap is "discounted," for the lack of a better term.
                        let num_loops = (rocks_remaining - 1) / loop_length;
                        rock_number += num_loops * loop_length;
                        floor = (chamber.len() - memo_value.chamber_height as usize) * num_loops;
                        memos.clear();
                    } else {
                        memos.insert(memo, MemoValue {
                            rock_index: rock_number as u32,
                            chamber_height: chamber.len() as u32
                        });
                    }
                    break;
                } else {
                    y -= 1;
                }
            }
            rock_number += 1;
        }
        floor + chamber.len()
    }
}

const INPUT: &str = include_str!("../../input/day17");

impl Solver for Day17 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day17 { jets: vec!() }
    }

    fn reset(&mut self) {
        self.jets.clear();
    }

    fn parse_input(&mut self) {
        for ch in INPUT.chars() {
            match ch {
                '<' => self.jets.push(Jet::Left),
                '>' => self.jets.push(Jet::Right),
                '\n' => break,
                _ => panic!()
            }
        }
    }

    fn solve_part1(&self) -> usize {
        self.simulate(2022)
    }

    fn solve_part2(&self) -> usize {
        self.simulate(1000000000000)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Tower height after 2022 rocks: {part1}");
        println!("Tower height after 1 trillion rocks: {part2}");
    }
}
