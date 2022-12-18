use std::collections::HashSet;

use crate::util::pos::Pos3d;

use super::Solver;

type Pos = Pos3d<i8>;

pub struct Day18 {
    voxels: Vec<Pos>
}

const INPUT: &str = include_str!("../../input/day18");

impl Solver for Day18 {
    type Solution1 = usize;
    type Solution2 = usize;

    fn new() -> Self {
        Day18 { voxels: vec!() }
    }

    fn reset(&mut self) {
        self.voxels.clear();
    }

    fn parse_input(&mut self) {
        for line in INPUT.lines() {
            let mut values = line.split(',');
            self.voxels.push(Pos::new(
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap(),
                values.next().unwrap().parse().unwrap()
            ));
        }
    }

    fn solve_part1(&self) -> usize {
        let droplet: HashSet<Pos> = HashSet::from_iter(self.voxels.iter().map(|x| *x));
        let mut surface_area = 0;
        for voxel in &droplet {
            for (dx, dy, dz) in [(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)] {
                if !droplet.contains(&Pos::new(voxel.x + dx, voxel.y + dy, voxel.z + dz)) {
                    surface_area += 1;
                }
            }
        }
        surface_area
    }

    fn solve_part2(&self) -> usize {
        let (min, max) = self.voxels
            .iter()
            .fold(
                (Pos::new(i8::MAX, i8::MAX, i8::MAX), Pos::new(i8::MIN, i8::MIN, i8::MIN)),
                |(mut min, mut max), voxel| {
                    min.x = min.x.min(voxel.x - 1);
                    min.y = min.y.min(voxel.y - 1);
                    min.z = min.z.min(voxel.z - 1);
                    max.x = max.x.max(voxel.x + 1);
                    max.y = max.y.max(voxel.y + 1);
                    max.z = max.z.max(voxel.z + 1);
                    (min, max)
                }
            );
        let droplet: HashSet<Pos> = HashSet::from_iter(self.voxels.iter().map(|x| *x));
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut to_expand: Vec<Pos> = vec!(min);
        visited.insert(min);
        let mut surface_area = 0;
        while !to_expand.is_empty() {
            let pos = to_expand.pop().unwrap();
            for dir in [Pos::new(1,0,0), Pos::new(-1,0,0), Pos::new(0,1,0),
                Pos::new(0,-1,0), Pos::new(0,0,1), Pos::new(0,0,-1)]
            {
                let new_pos = pos + dir;
                if new_pos.x < min.x || new_pos.y < min.y || new_pos.z < min.z ||
                    new_pos.x > max.x || new_pos.y > max.y || new_pos.z > max.z
                {
                    continue;
                }
                if visited.contains(&new_pos) {
                    continue;
                }
                if droplet.contains(&new_pos) {
                    surface_area += 1;
                    continue;
                }
                visited.insert(new_pos);
                to_expand.push(new_pos);
            }
        }
        surface_area
    }

    fn print_solutions(&self, part1: usize, part2: usize) {
        println!("Lava droplet surface area: {part1}");
        println!("Lava droplet exterior surface area: {part2}");
    }
}
