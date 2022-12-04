use super::Solver;

struct Range {
    min: u32,
    max: u32
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.min <= other.max && self.max >= other.min
    }
}

struct Pair {
    first: Range,
    second: Range
}

pub struct Day4 {
    pairs: Vec<Pair>
}

const INPUT: &str = include_str!("../../input/day4");

impl Solver for Day4 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day4 { pairs: vec!() }
    }

    fn reset(&mut self) {
        self.pairs.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let (first_min, line) = line.split_once('-').unwrap();
            let (first_max, line) = line.split_once(',').unwrap();
            let (second_min, line) = line.split_once('-').unwrap();
            let second_max = line;

            self.pairs.push(Pair {
                first: Range {
                    min: first_min.parse().unwrap(),
                    max: first_max.parse().unwrap()
                },
                second: Range {
                    min: second_min.parse().unwrap(),
                    max: second_max.parse().unwrap()
                }
            })
        }
    }

    fn solve_part1(&self) -> u32 {
        self.pairs
            .iter()
            .fold(0, |acc, pair| {
                if pair.first.contains(&pair.second) || pair.second.contains(&pair.first) {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn solve_part2(&self) -> u32 {
        self.pairs
            .iter()
            .fold(0, |acc, pair| {
                if pair.first.overlaps(&pair.second) || pair.second.overlaps(&pair.first) {
                    acc + 1
                } else {
                    acc
                }
            })
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Number of contained assignment ranges: {part1}");
        println!("Number of overlapped assignment ranges: {part2}");
    }
}
