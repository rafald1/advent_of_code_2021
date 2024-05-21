mod day_01_sonar_sweep;
mod day_02_dive;
mod day_03_binary_diagnostic;
mod day_04_giant_squid;
mod day_05_hydrothermal_venture;
mod day_06_lanternfish;
mod day_07_the_treachery_of_whales;
mod day_08_seven_segment_search;
mod day_09_smoke_basin;
mod day_10_syntax_scoring;

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
        day_07_the_treachery_of_whales::part_1::solve,
        day_07_the_treachery_of_whales::part_2::solve,
        day_08_seven_segment_search::part_1::solve,
        day_08_seven_segment_search::part_2::solve,
        day_09_smoke_basin::part_1::solve,
        day_09_smoke_basin::part_2::solve,
        day_10_syntax_scoring::part_1::solve,
        day_10_syntax_scoring::part_2::solve,
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
