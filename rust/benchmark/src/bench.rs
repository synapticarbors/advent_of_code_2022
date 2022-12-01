const NRUNS: usize = 50;
const KEEP: usize = NRUNS / 10;

fn timeit(func: fn() -> ()) -> std::time::Duration {
    let start = std::time::Instant::now();
    (func)();
    start.elapsed()
}

fn main() {
    let times: Vec<_> = benchmark::solvers()
        .iter()
        .map(|s| {
            let mut x = (0..NRUNS).map(|_| timeit(s.func)).collect::<Vec<_>>();
            x.sort_unstable();
            (
                s.name,
                x.iter().take(KEEP).sum::<std::time::Duration>() / (KEEP as u32),
            )
        })
        .collect();

    times.iter().for_each(|t| println!("{}: {:?}", t.0, t.1));
    println!(
        "Total: {:?}",
        times.iter().map(|(_, t)| t).sum::<std::time::Duration>()
    );
}
