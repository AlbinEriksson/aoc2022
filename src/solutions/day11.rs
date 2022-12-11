use core::panic;

use crate::util::top::TopIter;

use super::Solver;

type Item = usize;

#[derive(Clone, Debug)]
enum WorryOp {
    Add(u8),
    Multiply(u8),
    Square
}

impl WorryOp {
    fn apply(&self, item: Item) -> Item {
        match self {
            WorryOp::Add(term) => item + *term as Item,
            WorryOp::Multiply(factor) => {
                item.checked_mul(*factor as Item).unwrap_or_else(||
                    panic!("{self:?}, {item:?}")
                )
            },
            WorryOp::Square => item * item
        }
    }
}

type MonkeyID = u8;

#[derive(Clone, Debug)]
struct Test {
    div: u8,
    if_true: MonkeyID,
    if_false: MonkeyID
}

impl Test {
    fn check_item(&self, item: Item) -> MonkeyID {
        if item % self.div as Item == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<Item>,
    worry_op: WorryOp,
    test: Test
}

pub struct Day11 {
    monkeys: Vec<Monkey>
}

impl Day11 {
    fn keep_away(&self, num_rounds: usize, relief: bool) -> usize {
        let mut monkeys = self.monkeys.clone();
        let mut inspections = vec![0; monkeys.len()];
        // Turns out the worry levels get so big that we must use modular arithmetic to fit them in
        // 64 bits or less while leaving the divisibility tests unaffected.
        let modulo: usize = monkeys
            .iter()
            .map(|monkey| monkey.test.div as usize)
            .product();
        for _round in 0..num_rounds {
            for i in 0..monkeys.len() {
                for j in 0..monkeys[i].items.len() {
                    let item = monkeys[i].items[j];
                    let mut worry_level = monkeys[i].worry_op.apply(item);
                    if relief {
                        worry_level /= 3;
                    }
                    worry_level %= modulo;
                    let new_monkey = monkeys[i].test.check_item(worry_level);
                    monkeys[new_monkey as usize].items.push(worry_level);
                }
                inspections[i] += monkeys[i].items.len();
                monkeys[i].items.clear();
            }
        }

        inspections
            .iter()
            .top(2)
            .iter()
            .map(|x| **x)
            .reduce(|x, y| x * y)
            .unwrap()
    }
}

const INPUT: &str = include_str!("../../input/day11");

impl Solver for Day11 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day11 { monkeys: vec!() }
    }

    fn reset(&mut self) {
        self.monkeys.clear();
    }

    fn parse_input(&mut self) {
        let mut lines = INPUT.lines().enumerate().map(|(index, line)| (index + 1, line));
        let mut last_monkey_id: Option<MonkeyID> = None;
        let (mut index, mut line);
        loop {
            fn next_line<'a>(lines: &mut impl Iterator<Item = (usize, &'a str)>,
            line_start: &str) -> Option<(usize, &'a str)> {
                lines.next()
                    .and_then(|(index, line)| {
                        if !line.starts_with(line_start) {
                            panic!("Line {index}: Expected to start with '{line_start}' but got \
                                '{line}'")
                        }
                        Some((index, line))
                    })
            }

            fn get_line<'a>(index: usize, lines: &mut impl Iterator<Item = (usize, &'a str)>,
            line_start: &str) -> (usize, &'a str) {
                next_line(lines, line_start).unwrap_or_else(||
                    panic!("Line {}: Expected line to be '{line_start}...' but got end of file",
                        index + 1)
                )
            }

            fn check_end(index: usize, line: &str, line_end: &str) {
                if !line.ends_with(line_end) {
                    panic!("Line {index}: Expected to end with '{line_end}' but got '{line}'");
                }
            }

            (index, line) = match next_line(&mut lines, "Monkey ") {
                Some((index, line)) => (index, line),
                None => break
            };
            check_end(index, line, ":");
            let monkey_id = &line[7..line.len() - 1];
            let monkey_id: MonkeyID = monkey_id.parse().unwrap_or_else(|_|
                panic!("Line {index}: Failed to parse monkey ID: \
                    {monkey_id}")
            );
            if let Some(last_monkey_id) = last_monkey_id {
                if last_monkey_id + 1 != monkey_id {
                    panic!("Line {index}: Expected monkey {monkey_id} directly after monkey \
                        {last_monkey_id}");
                }
            }
            last_monkey_id = Some(monkey_id);

            (index, line) = get_line(index, &mut lines, "  Starting items: ");
            let items: Vec<Item> = line[18..]
                .split(", ")
                .map(|item| item.parse().unwrap_or_else(
                    |_| panic!("Line {index}: Failed to parse item: {item}")
                ))
                .collect();
            
            (index, line) = get_line(index, &mut lines, "  Operation: ");
            let worry_op = &line[13..];
            let worry_op = match (&worry_op[..12], &worry_op[12..]) {
                ("new = old + ", term) => WorryOp::Add(term.parse().unwrap()),
                ("new = old * ", "old") => WorryOp::Square,
                ("new = old * ", factor) => WorryOp::Multiply(factor.parse().unwrap()),
                (_, _) => panic!("Line {index}: Failed to parse operation: '{worry_op}'")
            };

            (index, line) = get_line(index, &mut lines, "  Test: ");
            let div = &line[21..];
            let div: u8 = div.parse().unwrap_or_else(
                |_| panic!("Line {index}: Failed to parse divisibility: {div}")
            );

            (index, line) = get_line(index, &mut lines, "    If true: throw to monkey ");
            let if_true = &line[29..];
            let if_true: MonkeyID = if_true.parse().unwrap_or_else(|_| {
                panic!("Line {index}: Failed to parse monkey ID: {if_true}")
            });

            (index, line) = get_line(index, &mut lines, "    If false: throw to monkey ");
            let if_false = &line[30..];
            let if_false: MonkeyID = if_false.parse().unwrap_or_else(|_| {
                panic!("Line {index}: Failed to parse monkey ID: {if_false}")
            });

            if let Some((index, line)) = lines.next() {
                if !line.is_empty() {
                    panic!("Line {index}: Expected blank line or end of file after monkey block, \
                        got '{line}'");
                }
            }

            self.monkeys.push(Monkey {
                items,
                worry_op,
                test: Test {
                    div,
                    if_true,
                    if_false
                }
            });
        }
    }

    fn solve_part1(&self) -> usize {
        self.keep_away(20, true)
    }

    fn solve_part2(&self) -> usize {
        self.keep_away(10000, false)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Max monkey business after 20 rounds with relief: {part1}");
        println!("Max monkey business after 10000 rounds: {part2}");
    }
}
