use core::panic;
use std::{collections::VecDeque, mem::swap};

use crate::util::array::AsArray;

use super::Solver;

enum Side {
    Left,
    Right
}

#[derive(Clone)]
enum Op {
    Const { value: isize },
    Var { id: [char; 4] },
    Add { left: Box<Op>, right: Box<Op> },
    Sub { left: Box<Op>, right: Box<Op> },
    Mul { left: Box<Op>, right: Box<Op> },
    Div { left: Box<Op>, right: Box<Op> }
}

impl Op {
    fn replace(&mut self, var: [char; 4], rep: &Op) {
        match self {
            Op::Const { value: _ } => (),
            Op::Var { id } => if *id == var {
                *self = rep.clone()
            },
            Op::Add { left, right } |
            Op::Sub { left, right } |
            Op::Mul { left, right } |
            Op::Div { left, right } => {
                left.replace(var, rep);
                right.replace(var, rep);
            }
        }
    }

    fn eval(&self) -> isize {
        match self {
            Op::Const { value } => *value,
            Op::Var { id: _ } => panic!(),
            Op::Add { left, right } => left.eval() + right.eval(),
            Op::Sub { left, right } => left.eval() - right.eval(),
            Op::Mul { left, right } => left.eval() * right.eval(),
            Op::Div { left, right } => left.eval() / right.eval()
        }
    }

    fn contains(&self, var: [char; 4]) -> bool {
        match self {
            Op::Const { value: _ } => false,
            Op::Var { id } => *id == var,
            Op::Add { left, right } |
            Op::Sub { left, right } |
            Op::Mul { left, right } |
            Op::Div { left, right } => left.contains(var) || right.contains(var)
        }
    }

    fn find_side(&self, var: [char; 4]) -> Option<Side> {
        match self {
            Op::Const { value: _ } => None,
            Op::Var { id: _ } => None,
            Op::Add { left, right } |
            Op::Sub { left, right } |
            Op::Mul { left, right } |
            Op::Div { left, right } => if left.contains(var) {
                Some(Side::Left)
            } else if right.contains(var) {
                Some(Side::Right)
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
struct Monkey {
    id: [char; 4],
    op: Op
}

pub struct Day21 {
    monkeys: Vec<Monkey>
}

const INPUT: &str = include_str!("../../input/day21");

impl Solver for Day21 {
    type Solution1 = isize;
    type Solution2 = isize;

    fn new() -> Self {
        Day21 { monkeys: vec!() }
    }

    fn reset(&mut self) {
        self.monkeys.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let id: [char; 4] = line[..4].as_array();
            let tokens: Vec<&str> = line[6..].split(' ').collect();
            let op = match tokens[..] {
                [number] => {
                    Op::Const { value: number.parse().unwrap() }
                },
                [left, op, right] => {
                    let left = Box::new(Op::Var { id: left.as_array() });
                    let right = Box::new(Op::Var { id: right.as_array() });
                    match op {
                        "+" => Op::Add { left, right },
                        "-" => Op::Sub { left, right },
                        "*" => Op::Mul { left, right },
                        "/" => Op::Div { left, right },
                        _ => panic!()
                    }
                },
                _ => panic!()
            };
            self.monkeys.push(Monkey { id, op });
        }
    }

    fn solve_part1(&self) -> isize {
        let mut monkeys: VecDeque<Monkey> = VecDeque::from_iter(self.monkeys.iter().map(|m| m.clone()));
        while monkeys.len() > 1 {
            let monkey = monkeys.pop_front().unwrap();
            if monkey.id == ['r', 'o', 'o', 't'] {
                monkeys.push_back(monkey);
                continue;
            }
            for other in monkeys.iter_mut() {
                other.op.replace(monkey.id, &monkey.op);
            }
        }

        let root = monkeys.pop_front().unwrap();
        root.op.eval()
    }

    fn solve_part2(&self) -> isize {
        let mut monkeys: VecDeque<Monkey> = VecDeque::from_iter(self.monkeys.iter().map(|m| m.clone()));
        let humn_id = ['h', 'u', 'm', 'n'];
        while monkeys.len() > 1 {
            let monkey = monkeys.pop_front().unwrap();
            if monkey.id == ['r', 'o', 'o', 't'] {
                monkeys.push_back(monkey);
                continue;
            } else if monkey.id == humn_id {
                continue;
            }
            for other in monkeys.iter_mut() {
                other.op.replace(monkey.id, &monkey.op);
            }
        }

        let root = monkeys.pop_front().unwrap();
        let (mut left, mut right) = match &root.op {
            Op::Const { value: _ } => panic!(),
            Op::Var { id: _ } => panic!(),
            Op::Add { left, right } |
            Op::Sub { left, right } |
            Op::Mul { left, right } |
            Op::Div { left, right } => ((**left).clone(), (**right).clone())
        };
        match root.op.find_side(humn_id) {
            Some(Side::Right) => swap(&mut left, &mut right),
            Some(Side::Left) => (),
            None => panic!()
        };

        loop {
            match &left {
                Op::Var { id } if *id == humn_id => break,
                _ => ()
            }

            let side = left.find_side(humn_id);
            (left, right) = match (left, side) {
                (Op::Const { value: _ }, _) => panic!(),
                (Op::Var { id: _ }, _) => panic!(),

                (Op::Add { left: rest, right: term }, Some(Side::Left)) |
                (Op::Add { left: term, right: rest }, Some(Side::Right)) => {
                    (*rest, Op::Sub { left: Box::new(right), right: term })
                },

                (Op::Sub { left: rest, right: term }, Some(Side::Left)) => {
                    (*rest, Op::Add { left: Box::new(right), right: term })
                },
                (Op::Sub { left: term, right: rest }, Some(Side::Right)) => {
                    (*rest, Op::Sub { left: term, right: Box::new(right) })
                },

                (Op::Mul { left: rest, right: factor }, Some(Side::Left)) |
                (Op::Mul { left: factor, right: rest }, Some(Side::Right)) => {
                    (*rest, Op::Div { left: Box::new(right), right: factor })
                },

                (Op::Div { left: rest, right: denom }, Some(Side::Left)) => {
                    (*rest, Op::Mul { left: Box::new(right), right: denom })
                },
                (Op::Div { left: numer, right: rest }, Some(Side::Right)) => {
                    (*rest, Op::Div { left: numer, right: Box::new(right) })
                },

                (_, _) => panic!()
            }
        }

        right.eval()
    }

    fn print_solutions(&self, part1: isize, part2: isize) {
        println!("Value yelled by root monkey: {part1}");
        println!("Value that I should yell: {part2}");
    }
}
