use std::cmp::Ordering;

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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (Packet::Num(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            (_, Packet::Num(_)) => self.cmp(&Packet::List(vec![other.clone()])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(s: &str) -> Packet {
    if &s[0..1] == "[" {
        let mut stack: i32 = 0;
        Packet::List(
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter(|s| !s.is_empty())
                .map(parse)
                .collect(),
        )
    } else {
        Packet::Num(s.parse().unwrap())
    }
}

pub fn solve_a() -> Result<usize> {
    let packets = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse)
        .collect::<Vec<_>>();

    let x = packets
        .chunks(2)
        .enumerate()
        .map(|(i, p)| {
            if p[0].cmp(&p[1]) == Ordering::Less {
                i + 1
            } else {
                0
            }
        })
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let dp1 = parse("[[2]]");
    let dp2 = parse("[[6]]");

    let packets = include_str!("../input")
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse)
        .filter(|x| x < &dp2)
        .collect::<Vec<_>>();

    let x = (packets.iter().filter(|x| *x < &dp1).count() + 1) * (packets.len() + 2);

    Ok(x)
}
