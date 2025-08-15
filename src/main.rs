mod day1;
mod day2;
mod shared;

use std::env;

fn main() {
    let solvers: Vec<&dyn Fn()> = vec![&day1::solver::solve, &day2::solver::solve];

    let selected_day = env::args().nth(1);

    if let Some(day_str) = selected_day {
        let day = match day_str.parse::<usize>() {
            Ok(d) => d,
            Err(_) => panic!(
                "Selected day argument must be a positive integer, not '{}'.",
                day_str
            ),
        };

        if day > solvers.len() {
            panic!(
                "Selected day out of bounds, must be between 1 and {}.",
                solvers.len()
            )
        }

        solvers.iter().nth(day - 1).unwrap()();
    } else {
        for solver in solvers {
            solver();
        }
    }
}
