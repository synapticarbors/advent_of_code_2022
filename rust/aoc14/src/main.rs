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

// Found by trial and error for input
const WIDTH: usize = 675;
const HEIGHT: usize = 165;

type Grid = [[u8; HEIGHT]; WIDTH];

#[derive(Debug)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Cave {
    g: Grid,
}

impl Cave {
    fn fill(&mut self) {
        while let Some(pos) = self.drop_sand() {
            self.g[pos.x][pos.y] = b'o';
        }
    }

    fn drop_sand(&self) -> Option<Pos> {
        let mut p = Pos { x: 500, y: 0 };

        if self.g[p.x][p.y] != b' ' {
            return None;
        }

        let fp = loop {
            if p.y + 1 >= HEIGHT {
                break None;
            }

            if self.g[p.x][p.y + 1] == b' ' {
                p.y += 1;
            } else if self.g[p.x - 1][p.y + 1] == b' ' {
                p.y += 1;
                p.x -= 1;
            } else if self.g[p.x + 1][p.y + 1] == b' ' {
                p.y += 1;
                p.x += 1;
            } else {
                break Some(p);
            }
        };

        return fp;
    }

    fn add_floor(&mut self) {
        let mut ymax = 0;
        for col in self.g.iter() {
            if let Some(col_ymax) = col.iter().rposition(|y| *y == b'#') {
                if col_ymax > ymax {
                    ymax = col_ymax;
                }
            }
        }

        for x in 0..WIDTH {
            self.g[x][ymax + 2] = b'#';
        }
    }

    fn num_resting_sand(&self) -> usize {
        self.g
            .iter()
            .map(|x| x.iter().filter(|b| **b == b'o').count())
            .sum()
    }
}

fn sort2(a: usize, b: usize) -> (usize, usize) {
    match (a, b) {
        (x, y) if x > y => (b, a),
        _ => (a, b),
    }
}

fn parse_input(input: &str) -> Grid {
    let mut g = [[b' '; HEIGHT]; WIDTH];
    for line in input.lines() {
        let mut coords = line
            .split(" -> ")
            .map(|x| x.split_once(',').unwrap())
            .map(|(a, b)| Pos {
                x: a.parse::<usize>().unwrap(),
                y: b.parse::<usize>().unwrap(),
            });

        let mut start = coords.next().unwrap();
        for end in coords {
            if start.y == end.y {
                let (xstart, xend) = sort2(start.x, end.x);
                for x in xstart..=xend {
                    g[x][start.y] = b'#';
                }
            }

            if start.x == end.x {
                let (ystart, yend) = sort2(start.y, end.y);
                for y in ystart..=yend {
                    g[start.x][y] = b'#';
                }
            }

            start = end;
        }
    }

    g
}

pub fn solve_a() -> Result<usize> {
    let g = parse_input(include_str!("../input"));
    let mut cave = Cave { g };
    cave.fill();
    let x = cave.num_resting_sand();

    Ok(x)
}

pub fn solve_b() -> Result<usize> {
    let g = parse_input(include_str!("../input"));
    let mut cave = Cave { g };
    cave.add_floor();
    cave.fill();
    let x = cave.num_resting_sand();

    Ok(x)
}
