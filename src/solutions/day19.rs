use std::{fmt::Debug, str::FromStr, collections::{HashSet, VecDeque}};

use super::Solver;

#[derive(Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Resources {
    ore: i16,
    clay: i16,
    obsidian: i16,
    geode: i16
}

impl Resources {
    fn can_afford(&self, other: &Resources) -> bool {
        self.ore >= other.ore && self.clay >= other.clay && self.obsidian >= other.obsidian
    }

    fn build(&mut self, cost: &Resources) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
        self.geode -= cost.geode;
    }

    fn produce(&self, resources: &mut Resources) {
        resources.ore += self.ore;
        resources.clay += self.clay;
        resources.obsidian += self.obsidian;
        resources.geode += self.geode;
    }
}

struct Blueprint {
    blueprint_number: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources
}

impl Blueprint {
    fn build(&self, robot: Robot, resources: &mut Resources, robots: &mut Resources) {
        match robot {
            Robot::Ore => {
                resources.build(&self.ore_robot_cost);
                robots.ore += 1;
            },
            Robot::Clay => {
                resources.build(&self.clay_robot_cost);
                robots.clay += 1;
            },
            Robot::Obsidian => {
                resources.build(&self.obsidian_robot_cost);
                robots.obsidian += 1;
            },
            Robot::Geode => {
                resources.build(&self.geode_robot_cost);
                robots.geode += 1;
            }
        }
    }

    fn can_afford(&self, robot: Robot, resources: &Resources) -> bool {
        match robot {
            Robot::Ore => resources.can_afford(&self.ore_robot_cost),
            Robot::Clay => resources.can_afford(&self.clay_robot_cost),
            Robot::Obsidian => resources.can_afford(&self.obsidian_robot_cost),
            Robot::Geode => resources.can_afford(&self.geode_robot_cost)
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    resources: Resources,
    robots: Resources,
    time: u8
}

pub struct Day19 {
    blueprints: Vec<Blueprint>
}

fn search(blueprint: &Blueprint, time: u8) -> i16 {
    let max_ore_cost = *[
        blueprint.ore_robot_cost.ore,
        blueprint.clay_robot_cost.ore,
        blueprint.obsidian_robot_cost.ore,
        blueprint.geode_robot_cost.ore
    ].iter().max().unwrap();

    let mut states = VecDeque::new();
    states.push_back(State {
        resources: Resources { ore: 0, clay: 0, obsidian: 0, geode: 0 },
        robots: Resources { ore: 1, clay: 0, obsidian: 0, geode: 0 },
        time
    });
    let mut visited: HashSet<State> = HashSet::new();
    let mut most_geodes = 0;
    while !states.is_empty() {
        let mut state = states.pop_front().unwrap();

        let min_possible_geodes = state.resources.geode + state.robots.geode * state.time as i16;
        let potential_geodes = state.time as i16 * (state.time as i16 - 1) / 2;
        if most_geodes > min_possible_geodes + potential_geodes {
            continue;
        }

        state.robots.ore = state.robots.ore.min(max_ore_cost);
        state.robots.clay = state.robots.clay.min(blueprint.obsidian_robot_cost.clay);
        state.robots.obsidian = state.robots.obsidian.min(blueprint.geode_robot_cost.obsidian);
        state.resources.ore = {
            let max_ore_to_spend = state.time as isize * max_ore_cost as isize;
            let max_ore_to_produce = state.robots.ore as isize * (state.time as isize - 1);
            state.resources.ore.min((max_ore_to_spend - max_ore_to_produce) as i16)
        };
        state.resources.clay = {
            let max_clay_to_spend = state.time as isize * blueprint.obsidian_robot_cost.clay as isize;
            let max_clay_to_produce = state.robots.clay as isize * (state.time as isize - 1);
            state.resources.clay.min((max_clay_to_spend - max_clay_to_produce) as i16)
        };
        state.resources.obsidian = {
            let max_obsidian_to_spend = state.time as isize * blueprint.geode_robot_cost.obsidian as isize;
            let max_obsidian_to_produce = state.robots.obsidian as isize * (state.time as isize - 1);
            state.resources.obsidian.min((max_obsidian_to_spend - max_obsidian_to_produce) as i16)
        };
        
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);

        let mut new_resources = state.resources;
        state.robots.produce(&mut new_resources);
        if state.time == 1 {
            most_geodes = most_geodes.max(new_resources.geode);
            continue;
        }

        states.push_back(State { resources: new_resources, robots: state.robots, time: state.time - 1 });
        for robot in [Robot::Ore, Robot::Clay, Robot::Obsidian] {
            if !blueprint.can_afford(robot, &state.resources) {
                continue;
            }

            let mut new_resources = new_resources;
            let mut new_robots = state.robots;
            blueprint.build(robot, &mut new_resources, &mut new_robots);
            states.push_back(State { resources: new_resources, robots: new_robots, time: state.time - 1});
        }

        // If a new state builds a geode robot, then we can push it to the front of the state queue
        // to increase the most_geodes value sooner and therefore prune more inferior states later
        for robot in [Robot::Geode] {
            if !blueprint.can_afford(robot, &state.resources) {
                continue;
            }

            let mut new_resources = new_resources;
            let mut new_robots = state.robots;
            blueprint.build(robot, &mut new_resources, &mut new_robots);
            states.push_front(State { resources: new_resources, robots: new_robots, time: state.time - 1});
        }
    }

    most_geodes
}

const INPUT: &str = include_str!("../../input/day19");

impl Solver for Day19 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day19 { blueprints: vec!() }
    }

    fn reset(&mut self) {
        self.blueprints.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            fn receive<'a>(line: &'a str, prefix: &str) -> &'a str {
                if !line.starts_with(prefix) {
                    panic!();
                }
                return &line[prefix.len()..];
            }

            fn parse<'a, T: FromStr>(line: &'a str, delimiter: char) -> (&'a str, T)
            where
                <T as FromStr>::Err: Debug
            {
                let delimiter = line.find(delimiter).unwrap();
                let value: T = line[..delimiter].parse().unwrap();
                (&line[delimiter..], value)
            }

            let line = receive(line, "Blueprint ");
            let (line, blueprint_number) = parse::<usize>(line, ':');

            let line = receive(line, ": Each ore robot costs ");
            let (line, ore) = parse::<i16>(line, ' ');
            let ore_robot_cost = Resources { ore, clay: 0, obsidian: 0, geode: 0 };
            
            let line = receive(line, " ore. Each clay robot costs ");
            let (line, ore) = parse::<i16>(line, ' ');
            let clay_robot_cost = Resources { ore, clay: 0, obsidian: 0, geode: 0 };

            let line = receive(line, " ore. Each obsidian robot costs ");
            let (line, ore) = parse::<i16>(line, ' ');
            let line = receive(line, " ore and ");
            let (line, clay) = parse::<i16>(line, ' ');
            let obsidian_robot_cost = Resources { ore, clay, obsidian: 0, geode: 0 };

            let line = receive(line, " clay. Each geode robot costs ");
            let (line, ore) = parse::<i16>(line, ' ');
            let line = receive(line, " ore and ");
            let (line, obsidian) = parse::<i16>(line, ' ');
            let geode_robot_cost = Resources { ore, clay: 0, obsidian, geode: 0 };

            let _line = receive(line, " obsidian.");

            self.blueprints.push(Blueprint {
                blueprint_number,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost
            });
        }
    }

    fn solve_part1(&self) -> usize {
        let mut quality_level = 0usize;

        for blueprint in &self.blueprints {
            let most_geodes = search(blueprint, 24);
            quality_level += most_geodes as usize * blueprint.blueprint_number as usize;
        }

        quality_level
    }

    fn solve_part2(&self) -> usize {
        let mut product = 1usize;

        for blueprint in self.blueprints.iter().take(3) {
            let most_geodes = search(blueprint, 32);
            product *= most_geodes as usize;
        }

        product
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Total quality level: {part1}");
        println!("Product of most geodes: {part2}");
    }
}
