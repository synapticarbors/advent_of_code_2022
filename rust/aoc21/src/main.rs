use std::collections::{HashMap, VecDeque};

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
enum Operation {
    Add,
    Sub,
    Mult,
    Div,
    Eq,
}

#[derive(Debug)]
struct MathProb {
    lhs: String,
    rhs: String,
    op: Operation,
}

impl MathProb {
    fn solve(&self, known: &HashMap<String, i64>) -> Option<i64> {
        match (known.get(&self.lhs), known.get(&self.rhs)) {
            (Some(a), Some(b)) => Some(evaluate_op(*a, *b, &self.op)),
            _ => None,
        }
    }
}

fn evaluate_op(lhs: i64, rhs: i64, op: &Operation) -> i64 {
    match op {
        Operation::Add => lhs + rhs,
        Operation::Sub => lhs - rhs,
        Operation::Mult => lhs * rhs,
        Operation::Div => lhs / rhs,
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> (HashMap<String, i64>, HashMap<String, MathProb>) {
    input.lines().fold(
        (HashMap::new(), HashMap::new()),
        |(mut known, mut probs), line| {
            let key = line[0..4].to_string();

            if line[6..7].chars().next().unwrap().is_ascii_digit() {
                let val = line[6..].parse::<i64>().unwrap();
                known.insert(key, val);
            } else {
                let lhs = line[6..10].to_string();
                let rhs = line[13..].to_string();
                let op = match &line[11..12] {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mult,
                    "/" => Operation::Div,
                    _ => unreachable!(),
                };
                probs.insert(key, MathProb { lhs, rhs, op });
            }
            (known, probs)
        },
    )
}

fn solve_root(known: &mut HashMap<String, i64>, probs: &mut HashMap<String, MathProb>) -> i64 {
    let to_solve = build_branch("root".to_string(), known, probs);
    traverse_solve(&to_solve, known, probs);

    *known.get("root").unwrap()
}

fn traverse_solve(
    to_solve: &[String],
    known: &mut HashMap<String, i64>,
    probs: &mut HashMap<String, MathProb>,
) {
    for x in to_solve.iter().rev() {
        if known.contains_key(x) {
            continue;
        } else if probs.contains_key(x) {
            let mp = probs.get(x).unwrap();
            let soln = mp.solve(known).unwrap();
            known.insert(x.to_owned(), soln);
        } else {
            unreachable!();
        }
    }
}

fn build_branch(
    start: String,
    known: &HashMap<String, i64>,
    probs: &HashMap<String, MathProb>,
) -> Vec<String> {
    let mut queue = VecDeque::from([start.clone()]);
    let mut to_solve = Vec::new();
    to_solve.push(start);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        if known.contains_key(&current) {
            continue;
        }

        let mp = probs.get(&current).unwrap();

        queue.push_back(mp.lhs.clone());
        queue.push_back(mp.rhs.clone());

        to_solve.push(mp.lhs.clone());
        to_solve.push(mp.rhs.clone());
    }

    to_solve
}

fn reduce_problems(known: &mut HashMap<String, i64>, probs: &mut HashMap<String, MathProb>) {
    loop {
        let n_unsolved = probs.len();

        probs.retain(|name, mp| {
            if let Some(val) = mp.solve(known) {
                known.insert(name.to_owned(), val);
                false
            } else {
                true
            }
        });

        if probs.len() == n_unsolved {
            break;
        }
    }
}

pub fn solve_a() -> Result<i64> {
    let (mut known, mut problems) = parse_input(include_str!("../input"));

    let x = solve_root(&mut known, &mut problems);

    Ok(x)
}

pub fn solve_b() -> Result<i64> {
    let (mut known, mut problems) = parse_input(include_str!("../input"));

    known.remove(&String::from("humn"));
    if let Some(p) = problems.get_mut(&String::from("root")) {
        p.op = Operation::Eq;
    } else {
        panic!();
    }

    reduce_problems(&mut known, &mut problems);

    let mut search = String::from("root");
    let mut soln = 0;

    while search != *"humn" {
        let mp = problems.get(&search).unwrap();
        let clhs = mp.lhs.to_string();
        let crhs = mp.rhs.to_string();
        (search, soln) = match (&mp.op, known.get(&mp.lhs), known.get(&mp.rhs)) {
            (Operation::Add, Some(x), None) => (crhs, soln - x),
            (Operation::Add, None, Some(x)) => (clhs, soln - x),
            (Operation::Sub, Some(x), None) => (crhs, x - soln),
            (Operation::Sub, None, Some(x)) => (clhs, soln + x),
            (Operation::Mult, Some(x), None) => (crhs, soln / x),
            (Operation::Mult, None, Some(x)) => (clhs, soln / x),
            (Operation::Div, Some(x), None) => (crhs, x / soln),
            (Operation::Div, None, Some(x)) => (clhs, soln * x),
            (Operation::Eq, Some(x), None) => (crhs, *x),
            (Operation::Eq, None, Some(x)) => (clhs, *x),
            _ => unreachable!(),
        };
    }

    Ok(soln)
}
