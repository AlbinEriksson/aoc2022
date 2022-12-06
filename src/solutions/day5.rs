use super::Solver;

type Stack = String;

struct Step {
    amount: u8,
    from: u8,
    to: u8
}

pub struct Day5 {
    stacks: Vec<Stack>,
    steps: Vec<Step>
}

const INPUT: &str = include_str!("../../input/day5");

impl Solver for Day5 {
    type Solution1 = String;
    type Solution2 = String;

    fn new() -> Self {
        Day5 { stacks: vec!(), steps: vec!() }
    }

    fn reset(&mut self) {
        self.stacks.clear();
        self.steps.clear();
    }

    fn parse_input(&mut self) {
        let stack_count = (INPUT.find('\n').unwrap() + 1) / 4;
        self.stacks.resize(stack_count, String::new());
        let mut lines = INPUT.lines();
        let mut stack_lines: Vec<&str> = vec!();
        while let Some(line) = lines.next() {
            if line.starts_with(" 1") {
                break;
            }
            stack_lines.push(line);
        }
        for line in stack_lines.iter().rev() {
            for (index, crate_id) in line.chars().skip(1).step_by(4).enumerate() {
                if crate_id == ' ' {
                    continue
                }
                self.stacks[index].push(crate_id);
            }
        }
        for line in lines.skip(1) {
            let mut words = line.split(' ').skip(1).step_by(2);
            let amount: u8 = words.next().unwrap().parse().unwrap();
            let from = words.next().unwrap().parse::<u8>().unwrap() - 1;
            let to = words.next().unwrap().parse::<u8>().unwrap() - 1;
            self.steps.push(Step { amount, from, to });
        }
    }

    fn solve_part1(&self) -> String {
        let mut stacks = self.stacks.clone();
        for step in &self.steps {
            let crates: Stack = {
                let from = &mut stacks[step.from as usize];
                from.drain(from.len() - step.amount as usize ..).collect()
            };
            stacks[step.to as usize].extend(crates.chars().rev());
        }
        stacks
            .iter()
            .map(|stack| stack.chars().last().unwrap())
            .collect()
    }

    fn solve_part2(&self) -> String {
        let mut stacks = self.stacks.clone();
        for step in &self.steps {
            let crates: Stack = {
                let from = &mut stacks[step.from as usize];
                from.drain(from.len() - step.amount as usize ..).collect()
            };
            stacks[step.to as usize].extend(crates.chars());
        }
        stacks
            .iter()
            .map(|stack| stack.chars().last().unwrap())
            .collect()
    }

    fn print_solutions(&self, part1: String, part2: String) {
        println!("Topmost crates when moving one at a time: {part1}");
        println!("Topmost crates when moving multiple: {part2}");
    }
}
