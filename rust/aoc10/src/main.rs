use anyhow::Result;

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    let soln_a = solve_a()?;
    eprintln!("Part A elapsed {:?}", start.elapsed());
    println!("solution part A: {}", soln_a);

    let start = std::time::Instant::now();
    let soln_b = solve_b()?;
    eprintln!("Part B elapsed {:?}", start.elapsed());
    println!("solution part B: \n{}", soln_b);

    Ok(())
}

pub fn solve_a() -> Result<i32> {
    let mut interesting_cycles = vec![220, 180, 140, 100, 60, 20];

    let x = include_bytes!("../input")
        .split(|b| *b == b'\n')
        .fold((1, 0, 0), |(mut register, mut sss, mut cycle), line| {
            let (dcycle, dreg) = match line {
                line if line.starts_with(b"n") => (1, 0),
                _ => {
                    let n = atoi::atoi::<i32>(line.split(|b| *b == b' ').last().unwrap()).unwrap();
                    (2, n)
                }
            };

            if let Some(coi) = interesting_cycles.last() {
                if cycle + dcycle >= *coi {
                    sss += coi * register;
                    interesting_cycles.pop();
                }
            }

            register += dreg;
            cycle += dcycle;

            (register, sss, cycle)
        })
        .1;

    Ok(x)
}

fn push_pixel(crt: &mut Vec<char>, cycle: i32, x: i32) {
    let p = cycle % 40;
    if x - 1 <= p && x + 1 >= p {
        crt.push('#');
    } else {
        crt.push('.');
    }
}

pub fn solve_b() -> Result<String> {
    let display = include_bytes!("../input")
        .split(|b| *b == b'\n')
        .fold(
            (1, 0, Vec::with_capacity(40 * 6)),
            |(mut x, mut cycle, mut crt), line| {
                push_pixel(&mut crt, cycle, x);
                cycle += 1;

                if line.starts_with(b"a") {
                    push_pixel(&mut crt, cycle, x);
                    cycle += 1;
                    let n = atoi::atoi::<i32>(line.split(|b| *b == b' ').last().unwrap()).unwrap();
                    x += n;
                }

                (x, cycle, crt)
            },
        )
        .2;

    let x = display
        .chunks(40)
        .map(|chunk| format!("{:?}\n", String::from_iter(chunk)))
        .collect();

    Ok(x)
}
