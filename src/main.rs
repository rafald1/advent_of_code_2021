mod day_01_sonar_sweep;
mod day_02_dive;
mod day_03_binary_diagnostic;
mod day_04_giant_squid;
mod day_05_hydrothermal_venture;
mod day_06_lanternfish;

fn main() {
    let solutions = [
        day_01_sonar_sweep::part_1::solve,
        day_01_sonar_sweep::part_2::solve,
        day_02_dive::part_1::solve,
        day_02_dive::part_2::solve,
        day_03_binary_diagnostic::part_1::solve,
        day_03_binary_diagnostic::part_2::solve,
        day_04_giant_squid::part_1::solve,
        day_04_giant_squid::part_2::solve,
        day_05_hydrothermal_venture::part_1::solve,
        day_05_hydrothermal_venture::part_2::solve,
        day_06_lanternfish::part_1::solve,
        day_06_lanternfish::part_2::solve,
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
