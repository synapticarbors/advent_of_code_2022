use std::collections::VecDeque;

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
    Add(usize),
    Multiply(usize),
    Square,
}

type MonkeyIndex = usize;

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    test_div_by: usize,
    ttt: MonkeyIndex,
    ttf: MonkeyIndex,
    ninspected: usize,
}

fn parse_ending_int(line: &str) -> usize {
    line.split_ascii_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn parse_starting_items(line: &str) -> VecDeque<usize> {
    line.split_once(':')
        .unwrap()
        .1
        .split(',')
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Operation {
    let p = line
        .split_once('=')
        .unwrap()
        .1
        .trim()
        .splitn(3, ' ')
        .collect::<Vec<_>>();

    match (p[1], p[2]) {
        ("*", "old") => Operation::Square,
        ("+", x) => Operation::Add(x.parse::<usize>().unwrap()),
        ("*", x) => Operation::Multiply(x.parse::<usize>().unwrap()),
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|block| {
            let lines = block.splitn(6, '\n').collect::<Vec<_>>();

            let items = parse_starting_items(lines[1]);
            let op = parse_operation(lines[2]);
            let test_div_by = parse_ending_int(lines[3]);
            let ttt = parse_ending_int(lines[4]);
            let ttf = parse_ending_int(lines[5]);

            Monkey {
                items,
                op,
                test_div_by,
                ttt,
                ttf,
                ninspected: 0,
            }
        })
        .collect()
}

pub fn solve_a() -> Result<usize> {
    let nrounds = 20;

    let mut monkeys = parse_input(include_str!("../input"));

    for _ in 0..nrounds {
        for mi in 0..monkeys.len() {
            while let Some(w) = monkeys[mi].items.pop_front() {
                let m = &mut monkeys[mi];
                let nw = match m.op {
                    Operation::Add(x) => (w + x) / 3,
                    Operation::Multiply(x) => (w * x) / 3,
                    Operation::Square => (w * w) / 3,
                };
                m.ninspected += 1;

                let target_mi = if nw % m.test_div_by == 0 {
                    m.ttt
                } else {
                    m.ttf
                };

                monkeys[target_mi].items.push_back(nw);
            }
        }
    }

    monkeys.sort_unstable_by_key(|e| e.ninspected);

    let monkey_business =
        monkeys[monkeys.len() - 1].ninspected * monkeys[monkeys.len() - 2].ninspected;

    Ok(monkey_business)
}

pub fn solve_b() -> Result<usize> {
    let nrounds = 10_000;

    let mut monkeys = parse_input(include_str!("../input"));

    let lcm = monkeys.iter().map(|m| m.test_div_by).product::<usize>();

    for _ in 0..nrounds {
        for mi in 0..monkeys.len() {
            while let Some(w) = monkeys[mi].items.pop_front() {
                let m = &mut monkeys[mi];
                let nw = match m.op {
                    Operation::Add(x) => (w + x) % lcm,
                    Operation::Multiply(x) => (w * x) % lcm,
                    Operation::Square => (w * w) % lcm,
                };
                m.ninspected += 1;

                let target_mi = if nw % m.test_div_by == 0 {
                    m.ttt
                } else {
                    m.ttf
                };

                monkeys[target_mi].items.push_back(nw);
            }
        }
    }

    monkeys.sort_unstable_by_key(|e| e.ninspected);

    let monkey_business =
        monkeys[monkeys.len() - 1].ninspected * monkeys[monkeys.len() - 2].ninspected;

    Ok(monkey_business)
}
