use core::panic;
use std::iter;

use crate::util::interleave::InterleaveIter;

use super::Solver;

enum Instr {
    AddX(i8),
    NoOp
}

impl Instr {
    fn num_cycles(&self) -> usize {
        match self {
            Instr::AddX(_) => 2,
            Instr::NoOp => 1
        }
    }
}

pub struct Day10 {
    program: Vec<Instr>,
    crt_width: usize,
    crt_height: usize,
}

impl Day10 {
    fn get_runtime(&self) -> impl Iterator<Item = (usize, isize)> + '_ {
        iter::once(&Instr::NoOp).cycle().take(1)
            .chain(self.program.iter())
            .flat_map(|instr| {
                iter::once(instr).cycle().take(instr.num_cycles()).enumerate()
            })
            .scan(1, |x_reg, (step, instr)| {
                match (step, instr) {
                    (0, Instr::AddX(_)) => (),
                    (1, Instr::AddX(imm)) => *x_reg += *imm as isize,
                    (0, Instr::NoOp) => (),
                    (_, _) => panic!()
                }
                Some(*x_reg)
            })
            .enumerate()
    }
}

const INPUT: &str = include_str!("../../input/day10");

impl Solver for Day10 {
    type Solution1 = isize;
    type Solution2 = String;

    fn new() -> Self {
        Day10 { program: vec!(), crt_width: 0, crt_height: 0 }
    }

    fn reset(&mut self) {
        self.program.clear();
        self.crt_width = 0;
        self.crt_height = 0;
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let mut words = line.split(' ');
            let opcode = words.next().unwrap();
            let instr = match opcode {
                "noop" => Instr::NoOp,
                "addx" => Instr::AddX(words.next().unwrap().parse().unwrap()),
                _ => panic!("Unknown opcode {opcode}")
            };
            self.program.push(instr);
        }
        self.crt_width = 40;
        self.crt_height = 6;
    }

    fn solve_part1(&self) -> isize {
        self.get_runtime()
            .skip(19).step_by(self.crt_width).take(6)
            .map(|(cycle, x_reg)| (cycle + 1, x_reg))
            .map(|(cycle, x_reg)| x_reg * cycle as isize)
            .sum()
    }

    fn solve_part2(&self) -> String {
        self.get_runtime()
            .take(self.crt_width * self.crt_height)
            .map(|(cycle, x_reg)| (cycle % self.crt_width, x_reg))
            .map(|(col, x_reg)| if (col as isize - x_reg).abs() <= 1 { '#' } else { '.' })
            .interleave('\n', self.crt_width)
            .collect()
    }

    fn print_solutions(&self, part1: isize, part2: String) {
        println!("Sum of signal strengths: {part1}");
        println!("CRT image:\n{part2}");
    }
}
