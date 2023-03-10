pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub trait Solver {
    type Solution1;
    type Solution2;

    fn new() -> Self;
    fn reset(&mut self);
    fn parse_input(&mut self);
    fn solve_part1(&self) -> Self::Solution1;
    fn solve_part2(&self) -> Self::Solution2;
    fn print_solutions(&self, part1: Self::Solution1, part2: Self::Solution2);

    fn run(repeat_count: u32) where Self: Sized {
        let mut solver = Self::new();
        for i in 0..repeat_count {
            solver.parse_input();
            let part1 = solver.solve_part1();
            let part2 = solver.solve_part2();
            if i == repeat_count - 1 {
                solver.print_solutions(part1, part2);
            }
            solver.reset();
        }
    }
}
