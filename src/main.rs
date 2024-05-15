mod day_01_sonar_sweep;
mod day_02_dive;
mod day_03_binary_diagnostic;

fn main() {
    let solutions = [
        day_01_sonar_sweep::part_1::solve,
        day_01_sonar_sweep::part_2::solve,
        day_02_dive::part_1::solve,
        day_02_dive::part_2::solve,
        day_03_binary_diagnostic::part_1::solve,
        day_03_binary_diagnostic::part_2::solve,
    ];

    for solution in solutions {
        let start = std::time::Instant::now();
        for _ in 0..999 {
            solution();
        }
        let result = solution();
        let duration = start.elapsed();
        println!(
            "{} Solved on average in {:.3}ms.",
            result,
            duration.as_secs_f64()
        );
    }
}
