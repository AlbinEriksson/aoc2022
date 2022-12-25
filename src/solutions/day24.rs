use std::collections::{BinaryHeap, HashMap};

use crate::util::{pos::Pos2d, number::Gcd};

use super::Solver;

type Pos = Pos2d<i8>;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    None
}

impl Dir {
    fn move_wrap(&self, pos: &Pos, min: &Pos, max: &Pos) -> Pos {
        match self {
            Dir::Up => if pos.y <= min.y {
                Pos::new(pos.x, max.y)
            } else {
                Pos::new(pos.x, pos.y - 1)
            },
            Dir::Down => if pos.y >= max.y {
                Pos::new(pos.x, min.y)
            } else {
                Pos::new(pos.x, pos.y + 1)
            },
            Dir::Left => if pos.x <= min.x {
                Pos::new(max.x, pos.y)
            } else {
                Pos::new(pos.x - 1, pos.y)
            },
            Dir::Right => if pos.x >= max.x {
                Pos::new(min.x, pos.y)
            } else {
                Pos::new(pos.x + 1, pos.y)
            },
            Dir::None => *pos
        }
    }

    fn move_once(&self, pos: &Pos) -> Pos {
        match self {
            Dir::Up => Pos::new(pos.x, pos.y - 1),
            Dir::Down => Pos::new(pos.x, pos.y + 1),
            Dir::Left => Pos::new(pos.x - 1, pos.y),
            Dir::Right => Pos::new(pos.x + 1, pos.y),
            Dir::None => *pos
        }
    }
}

impl From<char> for Dir {
    fn from(ch: char) -> Self {
        match ch {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!()
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy)]
struct State {
    pos: Pos,
    num_steps: usize,
    rem_dist: usize,
    goals_hit: usize
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.rem_dist + self.num_steps).cmp(
            &(other.rem_dist + other.num_steps)
        ).then((self.rem_dist).cmp(
            &(other.rem_dist)
        ))
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Memo {
    pos: Pos,
    time: usize,
    goals_hit: usize
}

struct BlizzardCache {
    horizontal: Vec<HashMap<Pos, Vec<Dir>>>,
    vertical: Vec<HashMap<Pos, Vec<Dir>>>
}

impl BlizzardCache {
    fn new(blizzards: &Vec<(Pos, Dir)>, min: Pos, max: Pos ) -> Self {
        let vertical: HashMap<Pos, Vec<Dir>> = blizzards
            .iter()
            .filter(|b| b.1 == Dir::Up || b.1 == Dir::Down)
            .map(|b| (b.0, vec!(b.1)))
            .collect();
        let horizontal: HashMap<Pos, Vec<Dir>> = blizzards
            .iter()
            .filter(|b| b.1 == Dir::Left || b.1 == Dir::Right)
            .map(|b| (b.0, vec!(b.1)))
            .collect();
        let mut vertical = vec!(vertical);
        let mut horizontal = vec!(horizontal);
        fn update(blizzards: &HashMap<Pos, Vec<Dir>>, min: &Pos, max: &Pos) -> HashMap<Pos, Vec<Dir>> {
            let mut new_blizzards: HashMap<Pos, Vec<Dir>> = HashMap::new();
            for (pos, dirs) in blizzards {
                for dir in dirs {
                    let new_pos = dir.move_wrap(pos, min, max);
                    if let Some(dirs) = new_blizzards.get_mut(&new_pos) {
                        dirs.push(*dir);
                    } else {
                        new_blizzards.insert(new_pos, vec!(*dir));
                    }
                }
            }
            new_blizzards
        }
        while vertical.len() < (max.y - min.y + 1).try_into().unwrap() {
            let new_blizzards = update(&vertical.last().unwrap(), &min, &max);
            vertical.push(new_blizzards);
        }
        while horizontal.len() < (max.x - min.x + 1).try_into().unwrap() {
            let new_blizzards = update(&horizontal.last().unwrap(), &min, &max);
            horizontal.push(new_blizzards);
        }
        BlizzardCache { horizontal, vertical }
    }

    fn get_vertical(&self, time: isize) -> &HashMap<Pos, Vec<Dir>> {
        &self.vertical[time.rem_euclid(self.vertical.len() as isize) as usize]
    }

    fn get_horizontal(&self, time: isize) -> &HashMap<Pos, Vec<Dir>> {
        &self.horizontal[time.rem_euclid(self.horizontal.len() as isize) as usize]
    }
}

pub struct Day24 {
    blizzards: Vec<(Pos, Dir)>,
    min: Pos,
    max: Pos,
    start: Pos,
    end: Pos
}

impl Day24 {
    fn is_valid_state(&self, pos: &Pos, time: usize, cache: &BlizzardCache) -> bool {
        let time = time as isize;
        (*pos == self.start || *pos == self.end || (
            pos.x >= self.min.x && pos.x <= self.max.x &&
            pos.y >= self.min.y && pos.y <= self.max.y
        )) && !cache.get_vertical(time).contains_key(&pos)
        && !cache.get_horizontal(time).contains_key(&pos)
    }

    fn expand(&self, state: &State, goals: &Vec<Pos>, queue: &mut BinaryHeap<State>,
        cache: &BlizzardCache, memos: &mut HashMap<Memo, usize>, cycle: usize)
    {
        for dir in [Dir::Right, Dir::Down, Dir::None, Dir::Left, Dir::Up]
        {
            let pos = dir.move_once(&state.pos);
            let num_steps = state.num_steps + 1;
            if !self.is_valid_state(&pos, num_steps, cache) {
                continue;
            }
            let goals_hit = if pos == goals[state.goals_hit] {
                state.goals_hit + 1
            } else {
                state.goals_hit
            };
            let rem_dist = if goals_hit < goals.len() {
                (pos - goals[state.goals_hit]).abs().sum::<usize>() +
                    goals.windows(2).skip(state.goals_hit).map(|g|
                        (g[0] - g[1]).abs().sum::<usize>()
                    ).sum::<usize>()
            } else {
                0
            };
            let memo = Memo {
                pos,
                // time: (state.end_time as isize - state.num_steps as isize).rem_euclid(cycle as isize) as usize
                time: (num_steps as isize).rem_euclid(cycle as isize) as usize,
                goals_hit
            };
            if state.num_steps < *memos.get(&memo).unwrap_or(&usize::MAX) {
                queue.push(State {
                    pos,
                    num_steps,
                    rem_dist,
                    goals_hit
                });
                memos.insert(memo, state.num_steps);
            }
        }
    }

    fn search(&self, goals: &Vec<Pos>, queue: &mut BinaryHeap<State>, cache: &BlizzardCache,
        memos: &mut HashMap<Memo, usize>, cycle: usize, initial_best_time: usize) -> usize
    {
        let mut best_time = initial_best_time;
        while let Some(to_expand) = queue.pop() {
            if to_expand.num_steps + to_expand.rem_dist >= best_time {
                continue;
            }
            if to_expand.rem_dist == 0 && to_expand.num_steps < best_time
            {
                best_time = to_expand.num_steps;
                continue;
            }
            self.expand(&to_expand, goals, queue, cache, memos, cycle);
        }
        best_time
    }

    fn search_greedy<F>(&self, state: State, goal: &Pos, cache: &BlizzardCache, next: F)
    -> Option<State> where
    F: Copy + Fn(&State) -> Option<State>
    {
        let dir_order = if state.pos.x < goal.x {
            [Dir::Right, Dir::Down, Dir::None, Dir::Up, Dir::Left]
        } else {
            [Dir::Left, Dir::Up, Dir::None, Dir::Down, Dir::Right]
        };

        let mut stack = vec!((state, 0));
        'search: while !stack.is_empty() {
            let (state, i) = stack.last_mut().unwrap();

            if state.pos == *goal {
                if let Some(state) = next(&state) {
                    return Some(state)
                } else {
                    stack.pop();
                    continue;
                }
            }

            while *i < 5 {
                let dir = dir_order[*i];
                *i += 1;
                let pos = dir.move_once(&state.pos);
                let num_steps = state.num_steps + 1;
                if !self.is_valid_state(&pos, num_steps, cache) {
                    continue;
                }

                stack.push((State {
                    pos,
                    num_steps,
                    rem_dist: 0,
                    goals_hit: 0
                }, 0));
                continue 'search;
            }
            stack.pop();
        }

        None
    }
}

const INPUT: &str = include_str!("../../input/day24");

impl Solver for Day24 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day24 {
            blizzards: vec!(),
            min: Pos::default(),
            max: Pos::default(),
            start: Pos::default(),
            end: Pos::default()
        }
    }

    fn reset(&mut self) {
        self.blizzards.clear();
        self.min = Pos::default();
        self.max = Pos::default();
        self.start = Pos::default();
        self.end = Pos::default();
    }

    fn parse_input(&mut self) {
        let mut lines = INPUT.lines();

        let line = lines.next().unwrap();
        let x = line.chars().enumerate().find(|(_, ch)| *ch == '.').unwrap().0;
        let max_x = line.len() - 2;
        self.start = Pos::new(x.try_into().unwrap(), 0);

        self.min = Pos::new(1, 1);
        let mut line = lines.next().unwrap();
        let mut y = 1;
        while !line.starts_with("##") {
            for (x, ch) in line.chars().enumerate().skip(1).take_while(|ch| ch.1 != '#') {
                if ch == '.' {
                    continue;
                }
                let dir: Dir = ch.into();
                self.blizzards.push((Pos::new(x as i8, y), dir));
            }
            line = lines.next().unwrap();
            y += 1;
        }

        self.max = Pos::new(max_x.try_into().unwrap(), y - 1);

        let x = line.chars().enumerate().find(|(_, ch)| *ch == '.').unwrap().0;
        self.end = Pos::new(x.try_into().unwrap(), y);
    }

    fn solve_part1(&self) -> usize {
        let cycle = self.max - self.min + Pos::new(1, 1);
        let cycle = (cycle.x as usize).lcm(cycle.y as usize);

        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        let cache = BlizzardCache::new(&self.blizzards, self.min, self.max);
        let mut memos: HashMap<Memo, usize> = HashMap::new();
        let dist = (self.end - self.start).abs().sum();
        let state = State {
            pos: self.start,
            num_steps: 0,
            rem_dist: dist,
            goals_hit: 0
        };

        let initial_best_time = self.search_greedy(state, &self.end, &cache, |state| {
            Some(*state)
        }).unwrap().num_steps;

        let goals = vec!(self.end);
        self.expand(&state, &goals, &mut queue, &cache, &mut memos, cycle);
        self.search(&goals, &mut queue, &cache, &mut memos, cycle, initial_best_time)
    }

    fn solve_part2(&self) -> usize {
        let cycle = self.max - self.min + Pos::new(1, 1);
        let cycle = (cycle.x as usize).lcm(cycle.y as usize);

        let mut queue: BinaryHeap<State> = BinaryHeap::new();
        let cache = BlizzardCache::new(&self.blizzards, self.min, self.max);
        let mut memos: HashMap<Memo, usize> = HashMap::new();
        let dist = (self.end - self.start).abs().sum::<usize>() * 3;
        let state = State {
            pos: self.start,
            num_steps: 0,
            rem_dist: dist,
            goals_hit: 0
        };

        let initial_best_time = self.search_greedy(state, &self.end, &cache, |state| {
            self.search_greedy(*state, &self.start, &cache, |state| {
                self.search_greedy(*state, &self.end, &cache, |state| {
                    Some(*state)
                })
            })
        }).unwrap().num_steps;

        let goals = vec!(self.end, self.start, self.end);
        self.expand(&state, &goals, &mut queue, &cache, &mut memos, cycle);
        self.search(&goals, &mut queue, &cache, &mut memos, cycle, initial_best_time)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Fewest steps to reach the goal: {part1}");
        println!("Fewest steps to fetch the Elf's snacks: {part2}");
    }
}
