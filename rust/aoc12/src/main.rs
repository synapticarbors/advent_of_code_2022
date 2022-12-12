use std::collections::HashMap;

use anyhow::Result;
use pathfinding::prelude::bfs;

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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

fn calc_cost(a: &Pos, b: &Pos, g: &[Vec<u8>]) -> i32 {
    let ha = g[a.0][a.1];
    let hb = g[b.0][b.1];

    if hb as i32 - ha as i32 <= 1 {
        0
    } else {
        1000
    }
}

fn parse_input(input: &[u8]) -> (Pos, Pos, Vec<Pos>, HashMap<Pos, Vec<Pos>>) {
    let mut target = Pos(0, 0);
    let mut start = Pos(0, 0);

    let mut low_points = Vec::new();

    // Parse Grid
    let g = input
        .split(|b| *b == b'\n')
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, b)| match b {
                    b'S' => {
                        start = Pos(i, j);
                        low_points.push(Pos(i, j));
                        0
                    }
                    b'E' => {
                        target = Pos(i, j);
                        b'z' - b'a'
                    }
                    b'a' => {
                        low_points.push(Pos(i, j));
                        0
                    }
                    x => x - b'a',
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Build neighbors map
    let nrows = g.len();
    let ncols = g[0].len();

    let mut neighbors = HashMap::with_capacity(nrows * ncols);

    for i in 0..nrows {
        for j in 0..ncols {
            let mut n = Vec::new();
            if i > 0 && calc_cost(&Pos(i, j), &Pos(i - 1, j), &g) == 0 {
                n.push(Pos(i - 1, j));
            }

            if i + 1 < nrows && calc_cost(&Pos(i, j), &Pos(i + 1, j), &g) == 0 {
                n.push(Pos(i + 1, j));
            }

            if j > 0 && calc_cost(&Pos(i, j), &Pos(i, j - 1), &g) == 0 {
                n.push(Pos(i, j - 1));
            }

            if j > 0 && calc_cost(&Pos(i, j), &Pos(i, j - 1), &g) == 0 {
                n.push(Pos(i, j - 1));
            }

            if j + 1 < ncols && calc_cost(&Pos(i, j), &Pos(i, j + 1), &g) == 0 {
                n.push(Pos(i, j + 1));
            }

            neighbors.insert(Pos(i, j), n);
        }
    }

    (start, target, low_points, neighbors)
}

pub fn solve_a() -> Result<usize> {
    let (start, target, _, neighbors) = parse_input(include_bytes!("../input"));

    let path = bfs(
        &start,
        |p| neighbors.get(p).unwrap().clone().into_iter(),
        |p| *p == target,
    );

    let path_length = path.expect("No path found").len() - 1;

    Ok(path_length)
}

pub fn solve_b() -> Result<usize> {
    let (_, target, low_points, neighbors) = parse_input(include_bytes!("../input"));

    let mut path_lengths = Vec::with_capacity(low_points.len());

    for sp in low_points {
        let path = bfs(
            &sp,
            |p| neighbors.get(p).unwrap().clone().into_iter(),
            |p| *p == target,
        );

        if let Some(x) = path {
            path_lengths.push(x.len() - 1);
        }
    }

    Ok(*path_lengths.iter().min().unwrap())
}
