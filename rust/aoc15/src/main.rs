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
struct Pos(i32, i32);

#[derive(Debug)]
enum Span {
    Pair(i32, i32),
    Point(i32),
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }

    fn solve_bounds_at_y(&self, radius: i32, y: i32) -> Option<Span> {
        // r = |bx - X| + |by - y|
        // Solving for X

        let dy = (self.1 - y).abs();

        if radius == dy {
            Some(Span::Point(self.0))
        } else if radius > dy {
            let a = self.0 + dy - radius;
            let b = self.0 - dy + radius;

            if a < b {
                Some(Span::Pair(a, b))
            } else {
                Some(Span::Pair(b, a))
            }
        } else {
            None
        }
    }
}

fn parse_line(line: &str) -> (Pos, Pos) {
    let s = &line[12..];
    let (sx, s) = s.split_once(',').unwrap();
    let s = &s[3..];
    let (sy, s) = s.split_once(':').unwrap();
    // let s = s.strip_prefix(" closest beacon is at x=").unwrap();
    let s = &s[24..];
    let (bx, s) = s.split_once(',').unwrap();
    let (_, by) = s.split_once('=').unwrap();

    (
        Pos(sx.parse().unwrap(), sy.parse().unwrap()),
        Pos(bx.parse().unwrap(), by.parse().unwrap()),
    )
}

fn union_ranges(ranges: &mut [(i32, i32)]) -> Vec<(i32, i32)> {
    ranges.sort_unstable();

    let mut u = vec![ranges[0]];

    for (start, end) in ranges.iter().skip(1) {
        let x = u.last().unwrap();
        if x.1 >= *start - 1 {
            let sz = u.len();
            u[sz - 1].1 = std::cmp::max(x.1, *end);
        } else {
            u.push((*start, *end));
        }
    }

    u
}

fn scan_row(pairs: &[(Pos, Pos)], xmin: i32, xmax: i32, target_row: i32) -> Option<Pos> {
    let mut ranges = Vec::with_capacity(25);

    for (sensor, beacon) in pairs {
        let radius = sensor.manhattan_distance(beacon);

        if beacon.1 == target_row && beacon.0 >= xmin && beacon.0 <= xmax {
            ranges.push((beacon.0, beacon.0));
        }

        if let Some(s) = sensor.solve_bounds_at_y(radius, target_row) {
            match s {
                Span::Point(x) => {
                    if x >= xmin && x <= xmax {
                        ranges.push((x, x));
                    }
                }
                Span::Pair(xstart, xend) => {
                    ranges.push((std::cmp::max(xstart, xmin), std::cmp::min(xend, xmax)));
                }
            }
        }
    }

    let ranges = union_ranges(&mut ranges);
    let sz = ranges.len();
    assert!(sz == 1 || sz == 2);

    // println!("{}: {:?}", target_row, ranges);

    if ranges.len() == 1 {
        if ranges[0].0 == xmin && ranges[0].1 == xmax {
            None
        } else if ranges[0].0 == xmin + 1 {
            Some(Pos(xmin, target_row))
        } else if ranges[0].1 == xmax - 1 {
            Some(Pos(xmax, target_row))
        } else {
            unreachable!();
        }
    } else {
        assert!(ranges[0].1 + 2 == ranges[1].0);
        Some(Pos(ranges[0].1 + 1, target_row))
    }
}

pub fn solve_a() -> Result<usize> {
    let target_row = 2_000_000;

    let mut min_col = i32::MAX;
    let mut max_col = i32::MIN;
    let mut points = Vec::new();

    let pairs = include_str!("../input")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    for (sensor, beacon) in &pairs {
        let radius = sensor.manhattan_distance(beacon);

        if let Some(s) = sensor.solve_bounds_at_y(radius, target_row) {
            match s {
                Span::Point(x) => {
                    points.push(x);
                }
                Span::Pair(xstart, xend) => {
                    if xstart < min_col {
                        min_col = xstart;
                    }

                    if xend > max_col {
                        max_col = xend;
                    }
                }
            }
        }
    }

    let span = max_col - min_col + 1;

    let mut beacon_in_target = pairs
        .iter()
        .map(|(_, b)| b)
        .filter(|b| b.1 == target_row)
        .map(|b| b.0)
        .collect::<Vec<_>>();

    beacon_in_target.dedup();

    let outside_points = points
        .iter()
        .filter(|x| (**x > max_col || **x < min_col) && !beacon_in_target.contains(x));

    let x = span as usize + outside_points.count()
        - beacon_in_target
            .iter()
            .filter(|x| **x >= min_col && **x <= max_col)
            .count();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    const SZ: usize = 4_000_000;

    let pairs = include_str!("../input")
        .lines()
        .map(parse_line)
        .collect::<Vec<_>>();

    if let Some(p) = (0..=SZ)
        .into_par_iter()
        .find_map_any(|tr| scan_row(&pairs, 0, SZ as i32, tr as i32))
    {
        return Ok(4000000 * p.0 as usize + p.1 as usize);
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_union() {
        let mut ranges = vec![(7, 10), (11, 13), (11, 15), (14, 20), (23, 39)];
        let x = union_ranges(&mut ranges);
        assert_eq!(x, vec![(7, 20), (23, 39)]);

        let mut ranges = vec![(7, 10), (11, 11), (6, 6)];
        let x = union_ranges(&mut ranges);
        assert_eq!(x, vec![(6, 11)]);
    }
}
