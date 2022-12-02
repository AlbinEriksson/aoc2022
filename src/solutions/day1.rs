use super::Solver;

type Calories = u32;

#[derive(Debug)]
pub struct Elf {
    food_calories: Vec<Calories>
}

impl Elf {
    fn get_total_calories(&self) -> Calories {
        self.food_calories.iter().sum()
    }
}

#[derive(Debug)]
pub struct Day1 {
    elves: Vec<Elf>
}

const INPUT: &str = include_str!("../../input/day1");

impl Day1 {
    fn add_elf(&mut self) -> &mut Elf {
        self.elves.push(Elf { food_calories: vec!() });
        self.elves.last_mut().unwrap()
    }
}

impl Solver for Day1 {
    type Solution1 = Calories;
    type Solution2 = Calories;

    fn new() -> Self {
        Day1 {
            elves: vec!()
        }
    }

    fn reset(&mut self) {
        self.elves.clear();
    }

    fn parse_input(&mut self) {
        let mut elf = self.add_elf();
        for line in INPUT.lines() {
            if line.is_empty() {
                elf = self.add_elf();
            } else {
                let calories: Calories = line.parse().expect("Failed to parse calorie count");
                elf.food_calories.push(calories);
            }
        }
    }

    fn solve_part1(&self) -> Calories {
        self.elves
            .iter()
            .map(|elf| elf.get_total_calories())
            .max()
            .unwrap()
    }

    fn solve_part2(&self) -> Calories {
        let mut top3_calories: [Calories; 3] = [0, 0, 0];
        for calories in self.elves.iter().map(|elf| elf.get_total_calories()) {
            let index = match top3_calories.binary_search(&calories) {
                Ok(0) | Err(0) => continue,
                Ok(index) | Err(index) => index,
            };
            top3_calories.copy_within(1..index, 0);
            top3_calories[index - 1] = calories;
        }
        top3_calories.iter().sum()
    }

    fn print_solutions(&self, part1: Calories, part2: Calories) {
        println!("Most calories carried: {part1}");
        println!("Sum of 3 most calories carried: {part2}");
    }
}
