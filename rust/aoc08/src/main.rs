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

fn parse_input(input: &[u8]) -> Vec<Vec<u8>> {
    input
        .split(|b| *b == b'\n')
        .map(|line| line.iter().map(|b| b - b'0').collect())
        .collect()
}

pub fn solve_a() -> Result<usize> {
    let map = parse_input(include_bytes!("../input"));
    let nrows = map.len();
    let ncols = map[0].len();
    let mut is_visible = vec![vec![false; ncols]; nrows];
    // let mut cummax = vec![vec![0; ncols]; nrows];

    // West-to-East
    for i in 0..nrows {
        let mut max_height = 0u8;
        for j in 0..ncols {
            let v = map[i][j];
            if j == 0 {
                is_visible[i][j] = true;
                max_height = v;
                continue;
            }

            if v > max_height {
                is_visible[i][j] = true;
                max_height = v;
            } else {
                is_visible[i][j] = false;
            }
        }
    }

    // East-to-West
    for i in 0..nrows {
        let mut max_height = 0u8;
        for j in (0..ncols).rev() {
            let v = map[i][j];
            if j == ncols - 1 {
                is_visible[i][j] = true;
                max_height = v;
                continue;
            }

            if v > max_height {
                is_visible[i][j] |= true;
                max_height = v;
            } else {
                is_visible[i][j] |= false;
            }
        }
    }

    // North-to-South
    for j in 0..ncols {
        let mut max_height = 0u8;
        for i in 0..nrows {
            let v = map[i][j];
            if i == 0 {
                is_visible[i][j] = true;
                max_height = v;
                continue;
            }

            if v > max_height {
                is_visible[i][j] |= true;
                max_height = v;
            } else {
                is_visible[i][j] |= false;
            }
        }
    }

    // South-to-North
    for j in 0..ncols {
        let mut max_height = 0u8;
        for i in (0..nrows).rev() {
            let v = map[i][j];
            if i == nrows - 1 {
                is_visible[i][j] = true;
                max_height = v;
                continue;
            }

            if v > max_height {
                is_visible[i][j] |= true;
                max_height = v;
            } else {
                is_visible[i][j] |= false;
            }
        }
    }

    Ok(is_visible.into_iter().flatten().map(|x| x as usize).sum())
}

pub fn solve_b() -> Result<usize> {
    let map = parse_input(include_bytes!("../input"));
    let nrows = map.len();
    let ncols = map[0].len();

    let mut max_score = 0usize;

    for i in 1..nrows - 1 {
        for j in 1..ncols - 1 {
            let h = map[i][j];
            if h <= 1 {
                continue;
            }
            max_score = max_score.max({
                // West
                let w = (1..j + 1)
                    .map(|offset| map[i][j - offset])
                    .position(|nh| nh >= h)
                    .unwrap_or_else(|| j.wrapping_sub(1))
                    .wrapping_add(1);

                // East
                let e = (1..(ncols - j))
                    .map(|offset| map[i][j + offset])
                    .position(|nh| nh >= h)
                    .unwrap_or_else(|| (ncols - j).wrapping_sub(2))
                    .wrapping_add(1);

                // North
                let n = (1..i + 1)
                    .map(|offset| map[i - offset][j])
                    .position(|nh| nh >= h)
                    .unwrap_or_else(|| i.wrapping_sub(1))
                    .wrapping_add(1);

                // South
                let s = (1..(nrows - i))
                    .map(|offset| map[i + offset][j])
                    .position(|nh| nh >= h)
                    .unwrap_or_else(|| (nrows - i).wrapping_sub(2))
                    .wrapping_add(1);

                n * s * e * w
            });
        }
    }

    Ok(max_score)
}
