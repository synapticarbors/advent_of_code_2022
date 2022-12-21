macro_rules! drop_result {
    ($e:expr) => {{
        fn x() {
            let _ = $e();
        }

        x
    }};
}

macro_rules! soln {
    ($x:expr, $y:expr) => {{
        Soln {
            func: drop_result!($x),
            name: $y,
        }
    }};
}

pub struct Soln {
    pub func: fn() -> (),
    pub name: &'static str,
}

pub fn solvers() -> &'static [Soln] {
    &[
        soln!(aoc01::solve_a, "aoc01a"),
        soln!(aoc01::solve_b, "aoc01b"),
        soln!(aoc02::solve_a, "aoc02a"),
        soln!(aoc02::solve_b, "aoc02b"),
        soln!(aoc03::solve_a, "aoc03a"),
        soln!(aoc03::solve_b, "aoc03b"),
        soln!(aoc04::solve_a, "aoc04a"),
        soln!(aoc04::solve_b, "aoc04b"),
        soln!(aoc05::solve_a, "aoc05a"),
        soln!(aoc05::solve_b, "aoc05b"),
        soln!(aoc06::solve_a, "aoc06a"),
        soln!(aoc06::solve_b, "aoc06b"),
        soln!(aoc07::solve_a, "aoc07a"),
        soln!(aoc07::solve_b, "aoc07b"),
        soln!(aoc08::solve_a, "aoc08a"),
        soln!(aoc08::solve_b, "aoc08b"),
        soln!(aoc09::solve_a, "aoc09a"),
        soln!(aoc09::solve_b, "aoc09b"),
        soln!(aoc10::solve_a, "aoc10a"),
        soln!(aoc10::solve_b, "aoc10b"),
        soln!(aoc11::solve_a, "aoc11a"),
        soln!(aoc11::solve_b, "aoc11b"),
        soln!(aoc12::solve_a, "aoc12a"),
        soln!(aoc12::solve_b, "aoc12b"),
        soln!(aoc13::solve_a, "aoc13a"),
        soln!(aoc13::solve_b, "aoc13b"),
        soln!(aoc14::solve_a, "aoc14a"),
        soln!(aoc14::solve_b, "aoc14b"),
        soln!(aoc15::solve_a, "aoc15a"),
        soln!(aoc15::solve_b, "aoc15b"),
        soln!(aoc16::solve_a, "aoc16a"),
        soln!(aoc16::solve_b, "aoc16b"),
        soln!(aoc17::solve_a, "aoc17a"),
        soln!(aoc17::solve_b, "aoc17b"),
        soln!(aoc18::solve_a, "aoc18a"),
        soln!(aoc18::solve_b, "aoc18b"),
        soln!(aoc19::solve_a, "aoc19a"),
        soln!(aoc19::solve_b, "aoc19b"),
        soln!(aoc20::solve_a, "aoc20a"),
        soln!(aoc20::solve_b, "aoc20b"),
        // soln!(aoc21::solve_a, "aoc21a"),
        // soln!(aoc21::solve_b, "aoc21b"),
        // soln!(aoc22::solve_a, "aoc22a"),
        // soln!(aoc22::solve_b, "aoc22b"),
        // soln!(aoc23::solve_a, "aoc23a"),
        // soln!(aoc23::solve_b, "aoc23b"),
        // soln!(aoc24::solve_a, "aoc24a"),
        // soln!(aoc24::solve_b, "aoc24b"),
        // soln!(aoc25::solve_a, "aoc25a"),
        // soln!(aoc25::solve_b, "aoc25b"),
    ]
}
