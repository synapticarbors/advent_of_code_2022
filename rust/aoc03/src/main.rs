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

fn get_priority(x: &u8) -> u64 {
    match *x {
        z if z >= b'a' => (z - b'a') as u64 + 1,
        z => (z - b'A') as u64 + 27,
    }
}

pub fn solve_a() -> Result<u64> {
    let x: u64 = include_bytes!("../input")
        .split(|b| *b == b'\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| {
            a.iter()
                .filter(|x| b.contains(x))
                .map(get_priority)
                .next()
                .unwrap()
        })
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<u64> {
    let x: u64 = include_bytes!("../input")
        .split(|b| *b == b'\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| {
            g[0].iter()
                .filter(|x| g[1].contains(x) && g[2].contains(x))
                .map(get_priority)
                .next()
                .unwrap()
        })
        .sum();

    Ok(x)
}
