use anyhow::Result;
use itertools::Itertools;

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
enum Direction {
    Right,
    Left,
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    Turn(Direction),
}

#[derive(Debug, PartialEq, Clone)]
enum GridPos {
    Void,
    Wall,
    Open,
}

#[derive(Debug, Default)]
struct Walker {
    fx: i8,
    fy: i8,
    px: usize,
    py: usize,
}

impl Walker {
    fn new(grid: &Grid) -> Walker {
        Walker {
            fx: 0,
            fy: 1,
            px: 0,
            py: grid[0].iter().position(|gp| *gp != GridPos::Void).unwrap(),
        }
    }
    fn turn(&mut self, d: &Direction) {
        match d {
            Direction::Left => {
                std::mem::swap(&mut self.fx, &mut self.fy);
                self.fx *= -1;
            }
            Direction::Right => {
                std::mem::swap(&mut self.fx, &mut self.fy);
                self.fy *= -1;
            }
        }
    }

    fn wrap(&self, grid: &Grid) -> (usize, usize) {
        match (self.fx, self.fy) {
            (0, 1) => (
                self.px,
                grid[self.px]
                    .iter()
                    .position(|gp| *gp != GridPos::Void)
                    .unwrap(),
            ),
            (0, -1) => (
                self.px,
                grid[self.px]
                    .iter()
                    .rposition(|gp| *gp != GridPos::Void)
                    .unwrap(),
            ),
            (1, 0) => (
                grid.iter()
                    .map(|row| &row[self.py])
                    .position(|gp| *gp != GridPos::Void)
                    .unwrap(),
                self.py,
            ),
            (-1, 0) => (
                grid.iter()
                    .map(|row| &row[self.py])
                    .rposition(|gp| *gp != GridPos::Void)
                    .unwrap(),
                self.py,
            ),
            _ => unreachable!(),
        }
    }
}

type Grid = Vec<Vec<GridPos>>;

fn parse_input(input: &str) -> (Grid, Vec<Instruction>) {
    let (grid_part, inst_part) = input.split_once("\n\n").unwrap();

    let mut grid = grid_part.lines().fold(Grid::new(), |mut acc, line| {
        let row = line
            .as_bytes()
            .iter()
            .map(|b| match b {
                b' ' => GridPos::Void,
                b'#' => GridPos::Wall,
                b'.' => GridPos::Open,
                _ => unreachable!(),
            })
            .collect::<Vec<GridPos>>();
        acc.push(row);

        acc
    });

    // Ensure every row is the correct length
    let row_max_len = grid.iter().map(|row| row.len()).max().unwrap();
    for row in grid.iter_mut() {
        if row.len() < row_max_len {
            row.extend(vec![GridPos::Void; row_max_len - row.len()]);
        }
    }

    let nums = inst_part
        .clone()
        .split(char::is_uppercase)
        .map(|x| Instruction::Move(x.parse().unwrap()));

    let turns = inst_part
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .map(|x| match x {
            'R' => Instruction::Turn(Direction::Right),
            'L' => Instruction::Turn(Direction::Left),
            _ => unreachable!(),
        });

    let inst = nums
        .into_iter()
        .interleave(turns.into_iter())
        .collect::<Vec<_>>();

    (grid, inst)
}

fn trace_path(grid: &Grid, instructions: &[Instruction]) -> Walker {
    let mut w = Walker::new(grid);
    println!("walker init: {:?}", w);

    let nrows = grid.len();
    let ncols = grid[0].len();

    for inst in instructions {
        match inst {
            Instruction::Turn(d) => w.turn(d),
            Instruction::Move(nsteps) => {
                for _ in 0..*nsteps {
                    let mut cpx = w.px.saturating_add_signed(w.fx as isize).rem_euclid(nrows);
                    let mut cpy = w.py.saturating_add_signed(w.fy as isize).rem_euclid(ncols);

                    if grid[cpx][cpy] == GridPos::Void {
                        (cpx, cpy) = w.wrap(grid);
                    }

                    match grid[cpx][cpy] {
                        GridPos::Wall => break,
                        GridPos::Open => {
                            w.px = cpx;
                            w.py = cpy
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    println!("walker final: {:?}", w);

    w
}

fn get_password(w: &Walker) -> usize {
    let facing = match (w.fx, w.fy) {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };

    1000 * (w.px + 1) + 4 * (w.py + 1) + facing
}

pub fn solve_a() -> Result<usize> {
    let (grid, instructions) = parse_input(include_str!("../input"));

    for row in &grid {
        println!("{}", row.len());
    }
    let w = trace_path(&grid, &instructions);
    let pw = get_password(&w);

    Ok(pw)
}

pub fn solve_b() -> Result<usize> {
    let input = include_str!("../input");

    Ok(0)
}
