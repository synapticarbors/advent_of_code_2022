use std::collections::HashMap;

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

type Row = [bool; 7];
const MAX_H: usize = 6000;

#[derive(Debug, Clone, Copy)]
enum Shape {
    Hbar,
    Cross,
    FlipL,
    Vbar,
    Box,
}

impl Shape {
    fn height(&self) -> usize {
        match self {
            Shape::Hbar => 1,
            Shape::Cross => 3,
            Shape::FlipL => 3,
            Shape::Vbar => 4,
            Shape::Box => 2,
        }
    }

    fn side_collision(&self, m: &Move, ph: usize, px: usize, chamber: &[Row]) -> bool {
        match (self, m) {
            (_, Move::Left) if px == 0 => true,
            (Shape::Hbar, Move::Left) => chamber[ph][px - 1],
            (Shape::Hbar, Move::Right) => px == 3 || chamber[ph][px + 4],
            (Shape::Box, Move::Left) => chamber[ph][px - 1] || chamber[ph + 1][px - 1],
            (Shape::Box, Move::Right) => px == 5 || chamber[ph][px + 2] || chamber[ph + 1][px + 2],
            (Shape::Vbar, Move::Left) => {
                chamber[ph][px - 1]
                    || chamber[ph + 1][px - 1]
                    || chamber[ph + 2][px - 1]
                    || chamber[ph + 3][px - 1]
            }
            (Shape::Vbar, Move::Right) => {
                px == 6
                    || chamber[ph][px + 1]
                    || chamber[ph + 1][px + 1]
                    || chamber[ph + 2][px + 1]
                    || chamber[ph + 3][px + 1]
            }
            (Shape::FlipL, Move::Left) => {
                chamber[ph][px - 1] || chamber[ph + 1][px + 1] || chamber[ph + 2][px + 1]
            }
            (Shape::FlipL, Move::Right) => {
                px == 4 || chamber[ph][px + 3] || chamber[ph + 1][px + 3] || chamber[ph + 2][px + 3]
            }
            (Shape::Cross, Move::Left) => {
                chamber[ph][px] || chamber[ph + 1][px - 1] || chamber[ph + 2][px]
            }
            (Shape::Cross, Move::Right) => {
                px == 4 || chamber[ph][px + 2] || chamber[ph + 1][px + 3] || chamber[ph + 2][px + 2]
            }
        }
    }

    fn bottom_collision(&self, ph: usize, px: usize, chamber: &[Row]) -> bool {
        match self {
            Shape::Hbar => {
                ph == 0
                    || chamber[ph - 1][px]
                    || chamber[ph - 1][px + 1]
                    || chamber[ph - 1][px + 2]
                    || chamber[ph - 1][px + 3]
            }
            Shape::Vbar => ph == 0 || chamber[ph - 1][px],
            Shape::FlipL => {
                ph == 0 || chamber[ph - 1][px] || chamber[ph - 1][px + 1] || chamber[ph - 1][px + 2]
            }
            Shape::Box => ph == 0 || chamber[ph - 1][px] || chamber[ph - 1][px + 1],
            Shape::Cross => {
                ph == 0 || chamber[ph - 1][px + 1] || chamber[ph][px] || chamber[ph][px + 2]
            }
        }
    }
}

const SHAPES: [Shape; 5] = [
    Shape::Hbar,
    Shape::Cross,
    Shape::FlipL,
    Shape::Vbar,
    Shape::Box,
];

#[derive(Debug, Clone, Copy)]
enum Move {
    Left,
    Right,
}

fn parse_input(input: &[u8]) -> Vec<Move> {
    input
        .iter()
        .filter(|b| **b != b'\n')
        .map(|b| match b {
            b'<' => Move::Left,
            b'>' => Move::Right,
            _ => unreachable!(),
        })
        .collect()
}

fn mark_occupied(ph: usize, px: usize, s: &Shape, chamber: &mut [Row]) {
    match s {
        Shape::Hbar => {
            chamber[ph][px] = true;
            chamber[ph][px + 1] = true;
            chamber[ph][px + 2] = true;
            chamber[ph][px + 3] = true;
        }
        Shape::Vbar => {
            chamber[ph][px] = true;
            chamber[ph + 1][px] = true;
            chamber[ph + 2][px] = true;
            chamber[ph + 3][px] = true;
        }
        Shape::Box => {
            chamber[ph][px] = true;
            chamber[ph][px + 1] = true;
            chamber[ph + 1][px] = true;
            chamber[ph + 1][px + 1] = true;
        }
        Shape::FlipL => {
            chamber[ph][px] = true;
            chamber[ph][px + 1] = true;
            chamber[ph][px + 2] = true;
            chamber[ph + 1][px + 2] = true;
            chamber[ph + 2][px + 2] = true;
        }
        Shape::Cross => {
            chamber[ph][px + 1] = true;
            chamber[ph + 1][px] = true;
            chamber[ph + 1][px + 1] = true;
            chamber[ph + 1][px + 2] = true;
            chamber[ph + 2][px + 1] = true;
        }
    }
}

#[allow(dead_code)]
fn draw_chamber(chamber: &[Row], s: &Shape, ph: usize, px: usize, max_height: usize) {
    println!();
    println!();
    println!("{:?}: {} {}", s, ph, px);
    for h in (0..max_height).rev() {
        println!(
            "{}",
            chamber[h]
                .iter()
                .map(|x| match x {
                    true => '#',
                    false => '.',
                })
                .collect::<String>()
        );
    }
}

#[allow(dead_code)]
fn count_occupied_rows(chamber: &[Row]) -> usize {
    chamber
        .iter()
        .filter(|r| r.iter().map(|e| *e as u8).sum::<u8>() > 0)
        .count()
}

fn drop_block(
    chamber: &mut [Row],
    blocks: &mut impl Iterator<Item = (usize, Shape)>,
    moves: &mut impl Iterator<Item = (usize, Move)>,
    maxh: usize,
) -> (usize, usize, usize) {
    let mut ph = maxh + 3;
    let mut px = 2;

    let (si, s) = blocks.next().unwrap();

    let mi = loop {
        let (mi, m) = moves.next().unwrap();
        match m {
            Move::Left => {
                if !s.side_collision(&m, ph, px, chamber) {
                    px -= 1;
                }
            }
            Move::Right => {
                if !s.side_collision(&m, ph, px, chamber) {
                    px += 1;
                }
            }
        }

        if !s.bottom_collision(ph, px, chamber) {
            ph -= 1;
        } else {
            mark_occupied(ph, px, &s, chamber);
            break mi;
        }
    };

    (si, mi, std::cmp::max(maxh, ph + s.height()))
}

fn simulate(
    target_rocks: usize,
    mut blocks: &mut impl Iterator<Item = (usize, Shape)>,
    mut moves: &mut impl Iterator<Item = (usize, Move)>,
) -> usize {
    let mut chamber = vec![Row::default(); MAX_H];
    let mut maxh = 0;

    for _ in 0..target_rocks {
        let (_, _, height) = drop_block(&mut chamber, &mut blocks, &mut moves, maxh);
        maxh = height;
    }

    maxh
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct State {
    ceiling: [usize; 7],
    si: usize,
    mi: usize,
}

fn get_chamber_ceiling(chamber: &[Row], maxh: usize) -> [usize; 7] {
    let mut cc = [0; 7];
    for (i, ccx) in cc.iter_mut().enumerate() {
        // for i in 0..7 {
        let mut testh = maxh;
        let mut cnt = 0;
        while !chamber[testh][i] {
            if testh == 0 {
                break;
            }
            testh -= 1;
            cnt += 1;
        }
        *ccx = cnt;
    }

    cc
}

fn simulate_cycle(
    target_rocks: usize,
    mut blocks: &mut impl Iterator<Item = (usize, Shape)>,
    mut moves: &mut impl Iterator<Item = (usize, Move)>,
) -> usize {
    let mut chamber = vec![Row::default(); MAX_H];
    let mut maxh = 0;

    let mut cache: HashMap<State, (usize, usize)> = HashMap::new();

    for nrocks in 1..=target_rocks {
        let (si, mi, height) = drop_block(&mut chamber, &mut blocks, &mut moves, maxh);

        maxh = height;

        let ceiling = get_chamber_ceiling(&chamber, maxh);
        let state = State { ceiling, si, mi };

        if cache.contains_key(&state) {
            let (nrocks_prev, maxh_prev) = cache.get(&state).unwrap();
            let nrocks_cycle = nrocks - nrocks_prev;

            let delta_height = maxh - maxh_prev;
            let repeats = (target_rocks - nrocks) / nrocks_cycle;
            let addtl_drops = (target_rocks - nrocks) - (repeats * nrocks_cycle);
            let mut total_height = delta_height * repeats;

            for _ in 0..addtl_drops {
                let (_, _, height) = drop_block(&mut chamber, &mut blocks, &mut moves, maxh);
                maxh = height;
            }

            total_height += maxh;
            return total_height;
        } else {
            cache.insert(state, (nrocks, maxh));
        }
    }

    maxh
}

pub fn solve_a() -> Result<usize> {
    let moves = parse_input(include_bytes!("../input"));

    let mut blocks = SHAPES.iter().copied().enumerate().cycle();
    let mut moves = moves.iter().copied().enumerate().cycle();

    let x = simulate(2022, &mut blocks, &mut moves);

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let moves = parse_input(include_bytes!("../input"));

    let mut blocks = SHAPES.iter().copied().enumerate().cycle();
    let mut moves = moves.iter().copied().enumerate().cycle();

    let x = simulate_cycle(1_000_000_000_000, &mut blocks, &mut moves);

    Ok(x)
}
