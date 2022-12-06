use super::Solver;

pub struct Day6(String);

const INPUT: &str = include_str!("../../input/day6");

impl Day6 {
    fn find_distinct_char_sequence(&self, num_chars: u32) -> u32 {
        self.0
            .as_str()
            .chars()
            .collect::<Vec<char>>()
            .as_slice()
            .windows(num_chars as usize)
            .enumerate()
            .find(|(_, window)| {
                window
                    .iter()
                    .enumerate()
                    .all(|(index, ch)| {
                        !window[index + 1 ..].contains(ch)
                    })
            })
            .unwrap()
            .0 as u32 + num_chars
    }
}

impl Solver for Day6 {
    type Solution1 = u32;
    type Solution2 = u32;

    fn new() -> Self {
        Day6(String::new())
    }

    fn reset(&mut self) {
        self.0.clear();
    }

    fn parse_input(&mut self) {
        self.0 = String::from(INPUT.lines().nth(0).unwrap());
    }

    fn solve_part1(&self) -> u32 {
        self.find_distinct_char_sequence(4)
    }

    fn solve_part2(&self) -> u32 {
        self.find_distinct_char_sequence(14)
    }

    fn print_solutions(&self, part1: u32, part2: u32) {
        println!("Number of characters until start-of-packet: {part1}");
        println!("Number of characters until start-of-message: {part2}");
    }
}
