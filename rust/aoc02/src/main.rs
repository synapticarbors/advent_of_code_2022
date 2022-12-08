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
        .map(|l| match l.split_once(' ').unwrap() {
            ("A", "X") => 4, // Rock v Rock
            ("A", "Y") => 8, // Rock v Paper
            ("A", "Z") => 3, // Rock v Scissors
            ("B", "X") => 1, // Paper v Rock
            ("B", "Y") => 5, // Paper v Paper
            ("B", "Z") => 9, // Paper v Scissors
            ("C", "X") => 7, // Scissors v Rock
            ("C", "Y") => 2, // Scissors v Paper
            ("C", "Z") => 6, // Scissors v Scissors
            _ => unreachable!(),
        })
        .sum();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let x = include_str!("../input")
        .lines()
        .map(|l| match l.split_once(' ').unwrap() {
            ("A", "X") => 3, // Rock v Scissors
            ("A", "Y") => 4, // Rock v Rock
            ("A", "Z") => 8, // Rock v Paper
            ("B", "X") => 1, // Paper v Rock
            ("B", "Y") => 5, // Paper v Paper
            ("B", "Z") => 9, // Paper v Scissors
            ("C", "X") => 2, // Scissors v Paper
            ("C", "Y") => 6, // Scissors v Scissors
            ("C", "Z") => 7, // Scissors v Rock
            _ => unreachable!(),
        })
        .sum();

    Ok(x)
}
