use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;
use rayon::prelude::*;

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: {}", soln_b);

    Ok(())
}

#[derive(Debug)]
enum Material {
    Geode,
    Obsidian,
    Clay,
    Ore,
}

const MATERIALS: [Material; 4] = [
    Material::Geode,
    Material::Obsidian,
    Material::Clay,
    Material::Ore,
];

#[derive(Debug, Eq, PartialEq)]
struct RobotCost(u32, u32, u32);

#[derive(Debug)]
struct Blueprint {
    cost: [RobotCost; 4],
    max_cost_ore: u32,
    max_cost_clay: u32,
    max_cost_obsidian: u32,
}

impl Blueprint {
    fn from_str(s: &str) -> Blueprint {
        let x = s
            .split_ascii_whitespace()
            .skip(1)
            .filter_map(|w| w.parse::<u32>().ok())
            .collect::<Vec<_>>();

        let cost = [
            RobotCost(x[0], 0, 0),
            RobotCost(x[1], 0, 0),
            RobotCost(x[2], x[3], 0),
            RobotCost(x[4], 0, x[5]),
        ];

        let max_cost_ore = cost.iter().map(|c| c.0).max().unwrap();
        let max_cost_clay = cost.iter().map(|c| c.1).max().unwrap();
        let max_cost_obsidian = cost.iter().map(|c| c.2).max().unwrap();

        Blueprint {
            cost,
            max_cost_ore,
            max_cost_clay,
            max_cost_obsidian,
        }
    }

    fn build_cost(&self, robot: &Material) -> &RobotCost {
        match robot {
            Material::Ore => &self.cost[0],
            Material::Clay => &self.cost[1],
            Material::Obsidian => &self.cost[2],
            Material::Geode => &self.cost[3],
        }
    }

    fn can_build(&self, robot: &Material, state: &State) -> bool {
        let c = self.build_cost(robot);
        state.n_ores >= c.0 && state.n_clay >= c.1 && state.n_obsidian >= c.2
    }

    fn heuristic(&self, robot: &Material, state: &State) -> bool {
        let h1 = match robot {
            Material::Ore => state.ore_robots <= self.max_cost_ore,
            Material::Clay => state.clay_robots <= self.max_cost_clay,
            Material::Obsidian => state.obsidian_robots <= self.max_cost_obsidian,
            _ => unreachable!(),
        };

        h1
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    n_ores: u32,
    n_clay: u32,
    n_obsidian: u32,
    n_geodes: u32,
}

impl State {
    fn new() -> State {
        State {
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn collect_material(&mut self) {
        self.n_ores += self.ore_robots;
        self.n_clay += self.clay_robots;
        self.n_obsidian += self.obsidian_robots;
        self.n_geodes += self.geode_robots;
    }

    fn build(&mut self, robot: &Material, bp: &Blueprint) {
        self.collect_material();

        let c = bp.build_cost(robot);
        self.n_ores -= c.0;
        self.n_clay -= c.1;
        self.n_obsidian -= c.2;

        match robot {
            Material::Ore => self.ore_robots += 1,
            Material::Clay => self.clay_robots += 1,
            Material::Obsidian => self.obsidian_robots += 1,
            Material::Geode => self.geode_robots += 1,
        }
    }

    fn stabilize(&mut self, bp: &Blueprint) {
        if self.n_ores > bp.max_cost_ore && self.ore_robots > bp.max_cost_ore {
            self.n_ores = bp.max_cost_ore;
        }

        if self.n_clay > bp.max_cost_clay && self.clay_robots > bp.max_cost_clay {
            self.n_clay = bp.max_cost_clay;
        }

        if self.n_obsidian > bp.max_cost_obsidian && self.obsidian_robots > bp.max_cost_obsidian {
            self.n_obsidian = bp.max_cost_obsidian;
        }
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|line| Blueprint::from_str(line))
        .collect()
}

fn find_max_geodes(bp: &Blueprint, time: u32) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((State::new(), 0));

    let mut cache: HashMap<u32, u32> = (0..=time).map(|x| (x as u32, 0)).collect();
    let mut seen = HashSet::new();

    while let Some((state, t)) = queue.pop_front() {
        let &prev_max = cache.get(&t).unwrap();

        if state.n_geodes + 2 < prev_max {
            continue;
        }

        if seen.contains(&state) {
            continue;
        }

        seen.insert(state);

        cache.insert(t, std::cmp::max(prev_max, state.n_geodes));

        if t == time {
            continue;
        }

        if bp.can_build(&Material::Geode, &state) {
            let mut new_state = state;
            new_state.build(&Material::Geode, bp);
            new_state.stabilize(bp);

            queue.push_back((new_state, t + 1));
            continue;
        }

        let mut new_state = state;
        new_state.collect_material();
        queue.push_back((new_state, t + 1));

        for robot in MATERIALS.iter().skip(1) {
            if bp.can_build(robot, &state) && bp.heuristic(robot, &state) {
                let mut new_state = state;
                new_state.build(robot, bp);
                new_state.stabilize(bp);

                queue.push_back((new_state, t + 1));
            }
        }
    }

    *cache.get(&time).unwrap()
}

pub fn solve_a() -> Result<u32> {
    let blueprints = parse_input(include_str!("../input"));

    let x = blueprints
        .par_iter()
        .enumerate()
        .map(|(i, bp)| {
            let max_geodes = find_max_geodes(bp, 24);
            max_geodes * (i as u32 + 1)
        })
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<u32> {
    let blueprints = parse_input(include_str!("../input"));

    let x = blueprints
        .par_iter()
        .take(3)
        .map(|bp| {
            let max_geodes = find_max_geodes(bp, 32);
            max_geodes
        })
        .product();

    Ok(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost() {
        let blueprints = parse_input("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.\n");

        assert_eq!(blueprints.len(), 1);
        let bp = &blueprints[0];

        let c = bp.build_cost(&Material::Geode);
        assert_eq!(*c, RobotCost(3, 0, 8));

        let c = bp.build_cost(&Material::Ore);
        assert_eq!(*c, RobotCost(3, 0, 0));

        let c = bp.build_cost(&Material::Clay);
        assert_eq!(*c, RobotCost(4, 0, 0));

        let c = bp.build_cost(&Material::Obsidian);
        assert_eq!(*c, RobotCost(4, 18, 0));
    }

    #[test]
    fn test_build() {
        let blueprints = parse_input("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 3 ore and 8 obsidian.\n");

        let bp = &blueprints[0];

        let mut state = State::new();
        state.obsidian_robots = 1;
        state.clay_robots = 1;
        state.n_ores = 100;
        state.n_obsidian = 100;
        state.build(&Material::Geode, bp);

        assert_eq!(state.geode_robots, 1);
        assert_eq!(state.n_clay, 1);
        assert_eq!(state.n_ores, 98);
        assert_eq!(state.n_obsidian, 93);
    }
}
