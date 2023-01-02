use std::collections::HashSet;
use std::hash::Hash;

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

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
struct NeighOccupied {
    n: bool,
    ne: bool,
    e: bool,
    se: bool,
    s: bool,
    sw: bool,
    w: bool,
    nw: bool,
}

impl NeighOccupied {
    fn all_empty(&self) -> bool {
        !self.n && !self.ne && !self.e && !self.se && !self.s && !self.sw && !self.w && !self.nw
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Elf(isize, isize);

impl Elf {
    fn neigh_occupied(&self, h: &HashSet<&Elf>) -> NeighOccupied {
        NeighOccupied {
            n: h.contains(&Elf(self.0 - 1, self.1)),
            ne: h.contains(&Elf(self.0 - 1, self.1 + 1)),
            e: h.contains(&Elf(self.0, self.1 + 1)),
            se: h.contains(&Elf(self.0 + 1, self.1 + 1)),
            s: h.contains(&Elf(self.0 + 1, self.1)),
            sw: h.contains(&Elf(self.0 + 1, self.1 - 1)),
            w: h.contains(&Elf(self.0, self.1 - 1)),
            nw: h.contains(&Elf(self.0 - 1, self.1 - 1)),
        }
    }
    fn check(&self, d: &Dir, noc: &NeighOccupied) -> bool {
        match d {
            Dir::North => noc.n || noc.ne || noc.nw,
            Dir::South => noc.s || noc.se || noc.sw,
            Dir::East => noc.e || noc.se || noc.ne,
            Dir::West => noc.w || noc.sw || noc.nw,
        }
    }

    fn propose_move(&self, d: &Dir) -> Elf {
        match d {
            Dir::North => Elf(self.0 - 1, self.1),
            Dir::South => Elf(self.0 + 1, self.1),
            Dir::East => Elf(self.0, self.1 + 1),
            Dir::West => Elf(self.0, self.1 - 1),
        }
    }
}

type ProposeOrder = [Dir; 4];

fn parse_input(input: &[u8]) -> Vec<Elf> {
    let mut pos = Vec::new();

    input
        .split(|b| *b == b'\n')
        .enumerate()
        .for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, x)| {
                if *x == b'#' {
                    pos.push(Elf(i as isize, j as isize));
                }
            });
        });

    pos
}

fn run_round(pos: &mut Vec<Elf>, propose: &ProposeOrder) {
    let current: HashSet<&Elf> = HashSet::from_iter(pos.iter());
    let mut candidate_pos = vec![None; pos.len()];

    for (i, epos) in pos.iter().enumerate() {
        let noc = epos.neigh_occupied(&current);
        if noc.all_empty() {
            continue;
        }
        for d in propose.iter() {
            if !epos.check(d, &noc) {
                candidate_pos[i] = Some(epos.propose_move(d));
                break;
            }
        }
    }

    for i in 0..candidate_pos.len() {
        if candidate_pos[i].is_none() {
            continue;
        }
        let x = candidate_pos[i].unwrap();

        for j in i + 1..candidate_pos.len() {
            match candidate_pos[j] {
                None => {
                    if x == pos[j] {
                        candidate_pos[i] = None;
                        break;
                    }
                }
                Some(y) => {
                    if x == y {
                        candidate_pos[i] = None;
                        candidate_pos[j] = None;
                        break;
                    }
                }
            }
        }
    }

    for i in 0..candidate_pos.len() {
        if let Some(x) = candidate_pos[i] {
            pos[i] = x;
        }
    }
}

fn calc_bounds(pos: &[Elf]) -> (isize, isize, isize, isize) {
    let mut xmin = isize::MAX;
    let mut xmax = isize::MIN;
    let mut ymin = isize::MAX;
    let mut ymax = isize::MIN;

    for p in pos.iter() {
        if p.0 < xmin {
            xmin = p.0;
        }

        if p.0 > xmax {
            xmax = p.0;
        }

        if p.1 < ymin {
            ymin = p.1;
        }

        if p.1 > ymax {
            ymax = p.1;
        }
    }

    (xmin, xmax, ymin, ymax)
}

fn calc_n_empty(pos: &[Elf]) -> isize {
    let (xmin, xmax, ymin, ymax) = calc_bounds(pos);
    (xmax - xmin + 1) * (ymax - ymin + 1) - pos.len() as isize
}

#[allow(dead_code)]
fn draw(pos: &[Elf]) {
    let (xmin, xmax, ymin, ymax) = calc_bounds(pos);

    let current: HashSet<&Elf> = HashSet::from_iter(pos.iter());

    for i in xmin - 1..=xmax + 1 {
        println!(
            "{}",
            (ymin - 1..=ymax + 1)
                .map(|j| if current.contains(&Elf(i, j)) {
                    '#'
                } else {
                    '.'
                })
                .collect::<String>()
        );
    }
}

pub fn solve_a() -> Result<isize> {
    let mut pos = parse_input(include_bytes!("../input"));
    let mut propose = [Dir::North, Dir::South, Dir::West, Dir::East];

    let nrounds = 10;

    for _ in 0..nrounds {
        run_round(&mut pos, &propose);
        propose.rotate_left(1);
        // draw(&pos);
    }

    let n_empty = calc_n_empty(&pos);

    Ok(n_empty)
}

pub fn solve_b() -> Result<usize> {
    let mut pos = parse_input(include_bytes!("../input"));
    let mut propose = [Dir::North, Dir::South, Dir::West, Dir::East];
    let mut nloops = 0;

    loop {
        nloops += 1;
        let pos_init = pos.clone();
        run_round(&mut pos, &propose);

        if pos_init == pos {
            break;
        }

        propose.rotate_left(1);
    }

    Ok(nloops)
}
