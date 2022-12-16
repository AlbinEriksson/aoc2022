use std::collections::{HashMap, HashSet};

use super::Solver;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd)]
struct ValveId([char; 2]);

impl FromIterator<char> for ValveId {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut arr = ['\0', '\0'];
        let mut iter = iter.into_iter();
        arr[0] = iter.next().unwrap();
        arr[1] = iter.next().unwrap();
        ValveId(arr)
    }
}

#[derive(Clone, Copy, Debug)]
struct Tunnel {
    length: u8,
    dest: ValveId
}

#[derive(Clone, Debug)]
struct Valve {
    flow_rate: u8,
    tunnels: Vec<Tunnel>
}

impl Valve {
    fn remove_tunnel_to(&mut self, other: ValveId) {
        if other == ValveId(['I', 'Q']) {
            print!("");
        }
        if let Some(tunnel_index) = self
            .tunnels
            .iter()
            .enumerate()
            .find(|(_idx, tunnel)| tunnel.dest == other)
            .map(|(idx, _tunnel)| idx)
        {
            self.tunnels.swap_remove(tunnel_index);
        }
    }

    fn add_or_merge_tunnel(&mut self, length: u8, dest: ValveId) {
        if let Some(tunnel_0_to_1) = self
            .tunnels
            .iter_mut()
            .find(|tunnel| tunnel.dest == dest)
        {
            tunnel_0_to_1.length = tunnel_0_to_1.length.min(length);
        } else {
            self.tunnels.push(Tunnel { length, dest });
        }
    }
}

#[derive(PartialEq, Eq)]
struct Cost {
    time_left: isize,
    current_pressure: usize,
    added_pressure: usize,
    valve_id: ValveId,
    opened_valves: HashSet<ValveId>
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.current_pressure + self.added_pressure as usize).partial_cmp(
            &(other.current_pressure + other.added_pressure))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.current_pressure + self.added_pressure as usize).cmp(
            &(other.current_pressure + other.added_pressure))
    }
}

pub struct Day16 {
    valves: HashMap<ValveId, Valve>
}

impl Day16 {
    fn exclude_broken_valves(&self, starting_valve_id: &ValveId) -> HashMap<ValveId, Valve> {
        let mut valves = self.valves.clone();
        loop {
            let broken_valve_id = loop {
                let mut valve_id: Option<ValveId> = None;
                for (id, valve) in &valves {
                    if valve.flow_rate == 0 && id != starting_valve_id {
                        valve_id = Some(*id);
                        break;
                    }
                }
                break valve_id;
            };

            if broken_valve_id.is_none() {
                break valves;
            }
            let broken_valve_id = broken_valve_id.unwrap();
            let broken_valve = valves.remove(&broken_valve_id).unwrap();
            
            let num_tunnels = broken_valve.tunnels.len();
            for tunnel_idx_0 in 0..num_tunnels {
                for tunnel_idx_1 in tunnel_idx_0+1..num_tunnels {
                    let tunnel_0 = broken_valve.tunnels[tunnel_idx_0];
                    let tunnel_1 = broken_valve.tunnels[tunnel_idx_1];

                    let valve_0 = valves.get_mut(&tunnel_0.dest).unwrap();
                    valve_0.remove_tunnel_to(broken_valve_id);
                    valve_0.add_or_merge_tunnel(tunnel_0.length + tunnel_1.length, tunnel_1.dest);

                    let valve_1 = valves.get_mut(&tunnel_1.dest).unwrap();
                    valve_1.remove_tunnel_to(broken_valve_id);
                    valve_1.add_or_merge_tunnel(tunnel_0.length + tunnel_1.length, tunnel_0.dest);
                }
            }
        }
    }
}

fn floyd_warshall(valves: &HashMap<ValveId, Valve>, valve_indices: &HashMap<ValveId, usize>)
    -> Vec<u8>
{
    let num_valves = valves.len();
    let num_paths = num_valves * num_valves;
    let mut matrix: Vec<u8> = vec![u8::MAX; num_paths];
    for (id, valve) in valves {
        let a = valve_indices[&id];
        for tunnel in &valve.tunnels {
            let b = valve_indices[&tunnel.dest];
            matrix[a * num_valves + b] = tunnel.length;
        }
        matrix[a * num_valves + a] = 0;
    }
    for alt in 0..num_valves {
        for src in 0..num_valves {
            for dst in 0..num_valves {
                let cur_dist = matrix[src * num_valves + dst];
                let alt_dist = matrix[src * num_valves + alt].saturating_add(
                    matrix[alt * num_valves + dst]);

                if alt_dist < cur_dist {
                    matrix[src * num_valves + dst] = alt_dist;
                }
            }
        }
    }
    matrix
}

#[derive(Hash, PartialEq, Eq)]
struct Memo {
    current_valve: ValveId,
    remaining_valves: Vec<ValveId>,
    time_left: usize
}

fn dfs<F>(matrix: &Vec<u8>, valve_indices: &HashMap<ValveId, usize>,
    valves: &HashMap<ValveId, Valve>, current: Memo, memos: &mut HashMap<Memo, usize>, fallback: F)
    -> usize
where
    F: Copy + Fn(&Vec<ValveId>, &mut HashMap<Memo, usize>) -> usize
{
    if let Some(flow) = memos.get(&current) {
        return *flow;
    }

    let mut max_flow = 0;
    let num_valves = valves.len();
    for valve in &current.remaining_valves {
        let src = valve_indices[&current.current_valve];
        let dst = valve_indices[valve];
        let distance = matrix[src * num_valves + dst];
        if (distance as usize) < current.time_left {
            let time_left = current.time_left - distance as usize - 1;
            let new_flow = valves[&valve].flow_rate as usize * time_left;
            let remaining_valves: Vec<ValveId> = current.remaining_valves
                .iter()
                .filter(|valve_id| *valve_id != valve)
                .map(|valve| *valve)
                .collect();
            let dfs_flow = dfs(matrix, valve_indices, valves, Memo { current_valve: *valve, remaining_valves: remaining_valves.clone(), time_left }, memos, fallback);
            let dfs_flow = if dfs_flow > 0 {
                dfs_flow
            } else {
                fallback(&remaining_valves, memos)
            };
            let flow = new_flow + dfs_flow;
            max_flow = max_flow.max(flow);
        }
    }
    memos.insert(current, max_flow);
    max_flow
}

const INPUT: &str = include_str!("../../input/day16");

impl Solver for Day16 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day16 { valves: HashMap::new() }
    }

    fn reset(&mut self) {
        self.valves.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            if !line.starts_with("Valve ") {
                panic!();
            }

            let line = &line[6..];
            let delimiter = line.find(' ').unwrap();
            let key: ValveId = line[..delimiter].chars().collect();

            let line = &line[delimiter..];
            if !line.starts_with(" has flow rate=") {
                panic!();
            }

            let line = &line[15..];
            let delimiter = line.find(';').unwrap();
            let flow_rate: u8 = line[..delimiter].parse().unwrap();

            let line = &line[delimiter..];
            let line = if line.starts_with("; tunnels lead to valves ") {
                &line[25..]
            } else if line.starts_with("; tunnel leads to valve ") {
                &line[24..]
            } else {
                panic!();
            };

            let tunnels: Vec<Tunnel> = line
                .split(", ")
                .map(|valve| valve.chars().collect::<ValveId>())
                .map(|valve| Tunnel { length: 1, dest: valve })
                .collect();
            
            self.valves.insert(key, Valve { flow_rate, tunnels });
        }
    }

    fn solve_part1(&self) -> usize {
        let starting_key = ValveId(['A', 'A']);
        let valves = self.exclude_broken_valves(&starting_key);

        let valve_indices: HashMap<ValveId, usize> = valves
            .iter().enumerate().map(|(index, (key, _))| (*key, index)).collect();
        let matrix = floyd_warshall(&valves, &valve_indices);

        let remaining_valves: Vec<ValveId> = valves
            .keys().filter(|key| **key != starting_key).map(|key| *key).collect();
        let mut memos: HashMap<Memo, usize> = HashMap::new();
        dfs(&matrix, &valve_indices, &valves, Memo { current_valve: starting_key, remaining_valves, time_left: 30 }, &mut memos, |_, _| 0)
    }

    fn solve_part2(&self) -> usize {
        let starting_key = ValveId(['A', 'A']);
        let valves = self.exclude_broken_valves(&starting_key);

        let valve_indices: HashMap<ValveId, usize> = valves
            .iter().enumerate().map(|(index, (key, _))| (*key, index)).collect();
        let matrix = floyd_warshall(&valves, &valve_indices);

        let remaining_valves: Vec<ValveId> = valves
            .keys().filter(|key| **key != starting_key).map(|key| *key).collect();
        let mut memos: HashMap<Memo, usize> = HashMap::new();
        dfs(&matrix, &valve_indices, &valves, Memo { current_valve: starting_key, remaining_valves, time_left: 26 }, &mut memos, |remaining_valves, memos| {
            dfs(&matrix, &valve_indices, &valves, Memo { current_valve: starting_key, remaining_valves: remaining_valves.clone(), time_left: 26 }, memos, |_, _| 0)
        })
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Maximum flow: {part1}");
        println!("Maximum flow with helper elephant: {part2}");
    }
}
