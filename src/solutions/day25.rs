use super::Solver;

pub struct Day25<'a> {
    fuel_reqs: Vec<&'a str>
}

const INPUT: &str = include_str!("../../input/day25");

impl<'a> Solver for Day25<'a> {
    type Solution1 = String;
    type Solution2 = ();

    fn new() -> Self {
        Day25 { fuel_reqs: vec!() }
    }

    fn reset(&mut self) {
        self.fuel_reqs.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            self.fuel_reqs.push(line);
        }
    }

    fn solve_part1(&self) -> String {
        let total_req: isize = self.fuel_reqs
            .iter()
            .map(|req| {
                req
                    .chars()
                    .rev()
                    .fold((1, 0), |(base, acc), ch| {
                        let digit = match ch {
                            '=' => -2,
                            '-' => -1,
                            '0' => 0,
                            '1' => 1,
                            '2' => 2,
                            _ => panic!()
                        };
                        (base * 5, acc + base * digit)
                    })
                    .1
            })
            .sum();
        let mut snafu_req = String::new();
        let mut rest = total_req.abs();
        while rest > 0 {
            snafu_req.insert(0, match (rest + 2).rem_euclid(5) {
                0 => '=',
                1 => '-',
                2 => '0',
                3 => '1',
                4 => '2',
                _ => unreachable!()
            });
            rest = (rest + 2) / 5;
        }
        if total_req.is_negative() {
            snafu_req = snafu_req
                .chars()
                .map(|ch| match ch {
                    '2' => '=',
                    '1' => '-',
                    '0' => '0',
                    '-' => '1',
                    '=' => '2',
                    _ => unreachable!()
                })
                .collect();
        }

        snafu_req
    }

    fn solve_part2(&self) -> () {
        // Hardest puzzle yet:
        ()
    }

    fn print_solutions(&self, part1: String, _part2: ()) {
        println!("Total fuel requirement in SNAFU: {part1}");
        println!("Go make a smoothie");
    }
}
