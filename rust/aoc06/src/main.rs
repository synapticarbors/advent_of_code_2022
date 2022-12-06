use std::collections::HashSet;

use anyhow::{anyhow, Result};

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

fn find_start(stream: &[u8], block_size: usize) -> Result<usize> {
    let mut h = HashSet::with_capacity(block_size);
    for (i, w) in stream.windows(block_size).enumerate() {
        let processed = i + block_size;
        h.clear();
        for e in w.iter() {
            if !h.insert(e) {
                break;
            }
        }
        if h.len() == block_size {
            return Ok(processed);
        }
    }

    Err(anyhow!("Could not find start"))
}

pub fn solve_a() -> Result<usize> {
    let stream = include_bytes!("../input");
    find_start(stream, 4)
}

pub fn solve_b() -> Result<usize> {
    let stream = include_bytes!("../input");
    find_start(stream, 14)
}
