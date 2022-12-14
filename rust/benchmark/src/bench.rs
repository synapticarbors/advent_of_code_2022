fn timeit(func: fn() -> ()) -> std::time::Duration {
    let start = std::time::Instant::now();
    (func)();
    start.elapsed()
}

fn main() {
    let times: Vec<_> = benchmark::solvers()
        .iter()
        .map(|s| {
            let x0 = timeit(s.func);
            let x1 = timeit(s.func);
            let xmid = (x0 + x1) / 2;
            let goal_duration = std::time::Duration::from_millis(200);
            let nruns = (goal_duration.as_nanos() as f64 / xmid.as_nanos() as f64).floor() as usize;
            let nruns = std::cmp::min(std::cmp::max(nruns, 10), 50) as usize;

            let keep = std::cmp::max(std::cmp::min(nruns, nruns / 10), 5) as usize;
            // println!("{} {}", nruns, keep);

            let mut x = (0..nruns).map(|_| timeit(s.func)).collect::<Vec<_>>();
            x.sort_unstable();
            (
                s.name,
                x.iter().take(keep).sum::<std::time::Duration>() / (keep as u32),
            )
        })
        .collect();

    times.iter().for_each(|t| println!("{}: {:?}", t.0, t.1));
    println!(
        "Total: {:?}",
        times.iter().map(|(_, t)| t).sum::<std::time::Duration>()
    );
}
