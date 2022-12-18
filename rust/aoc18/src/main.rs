use std::collections::{HashSet, VecDeque};

use anyhow::Result;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Droplet {
    x: i8,
    y: i8,
    z: i8,
}

type Lava = HashSet<Droplet>;

impl Droplet {
    fn neighbors(&self) -> Vec<Droplet> {
        vec![
            Droplet {
                x: self.x - 1,
                ..*self
            },
            Droplet {
                x: self.x + 1,
                ..*self
            },
            Droplet {
                y: self.y - 1,
                ..*self
            },
            Droplet {
                y: self.y + 1,
                ..*self
            },
            Droplet {
                z: self.z - 1,
                ..*self
            },
            Droplet {
                z: self.z + 1,
                ..*self
            },
        ]
    }
}

fn parse_input(input: &str) -> Lava {
    input.lines().fold(HashSet::new(), |mut acc, line| {
        let mut it = line.split(',');
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();

        acc.insert(Droplet { x, y, z });
        acc
    })
}

fn get_bounds(lava: &HashSet<Droplet>) -> (Droplet, Droplet) {
    let mut xmin = i8::MAX;
    let mut xmax = i8::MIN;
    let mut ymin = i8::MAX;
    let mut ymax = i8::MIN;
    let mut zmin = i8::MAX;
    let mut zmax = i8::MIN;

    for d in lava {
        if d.x < xmin {
            xmin = d.x;
        }

        if d.x > xmax {
            xmax = d.x;
        }

        if d.y < ymin {
            ymin = d.y;
        }

        if d.y > ymax {
            ymax = d.y;
        }

        if d.z < zmin {
            zmin = d.z;
        }

        if d.z > zmax {
            zmax = d.z;
        }
    }

    (
        Droplet {
            x: xmin - 1,
            y: ymin - 1,
            z: zmin - 1,
        },
        Droplet {
            x: xmax + 1,
            y: ymax + 1,
            z: zmax + 1,
        },
    )
}

fn within_boundary(d: &Droplet, bmin: &Droplet, bmax: &Droplet) -> bool {
    d.x >= bmin.x
        && d.x <= bmax.x
        && d.y >= bmin.y
        && d.y <= bmax.y
        && d.z >= bmin.z
        && d.z <= bmax.z
}

fn get_surrounding_air(lava: &Lava, bounds_min: &Droplet, bounds_max: &Droplet) -> Lava {
    let mut air = HashSet::new();
    let mut seen = HashSet::new();
    let mut to_check = VecDeque::new();

    to_check.push_back(*bounds_min);
    to_check.push_back(*bounds_max);

    while !to_check.is_empty() {
        let x = to_check.pop_front().unwrap();
        if !seen.contains(&x) && !lava.contains(&x) {
            air.insert(x);

            for n in x.neighbors() {
                if within_boundary(&n, bounds_min, bounds_max) {
                    to_check.push_back(n);
                }
            }
        }

        seen.insert(x);
    }

    air
}

pub fn solve_a() -> Result<usize> {
    let lava = parse_input(include_str!("../input"));

    let x = lava
        .iter()
        .map(|d| d.neighbors().iter().filter(|n| !lava.contains(n)).count())
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let lava = parse_input(include_str!("../input"));

    let (bound_min, bound_max) = get_bounds(&lava);

    let surrounding_air = get_surrounding_air(&lava, &bound_min, &bound_max);

    let x = lava
        .iter()
        .map(|d| {
            d.neighbors()
                .iter()
                .filter(|n| !lava.contains(n) && surrounding_air.contains(n))
                .count()
        })
        .sum::<usize>();

    Ok(x)
}
