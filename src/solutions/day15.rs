use crate::util::pos::Pos2d;

use super::Solver;

type Pos = Pos2d<i32>;

struct Sensor {
    pos: Pos,
    beacon: Pos
}

pub struct Day15 {
    sensors: Vec<Sensor>
}

impl Day15 {
    fn get_beacons_at(&self, y: i32) -> Vec<i32> {
        let mut row_beacons: Vec<i32> = self.sensors
            .iter()
            .filter_map(|sensor| {
                if sensor.beacon.y == y {
                    Some(sensor.beacon.x)
                } else {
                    None
                }
            })
            .collect();
        row_beacons.dedup();
        row_beacons
    }

    fn get_sensor_ranges_at(&self, y: i32) -> Vec<(i32, i32)> {
        self.sensors
            .iter()
            .filter_map(|sensor| {
                let beacon_dist =
                    sensor.pos.x.abs_diff(sensor.beacon.x) +
                    sensor.pos.y.abs_diff(sensor.beacon.y);
                let row_dist = sensor.pos.y.abs_diff(y);
                let row_range = beacon_dist.saturating_sub(row_dist);
                if row_range == 0 {
                    None
                } else {
                    Some((sensor.pos.x - row_range as i32, sensor.pos.x + row_range as i32))
                }
            })
            .collect()
    }
}

fn merge_ranges(ranges: &mut Vec<(i32, i32)>) {
    loop {
        let mut found = false;
        'outer: for i in 0..ranges.len() {
            for j in i+1..ranges.len() {
                if ranges[i].0 <= ranges[j].1 && ranges[i].1 >= ranges[j].0 {
                    let merged = (
                        ranges[i].0.min(ranges[j].0),
                        ranges[i].1.max(ranges[j].1)
                    );
                    ranges.swap_remove(j);
                    ranges.swap_remove(i);
                    ranges.push(merged);
                    found = true;
                    break 'outer;
                }
            }
        }
        if !found {
            break;
        }
    }
}

const INPUT: &str = include_str!("../../input/day15");

impl Solver for Day15 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day15 { sensors: vec!() }
    }

    fn reset(&mut self) {
        self.sensors.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            if !line.starts_with("Sensor at x=") {
                panic!();
            }

            let line = &line[12..];
            let delimiter = line.find(',').unwrap();
            let x: i32 = line[..delimiter].parse().unwrap();

            let line = &line[delimiter..];
            if !line.starts_with(", y=") {
                panic!();
            }

            let line = &line[4..];
            let delimiter = line.find(':').unwrap();
            let y: i32 = line[..delimiter].parse().unwrap();

            let pos = Pos::new(x, y);

            let line = &line[delimiter..];
            if !line.starts_with(": closest beacon is at x=") {
                panic!();
            }

            let line = &line[25..];
            let delimiter = line.find(',').unwrap();
            let x: i32 = line[..delimiter].parse().unwrap();

            let line = &line[delimiter..];
            if !line.starts_with(", y=") {
                panic!();
            }

            let line = &line[4..];
            let y: i32 = line.parse().unwrap();

            let beacon = Pos::new(x, y);

            self.sensors.push(Sensor { pos, beacon });
        }
    }

    fn solve_part1(&self) -> usize {
        let y = 2000000;
        let row_beacons = self.get_beacons_at(y);
        let mut sensor_ranges = self.get_sensor_ranges_at(y);
        merge_ranges(&mut sensor_ranges);
        let total_range: usize = sensor_ranges
            .iter()
            .map(|(min, max)| (max - min + 1) as usize)
            .sum();
        total_range - row_beacons.len()
    }

    fn solve_part2(&self) -> Self::Solution2 {
        for y in 0..=4000000 {
            let mut sensor_ranges = self.get_sensor_ranges_at(y);
            merge_ranges(&mut sensor_ranges);
            if sensor_ranges.len() == 1 {
                continue;
            }
            if sensor_ranges.len() != 2 {
                panic!();
            }
            let a = sensor_ranges[0];
            let b = sensor_ranges[1];
            let x = if a.1 < b.0 {
                a.1 + 1
            } else {
                a.0 - 1
            };
            return x as usize * 4000000 + y as usize;
        }
        0
    }

    fn print_solutions(&self, part1: usize, part2: Self::Solution2) {
        println!("Positions at y=2000000 with no beacons: {part1}");
        println!("Tuning frequency of distress beacon: {part2}");
    }
}
