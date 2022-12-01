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

pub fn solve_a() -> Result<u32> {
    let x = include_str!("../input")
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    Ok(x)
}

pub fn solve_b() -> Result<u32> {
    let mut ec = include_str!("../input")
        .split("\n\n")
        .map(|g| g.lines().map(|l| l.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    ec.sort_unstable();

    let x = ec.iter().rev().take(3).sum();

    Ok(x)
}
