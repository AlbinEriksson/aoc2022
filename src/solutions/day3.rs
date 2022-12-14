use crate::util::intset::IntSet;

use super::Solver;

struct Priority(u8);

impl TryFrom<char> for Priority {
    type Error = ();

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        match ch {
            ch if ch.is_lowercase() => Ok(Priority(ch as u8 - 'a' as u8 + 1)),
            ch if ch.is_uppercase() => Ok(Priority(ch as u8 - 'A' as u8 + 27)),
            _ => Err(())
        }
    }
}

struct Compartment<'a>(&'a str);

impl<'a> Compartment<'a> {
    fn get_priority_set(&self) -> IntSet {
        self.0
            .chars()
            .into_iter()
            .fold(IntSet::new(1, 52), |mut set, item| {
                if let Ok(priority) = <char as TryInto<Priority>>::try_into(item) {
                    set.add(priority.0 as isize);
                }
                set
            })
    }

    fn find_shared_priority(&self, priority_set: &IntSet) -> Option<u8> {
        self.0
            .chars()
            .into_iter()
            .map(|ch| <char as TryInto<Priority>>::try_into(ch).unwrap())
            .map(|priority| priority.0)
            .find(|priority| priority_set.contains(*priority as isize))
    }
}

struct Rucksack {
    items: String
}

impl Rucksack {
    fn get_first_compartment(&self) -> Compartment {
        let middle = self.items.len() / 2;
        Compartment(&self.items[0..middle])
    }

    fn get_second_compartment(&self) -> Compartment {
        let end = self.items.len();
        let middle = end / 2;
        Compartment(&self.items[middle..end])
    }

    fn get_both_compartments(&self) -> Compartment {
        Compartment(self.items.as_str())
    }
}

pub struct Day3 {
    rucksacks: Vec<Rucksack>
}

const INPUT: &str = include_str!("../../input/day3");

impl Solver for Day3 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day3 { rucksacks: vec!() }
    }

    fn reset(&mut self) {
        self.rucksacks.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            self.rucksacks.push(Rucksack { items: line.to_string() });
        }
    }

    fn solve_part1(&self) -> usize {
        self.rucksacks
            .iter()
            .map(|rucksack| {
                let priority_set = rucksack.get_first_compartment().get_priority_set();
                rucksack.get_second_compartment().find_shared_priority(&priority_set).unwrap_or(0)
            })
            .map(|v| v as usize)
            .sum()
    }

    fn solve_part2(&self) -> usize {
        self.rucksacks
            .chunks(3)
            .map(|chunk| {
                let mut priority_set = chunk[0].get_both_compartments().get_priority_set();
                priority_set.intersect(&chunk[1].get_both_compartments().get_priority_set());
                chunk[2].get_both_compartments().find_shared_priority(&priority_set).unwrap()
            })
            .map(|v| v as usize)
            .sum()
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Total priority of items in both compartments: {part1}");
        println!("Total priority of all group badges: {part2}");
    }
}
