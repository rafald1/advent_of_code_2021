mod day_01_sonar_sweep;

fn main() {
    let solutions = [
        day_01_sonar_sweep::part_1::solve,
        day_01_sonar_sweep::part_2::solve,
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
