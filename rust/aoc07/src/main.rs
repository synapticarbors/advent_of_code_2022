use anyhow::Result;
use std::collections::HashMap;

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
enum ProgLine<'a> {
    Cd(&'a str),
    Dir(&'a str),
    File(usize),
    Ls,
}

fn parse_line(line: &str) -> ProgLine {
    match line {
        x if x.starts_with("$ cd") => ProgLine::Cd(x.splitn(3, ' ').nth(2).unwrap()),
        x if x.starts_with("$ ls") => ProgLine::Ls,
        x if x.starts_with("dir") => ProgLine::Dir(x.split_once(' ').unwrap().1),
        x => {
            let (sz, _) = x.split_once(' ').unwrap();
            ProgLine::File(sz.parse::<usize>().unwrap())
        }
    }
}

fn get_directory_sizes(input: &str) -> HashMap<String, usize> {
    input
        .lines()
        .fold(
            (Vec::<&str>::new(), HashMap::<String, usize>::new()),
            |(mut cwd, mut dirsize), line| {
                match parse_line(line) {
                    ProgLine::Cd(x) => {
                        if x == ".." {
                            cwd.pop();
                        } else {
                            cwd.push(x);
                        }
                    }
                    ProgLine::Dir(_) => (),
                    ProgLine::Ls => (),
                    ProgLine::File(x) => cwd.iter().enumerate().for_each(|(i, _)| {
                        let dirpath = cwd[..i + 1].join("/");
                        *dirsize.entry(dirpath).or_insert(0) += x;
                    }),
                }
                (cwd, dirsize)
            },
        )
        .1
}

pub fn solve_a() -> Result<usize> {
    let x = get_directory_sizes(include_str!("../input"))
        .values()
        .filter(|x| **x <= 100000)
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let dirsize = get_directory_sizes(include_str!("../input"));

    let used_space = dirsize.get("/").unwrap();
    let free_space = 70000000 - used_space;
    let min_delete_size = 30000000 - free_space;

    let x = dirsize
        .values()
        .filter(|&s| *s >= min_delete_size)
        .copied()
        .min()
        .unwrap();

    Ok(x)
}
