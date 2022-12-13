use std::{cmp::Ordering, fmt::Display};

use super::Solver;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>)
}

impl Packet {
    fn parse(str: &[char]) -> Packet {
        fn parse_list_item(str: &mut &[char], items: &mut Vec<Packet>) {
            let item: Option<Packet>;
            (item, *str) = iter(&str[1..]);
            if let Some(item) = item {
                items.push(item);
            }
        }
        
        fn iter(mut str: &[char]) -> (Option<Packet>, &[char]) {
            match str.first() {
                Some('[') => {
                    let mut items: Vec<Packet> = vec!();
                    parse_list_item(&mut str, &mut items);
                    while let Some(',') = str.first() {
                        parse_list_item(&mut str, &mut items);
                    }
                    if str.is_empty() {
                        (Some(Packet::List(items)), str)
                    } else {
                        (Some(Packet::List(items)), &str[1..])
                    }
                },
                Some(']') => {
                    (None, str)
                },
                Some('0'..='9') => {
                    let mut number = 0u8;
                    while let Some('0'..='9') = str.first() {
                        number = number * 10 + *str.first().unwrap() as u8 - '0' as u8;
                        str = &str[1..];
                    }
                    (Some(Packet::Int(number)), str)
                },
                Some(_) => panic!(),
                None => {
                    (None, str)
                }
            }
        }
        iter(str).0.unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(left), Packet::Int(right)) => left.cmp(right),
            (Packet::List(left), Packet::List(right)) => {
                left.iter().zip(right.iter())
                    .find_map(|(left, right)| match left.cmp(right) {
                        Ordering::Equal => None,
                        ord => Some(ord)
                    })
                    .unwrap_or(left.len().cmp(&right.len()))
            }
            (Packet::List(_), Packet::Int(right)) =>
                self.cmp(&Packet::List(vec!(Packet::Int(*right)))),
            (Packet::Int(left), Packet::List(_)) =>
                Packet::List(vec!(Packet::Int(*left))).cmp(other)
        }
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(val) => write!(f, "{val}"),
            Packet::List(items) => {
                write!(f, "[")?;
                if let Some(item) = items.first() {
                    write!(f, "{item}")?;
                }
                for item in items.iter().skip(1) {
                    write!(f, ",{item}")?;
                }
                write!(f, "]")
            }
        }
    }
}

struct Pair {
    left: Packet,
    right: Packet
}

pub struct Day13 {
    pairs: Vec<Pair>
}

const INPUT: &str = include_str!("../../input/day13");

impl Solver for Day13 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day13 { pairs: vec!() }
    }

    fn reset(&mut self) {
        self.pairs.clear();
    }

    fn parse_input(&mut self) {
        for pair in INPUT.split("\n\n") {
            let mut pair = pair.lines();
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();
            
            let left = Packet::parse(left.chars().collect::<Vec<char>>().as_slice());
            let right = Packet::parse(right.chars().collect::<Vec<char>>().as_slice());
            self.pairs.push(Pair {
                left, right
            });
        }
    }

    fn solve_part1(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter_map(|(index, pair)| match pair.left.cmp(&pair.right) {
                Ordering::Less => Some(index + 1),
                _ => None
            })
            .sum()
    }

    fn solve_part2(&self) -> usize {
        let mut packets: Vec<&Packet> = self.pairs
            .iter()
            .flat_map(|pair| [&pair.left, &pair.right])
            .collect();
        packets.sort();
        let divider_two = Packet::List(vec!(Packet::List(vec!(Packet::Int(2)))));
        let divider_six = Packet::List(vec!(Packet::List(vec!(Packet::Int(6)))));
        let two_index = match packets.binary_search(&&divider_two) {
            Ok(index) | Err(index) => index
        };
        let six_index = match packets.binary_search(&&divider_six) {
            Ok(index) | Err(index) => index
        };

        (two_index + 1) * (six_index + 2)
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Sum of indices of correctly ordered packets: {part1}");
        println!("Decoder key for the distress signal: {part2}");
    }
}
