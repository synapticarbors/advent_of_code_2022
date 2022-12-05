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

fn parse_drawing(drawing: &str) -> Vec<Vec<char>> {
    drawing
        .lines()
        .rev()
        // .skip(1)
        .enumerate()
        .fold(Vec::<Vec<char>>::new(), |mut acc, (li, line)| {
            if li == 0 {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .for_each(|_| acc.push(vec![]));
            } else {
                line.chars()
                    .skip(1)
                    .step_by(4)
                    .enumerate()
                    .filter(|(_, c)| *c != ' ')
                    .for_each(|(i, c)| acc[i].push(c));
            }
            acc
        })
}

pub fn solve_a() -> Result<String> {
    let (drawing, procedure) = include_str!("../input").split_once("\n\n").unwrap();

    let mut x = parse_drawing(drawing);

    procedure
        .lines()
        .map(|line| line.splitn(6, ' '))
        .map(|g| {
            let x = g.collect::<Vec<_>>();
            (
                x[1].parse::<u16>().unwrap(),
                x[3].parse::<usize>().unwrap(),
                x[5].parse::<usize>().unwrap(),
            )
        })
        .for_each(|(n, src, dst)| {
            for _ in 0..n {
                let tmp = x[src - 1].pop().unwrap();
                x[dst - 1].push(tmp);
            }
        });

    let out = x.iter().map(|stack| stack.last().unwrap()).collect();
    Ok(out)
}

pub fn solve_b() -> Result<String> {
    let (drawing, procedure) = include_str!("../input").split_once("\n\n").unwrap();

    let mut x = parse_drawing(drawing);

    procedure
        .lines()
        .map(|line| line.splitn(6, ' '))
        .map(|g| {
            let x = g.collect::<Vec<_>>();
            (
                x[1].parse::<usize>().unwrap(),
                x[3].parse::<usize>().unwrap(),
                x[5].parse::<usize>().unwrap(),
            )
        })
        .for_each(|(n, src, dst)| {
            let src_len = x[src - 1].len();
            let mgrp = x[src - 1].drain(src_len - n..).collect::<Vec<_>>();
            x[dst - 1].extend_from_slice(&mgrp);
        });

    let out = x.iter().map(|stack| stack.last().unwrap()).collect();
    Ok(out)
}
