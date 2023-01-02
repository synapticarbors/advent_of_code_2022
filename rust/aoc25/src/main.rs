use anyhow::Result;

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    Ok(())
}

fn dec2snafu(dec: i64) -> String {
    let mut snafu = Vec::new();
    let mut n = dec;
    while n > 0 {
        let remainder = n % 5;
        let s = match remainder {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        snafu.push(s);

        if remainder > 2 {
            n += 5
        }
        n /= 5;
    }

    snafu.iter().rev().collect()
}

pub fn solve_a() -> Result<String> {
    let dec_sum = include_str!("../input")
        .lines()
        .map(|line| {
            line.chars().rev().enumerate().fold(0, |acc, (i, c)| {
                let value = match c {
                    '0' => 0,
                    '1' => 5_i64.pow(i as u32),
                    '2' => 2 * 5_i64.pow(i as u32),
                    '-' => -(5_i64.pow(i as u32)),
                    '=' => -2 * 5_i64.pow(i as u32),
                    _ => unreachable!(),
                };
                acc + value
            })
        })
        .sum::<i64>();

    let x = dec2snafu(dec_sum);
    Ok(x)
}
