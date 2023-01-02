use std::collections::HashMap;

use anyhow::Result;
use pathfinding::prelude::dijkstra;

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

const UP: u8 = 0b00000001;
const DOWN: u8 = 0b00000010;
const LEFT: u8 = 0b00000100;
const RIGHT: u8 = 0b00001000;
const WALL: u8 = 0b00010000;

type State = Vec<Vec<u8>>;

#[allow(dead_code)]
fn draw_state(s: &State) -> String {
    s.iter()
        .map(|row| {
            let x: String = row
                .iter()
                .map(|x| match x {
                    &WALL => '#',
                    &UP => '^',
                    &DOWN => 'v',
                    &LEFT => '<',
                    &RIGHT => '>',
                    0 => '.',
                    _ => 'X',
                })
                .collect();
            format!("{}\n", x)
        })
        .collect()
}

fn parse_input(input: &[u8]) -> State {
    input
        .split(|b| *b == b'\n')
        .map(|line| {
            line.iter()
                .map(|b| match b {
                    b'#' => WALL,
                    b'^' => UP,
                    b'v' => DOWN,
                    b'<' => LEFT,
                    b'>' => RIGHT,
                    b'.' => 0u8,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn next_state(s: &State) -> State {
    let nrows = s.len();
    let ncols = s[0].len();

    let col_edge = ncols - 2;
    let row_edge = nrows - 2;

    let mut ns = vec![vec![0; ncols]; nrows];

    for i in 0..nrows {
        for j in 0..ncols {
            if s[i][j] & WALL != 0 {
                ns[i][j] = WALL;
                continue;
            }

            if s[i][j] & UP != 0 {
                let k = if i == 1 { row_edge } else { i - 1 };
                ns[k][j] |= UP;
            }

            if s[i][j] & DOWN != 0 {
                let k = if i == row_edge { 1 } else { i + 1 };
                ns[k][j] |= DOWN;
            }

            if s[i][j] & LEFT != 0 {
                let k = if j == 1 { col_edge } else { j - 1 };
                ns[i][k] |= LEFT;
            }

            if s[i][j] & RIGHT != 0 {
                let k = if j == col_edge { 1 } else { j + 1 };
                ns[i][k] |= RIGHT;
            }
        }
    }

    ns
}

fn build_states(init_state: &State) -> (Vec<State>, usize) {
    let mut cache: HashMap<State, usize> = HashMap::new();
    let mut states = vec![init_state.clone()];
    let mut state = init_state.to_vec();

    cache.insert(init_state.to_vec(), 0);

    let cycle_start_idx = loop {
        let ns = next_state(&state);
        let clen = cache.len();

        let idx = *cache.entry(ns.clone()).or_insert(clen);
        if idx != clen {
            break idx;
        }

        states.push(ns.clone());

        state = ns.clone();
    };

    (states, cycle_start_idx)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos {
    x: usize,
    y: usize,
    t: usize,
}

impl Pos {
    fn successors(&self, states: &[State], cycle_start_idx: usize) -> Vec<(Pos, usize)> {
        let x = self.x;
        let y = self.y;

        let state_idx = if self.t < cycle_start_idx {
            self.t + 1
        } else {
            cycle_start_idx + (self.t + 1 - cycle_start_idx) % (states.len() - cycle_start_idx)
        };

        let s = &states[state_idx];
        let tnext = self.t + 1;

        let nrows = s.len();
        let ncols = s[0].len();

        let mut candidates = Vec::with_capacity(5);
        candidates.push(Pos { t: tnext, ..*self });

        if x < nrows - 1 {
            candidates.push(Pos {
                x: x + 1,
                t: tnext,
                ..*self
            });
        }

        if x > 0 {
            candidates.push(Pos {
                x: x - 1,
                t: tnext,
                ..*self
            });
        }

        if y < ncols - 1 {
            candidates.push(Pos {
                y: y + 1,
                t: tnext,
                ..*self
            });
        }

        if y > 0 {
            candidates.push(Pos {
                y: y - 1,
                t: tnext,
                ..*self
            });
        }

        candidates
            .into_iter()
            .filter(|p| s[p.x][p.y] == 0)
            .map(|p| (p, 1))
            .collect()
    }
}

fn solve_path_len(
    start: (usize, usize),
    end: (usize, usize),
    init_time: usize,
    cycle_start_idx: usize,
    states: &[State],
) -> usize {
    let path = dijkstra(
        &Pos {
            x: start.0,
            y: start.1,
            t: init_time,
        },
        |p| p.successors(states, cycle_start_idx),
        |p| p.x == end.0 && p.y == end.1,
    );

    path.unwrap().1
}

pub fn solve_a() -> Result<usize> {
    let state = parse_input(include_bytes!("../input"));
    let (states, cycle_start_idx) = build_states(&state);

    let nrows = state.len();
    let ncols = state[0].len();

    let goal = (nrows - 1, ncols - 2);

    let path_len = solve_path_len((0, 1), goal, 0, cycle_start_idx, &states);

    Ok(path_len)
}

pub fn solve_b() -> Result<usize> {
    let state = parse_input(include_bytes!("../input"));
    let (states, cycle_start_idx) = build_states(&state);

    let nrows = state.len();
    let ncols = state[0].len();

    let goal = (nrows - 1, ncols - 2);

    let path_len1 = solve_path_len((0, 1), goal, 0, cycle_start_idx, &states);
    let path_len2 = solve_path_len(goal, (0, 1), path_len1, cycle_start_idx, &states);
    let path_len3 = solve_path_len(
        (0, 1),
        goal,
        path_len1 + path_len2,
        cycle_start_idx,
        &states,
    );

    Ok(path_len1 + path_len2 + path_len3)
}
