use std::collections::HashSet;

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

type Motion = ((i32, i32), i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos(i32, i32);

fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(d, n)| {
            let x = n.parse::<i32>().unwrap();
            match d {
                "L" => ((-1, 0), x),
                "R" => ((1, 0), x),
                "U" => ((0, 1), x),
                "D" => ((0, -1), x),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn is_touching(h: &Pos, t: &Pos) -> bool {
    h.0.abs_diff(t.0) <= 1 && h.1.abs_diff(t.1) <= 1
}

pub fn solve_a() -> Result<usize> {
    let visited = parse_input(include_str!("../input"))
        .iter()
        .fold(
            (Pos(0, 0), Pos(0, 0), HashSet::from([Pos(0, 0)])),
            |(mut h, mut t, mut visited), (dir, mag)| {
                let hfinal = Pos(h.0 + dir.0 * mag, h.1 + dir.1 * mag);

                if hfinal.0.abs_diff(t.0) > 1 || hfinal.1.abs_diff(t.1) > 1 {
                    for _ in 0..*mag {
                        h.0 += dir.0;
                        h.1 += dir.1;

                        if is_touching(&h, &t) {
                            continue;
                        }

                        let diff = (h.0 - t.0, h.1 - t.1);

                        t.0 += diff.0.signum();
                        t.1 += diff.1.signum();

                        visited.insert(t);
                    }
                }

                (hfinal, t, visited)
            },
        )
        .2;

    let x = visited.len();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let nknots = 10;

    let visited = parse_input(include_str!("../input"))
        .iter()
        .fold(
            (vec![Pos(0, 0); nknots], HashSet::from([Pos(0, 0)])),
            |(mut knots, mut visited), (dir, mag)| {
                for _ in 0..*mag {
                    knots[0].0 += dir.0;
                    knots[0].1 += dir.1;

                    for ki in 1..nknots {
                        let (h, t) = knots.split_at_mut(ki);
                        let (h, t) = (h[ki - 1], &mut t[0]);

                        if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                            if is_touching(&h, t) {
                                continue;
                            }

                            let diff = (h.0 - t.0, h.1 - t.1);

                            t.0 += diff.0.signum();
                            t.1 += diff.1.signum();

                            if ki == 9 {
                                visited.insert(*t);
                            }
                        } else {
                            break;
                        }
                    }
                }
                (knots, visited)
            },
        )
        .1;

    let x = visited.len();

    Ok(x)
}
