use std::{collections::HashMap, fmt::Display};

use crate::util::pos::Pos2d;

use super::Solver;

type Pos = Pos2d<u8>;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Open,
    Wall
}

enum Step {
    Walk(u8),
    TurnCw,
    TurnCcw
}

#[derive(Clone, Copy)]
enum Facing {
    Right,
    Down,
    Left,
    Up
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3
        }
    }

    fn step(&self, pos: &mut Pos, dist: u8) {
        match self {
            Self::Right => pos.x += dist,
            Self::Down => pos.y += dist,
            Self::Left => pos.x -= dist,
            Self::Up => pos.y -= dist,
        }
    }

    fn opposite(&self) -> Facing {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down
        }
    }

    fn turn_cw(&self) -> Facing {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right
        }
    }

    fn turn_ccw(&self) -> Facing {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left
        }
    }

    fn diff(&self, other: &Facing) -> i8 {
        self.value() as i8 - other.value() as i8
    }

    fn turn(&self, by: i8) -> Facing {
        match by.rem_euclid(4) {
            0 => *self,
            1 => self.turn_cw(),
            2 => self.opposite(),
            3 => self.turn_ccw(),
            _ => unreachable!()
        }
    }
}

impl Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "right"),
            Self::Down => write!(f, "down"),
            Self::Left => write!(f, "left"),
            Self::Up => write!(f, "up")
        }
    }
}

#[derive(Clone, Copy)]
struct Edge {
    to: u8,
    side: Facing
}

#[derive(Default, Clone, Copy)]
struct Face {
    chunk: Pos,
    edges: [Option<Edge>; 4]
}

pub struct Day22 {
    tiles: HashMap<Pos, Tile>,
    steps: Vec<Step>
}

impl Day22 {
    fn find_start_pos(&self) -> Pos {
        let x = self.tiles.iter()
            .filter_map(|(pos, tile)|
                if pos.y == 1 && *tile == Tile::Open { Some(pos.x) } else { None })
            .min()
            .unwrap();
        Pos::new(x, 1)
    }

    fn grid_size(&self) -> Pos {
        self.tiles.keys().fold(Pos::new(0, 0), |acc, item|
            Pos::new(acc.x.max(item.x), acc.y.max(item.y))
        )
    }
}

const INPUT: &str = include_str!("../../input/day22");

impl Solver for Day22 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day22 { tiles: HashMap::new(), steps: vec!() }
    }

    fn reset(&mut self) {
        self.tiles.clear();
        self.steps.clear();
    }

    fn parse_input(&mut self) {
        let mut lines = INPUT.lines();

        let mut y = 0;
        loop {
            y += 1;
            let line = lines.next().unwrap();
            if line.is_empty() {
                break;
            }

            for (x, ch) in line.chars().enumerate() {
                let x = x + 1;
                match ch {
                    '.' => self.tiles.insert(Pos::new(x.try_into().unwrap(), y), Tile::Open),
                    '#' => self.tiles.insert(Pos::new(x.try_into().unwrap(), y), Tile::Wall),
                    ' ' => continue,
                    _ => panic!()
                };
            }
        }

        fn add_walk(steps: &mut Vec<Step>, walk_dist: &mut u8) {
            if *walk_dist > 0 {
                steps.push(Step::Walk(*walk_dist));
                *walk_dist = 0;
            }
        }

        let line = lines.next().unwrap();
        let mut walk_dist = 0;
        for ch in line.chars() {
            match ch {
                '0'..='9' => walk_dist = walk_dist * 10 + (ch as u8 - '0' as u8),
                'R' => {
                    add_walk(&mut self.steps, &mut walk_dist);
                    self.steps.push(Step::TurnCw);
                },
                'L' => {add_walk(&mut self.steps, &mut walk_dist);
                    self.steps.push(Step::TurnCcw);
                },
                _ => panic!()
            }
        }
        add_walk(&mut self.steps, &mut walk_dist);
    }

    fn solve_part1(&self) -> usize {
        let mut pos = self.find_start_pos();
        let mut facing = Facing::Right;
        for step in &self.steps {
            match step {
                Step::Walk(dist) => {
                    'walk: for _ in 0..*dist {
                        let mut new_pos = pos;
                        facing.step(&mut new_pos, 1);
                        match self.tiles.get(&new_pos) {
                            Some(Tile::Open) => pos = new_pos,
                            Some(Tile::Wall) => break 'walk,
                            None => {
                                let mut wrap_pos = new_pos;
                                let backwards = facing.opposite();
                                let mut wrap_tile = Tile::Open;
                                loop {
                                    backwards.step(&mut wrap_pos, 1);
                                    match self.tiles.get(&wrap_pos) {
                                        Some(tile) => wrap_tile = *tile,
                                        None => {
                                            if wrap_tile == Tile::Wall {
                                                break 'walk;
                                            }
                                            facing.step(&mut wrap_pos, 1);
                                            pos = wrap_pos;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Step::TurnCw => {
                    facing = facing.turn_cw();
                },
                Step::TurnCcw => {
                    facing = facing.turn_ccw();
                }
            }
        }

        1000 * pos.y as usize + 4 * pos.x as usize + facing.value()
    }

    fn solve_part2(&self) -> usize {
        let side_length = 50;
        let size = self.grid_size() / side_length;
        let mut faces = [Face::default(); 6];
        let mut chunk_to_face: HashMap<Pos, u8> = HashMap::new();

        // Find sides of cube
        let mut index = 0;
        for y in 0..size.y {
            for x in 0..size.x {
                if self.tiles.contains_key(&Pos::new(x * side_length + 1, y * side_length + 1)) {
                    chunk_to_face.insert(Pos::new(x + 1, y + 1), index);
                    index += 1;
                }
            }
        }

        // Combine adjacent sides of cube
        for (chunk, face) in &chunk_to_face {
            let face = &mut faces[*face as usize];
            face.chunk = *chunk;
            for dir in [Facing::Right, Facing::Down, Facing::Left, Facing::Up] {
                let mut other_chunk = *chunk;
                dir.step(&mut other_chunk, 1);
                if let Some(other_face) = chunk_to_face.get(&other_chunk) {
                    face.edges[dir.value() as usize] = Some(Edge {
                        to: *other_face,
                        side: dir.opposite()
                    });
                }
            }
        }

        // Glue non-adjacent sides of cube
        loop {
            let mut found = false;
            for face in 0..6 {
                let face = faces[face as usize];
                for dir_a in [Facing::Right, Facing::Down, Facing::Left, Facing::Up] {
                    let dir_b = dir_a.turn_cw();
                    if face.edges[dir_a.value()].is_none() || face.edges[dir_b.value()].is_none() {
                        continue;
                    }
                    let edge_a = face.edges[dir_a.value()].as_ref().unwrap();
                    let edge_b = face.edges[dir_b.value()].as_ref().unwrap();
                    let side_a = dir_b.turn(-dir_a.opposite().diff(&edge_a.side));
                    let side_b = dir_a.turn(-dir_b.opposite().diff(&edge_b.side));

                    if faces[edge_a.to as usize].edges[side_a.value()].is_some() ||
                        faces[edge_b.to as usize].edges[side_b.value()].is_some()
                    {
                        continue;
                    }

                    let face_a = &mut faces[edge_a.to as usize];
                    face_a.edges[side_a.value()] = Some(Edge { to: edge_b.to, side: side_b });
                    let face_b = &mut faces[edge_b.to as usize];
                    face_b.edges[side_b.value()] = Some(Edge { to: edge_a.to, side: side_a });
                    found = true;
                }
            }
            if !found {
                break;
            }
        }

        // Walk on cube
        let mut pos = self.find_start_pos();
        let mut facing = Facing::Right;
        for step in &self.steps {
            match step {
                Step::Walk(dist) => {
                    for _ in 0..*dist {
                        let mut new_pos = pos;
                        let mut new_facing = facing;
                        facing.step(&mut new_pos, 1);

                        let old_chunk = (pos - Pos::new(1, 1)) / side_length + Pos::new(1, 1);
                        if new_pos.x == 0 || new_pos.y == 0 ||
                            old_chunk != (new_pos - Pos::new(1, 1)) / side_length + Pos::new(1, 1)
                        {
                            let face = chunk_to_face[&old_chunk];
                            let face = &faces[face as usize];
                            let edge = &face.edges[facing.value()].unwrap();
                            let rotation = facing.opposite().diff(&edge.side);
                            new_facing = facing.turn(-rotation);
                            let relative_pos = pos - (old_chunk - Pos::new(1, 1)) * side_length;
                            let relative_pos = match (facing, rotation.rem_euclid(4)) {
                                (Facing::Left, 1) | (Facing::Right, 1) |
                                (Facing::Up, 3) | (Facing::Down, 3) =>
                                    Pos::new(relative_pos.y, relative_pos.x),
                                (Facing::Left, 0) | (Facing::Right, 0) |
                                (Facing::Up, 2) | (Facing::Down, 2) =>
                                    Pos::new(side_length - relative_pos.x + 1, relative_pos.y),
                                (Facing::Up, 0) | (Facing::Down, 0) |
                                (Facing::Left, 2) | (Facing::Right, 2) =>
                                    Pos::new(relative_pos.x, side_length - relative_pos.y + 1),
                                (Facing::Up, 1) | (Facing::Down, 1) |
                                (Facing::Left, 3) | (Facing::Right, 3) =>
                                    Pos::new(side_length - relative_pos.y + 1, side_length - relative_pos.x + 1),
                                _ => unreachable!()
                            };
                            new_pos = (faces[edge.to as usize].chunk - Pos::new(1, 1)) * side_length + relative_pos;
                        }

                        match self.tiles.get(&new_pos) {
                            Some(Tile::Open) => {
                                pos = new_pos;
                                facing = new_facing;
                            },
                            Some(Tile::Wall) => break,
                            None => unreachable!()
                        }
                    }
                },
                Step::TurnCw => {
                    facing = facing.turn_cw();
                },
                Step::TurnCcw => {
                    facing = facing.turn_ccw();
                }
            }
        }
        
        1000 * pos.y as usize + 4 * pos.x as usize + facing.value()
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Flat password: {part1}");
        println!("Cube password: {part2}");
    }
}
