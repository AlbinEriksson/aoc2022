use super::Solver;

type Score = u32;

#[derive(Copy, Clone)]
enum RoundResult {
    Loss,
    Draw,
    Win
}

impl RoundResult {
    fn get_score(self) -> Score {
        match self {
            RoundResult::Loss => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6
        }
    }

    fn opposite(self) -> RoundResult {
        match self {
            RoundResult::Loss => RoundResult::Win,
            RoundResult::Draw => RoundResult::Draw,
            RoundResult::Win => RoundResult::Loss,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn get_result_against(self, other: Shape) -> RoundResult {
        match (self, other) {
            (a, b) if a == b => RoundResult::Draw,
            (Shape::Rock, Shape::Paper) => RoundResult::Loss,
            (Shape::Rock, Shape::Scissors) => RoundResult::Win,
            (Shape::Paper, Shape::Scissors) => RoundResult::Loss,
            (Shape::Paper, Shape::Rock) => RoundResult::Win,
            (Shape::Scissors, Shape::Rock) => RoundResult::Loss,
            (Shape::Scissors, Shape::Paper) => RoundResult::Win,
            (_, _) => unreachable!()
        }
    }

    fn get_matchup(self, result: RoundResult) -> Shape {
        match (self, result) {
            (_, RoundResult::Draw) => self,
            (Shape::Rock, RoundResult::Loss) => Shape::Paper,
            (Shape::Rock, RoundResult::Win) => Shape::Scissors,
            (Shape::Paper, RoundResult::Loss) => Shape::Scissors,
            (Shape::Paper, RoundResult::Win) => Shape::Rock,
            (Shape::Scissors, RoundResult::Loss) => Shape::Rock,
            (Shape::Scissors, RoundResult::Win) => Shape::Paper
        }
    }

    fn get_score(self) -> Score {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

struct Round {
    wrong_choice: Shape,
    opponent: Shape,
    desired_result: RoundResult
}

impl Round {
    fn get_result(&self) -> RoundResult {
        self.wrong_choice.get_result_against(self.opponent)
    }
}

pub struct Day2 {
    rounds: Vec<Round>
}

const INPUT: &str = include_str!("../../input/day2");

impl Solver for Day2 {
    type Solution1 = Score;
    type Solution2 = Score;

    fn new() -> Self {
        Day2 { rounds: vec!() }
    }

    fn reset(&mut self) {
        self.rounds.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            if line.is_empty() {
                continue
            }
            let opponent = match line.chars().nth(0).unwrap() {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => unreachable!()
            };
            let (wrong_choice, desired_result) = match line.chars().nth(2).unwrap() {
                'X' => (Shape::Rock, RoundResult::Loss),
                'Y' => (Shape::Paper, RoundResult::Draw),
                'Z' => (Shape::Scissors, RoundResult::Win),
                _ => unreachable!()
            };
            self.rounds.push(Round { wrong_choice, opponent, desired_result });
        }
    }

    fn solve_part1(&self) -> Score {
        self.rounds
            .iter()
            .map(|round| round.get_result().get_score() + round.wrong_choice.get_score())
            .sum()
    }

    fn solve_part2(&self) -> Score {
        self.rounds
            .iter()
            .map(|round|
                round.desired_result.get_score() +
                round.opponent.get_matchup(round.desired_result.opposite()).get_score()
            )
            .sum()
    }

    fn print_solutions(&self, part1: Score, part2: Score) {
        println!("Total score for wrong strategy: {part1}");
        println!("Total score for correct strategy: {part2}");
    }
}
