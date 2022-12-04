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

pub fn solve_a() -> Result<usize> {
    let x = include_str!("../input")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(ea, eb)| (ea.split_once('-').unwrap(), eb.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                a.parse::<u32>().unwrap(),
                b.parse::<u32>().unwrap(),
                c.parse::<u32>().unwrap(),
                d.parse::<u32>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (c >= a && d <= b))
        .count();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let x = include_str!("../input")
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(ea, eb)| (ea.split_once('-').unwrap(), eb.split_once('-').unwrap()))
        .map(|((a, b), (c, d))| {
            (
                a.parse::<u32>().unwrap(),
                b.parse::<u32>().unwrap(),
                c.parse::<u32>().unwrap(),
                d.parse::<u32>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| {
            (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b)
        })
        .count();

    Ok(x)
}
