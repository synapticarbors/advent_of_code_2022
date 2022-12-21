use anyhow::Result;
use atoi::atoi;

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

fn parse_input(input: &[u8]) -> Vec<isize> {
    input
        .split(|b| *b == b'\n')
        .map(|b| atoi::<isize>(b).unwrap())
        .collect()
}

fn move_element<T>(x: &mut [T], src_ix: usize, dst_ix: usize) {
    if src_ix < dst_ix {
        x[src_ix..=dst_ix].rotate_left(1);
    } else {
        x[dst_ix..=src_ix].rotate_right(1);
    }
}

fn decrypt(seq: &mut [isize], ntimes: usize) {
    let n = seq.len();
    let mut idx = (0..n).collect::<Vec<usize>>();

    for _ in 0..ntimes {
        for j in 0..n {
            let src_ix = idx.iter().position(|&i| i == j).unwrap();

            let val = seq[src_ix];
            let dst_ix: usize =
                (src_ix + val.rem_euclid(n as isize - 1) as usize).rem_euclid(n - 1);

            move_element(seq, src_ix, dst_ix);
            move_element(&mut idx, src_ix, dst_ix);
        }
    }
}

fn get_grove_coords(seq: &[isize]) -> isize {
    seq.iter()
        .cycle()
        .skip_while(|v| **v != 0)
        .step_by(1000)
        .skip(1)
        .take(3)
        .sum()
}

pub fn solve_a() -> Result<isize> {
    let mut x = parse_input(include_bytes!("../input"));

    decrypt(&mut x, 1);

    let gcoords = get_grove_coords(&x);

    Ok(gcoords)
}

pub fn solve_b() -> Result<isize> {
    let key: isize = 811589153;

    let mut x = parse_input(include_bytes!("../input"))
        .iter()
        .map(|e| e * key)
        .collect::<Vec<_>>();

    decrypt(&mut x, 10);

    let gcoords = get_grove_coords(&x);

    Ok(gcoords)
}
