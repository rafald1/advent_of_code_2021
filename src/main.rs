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
mod day_11_dumbo_octopus;
mod day_12_passage_pathing;
mod day_13_transparent_origami;
mod day_14_extended_polymerization;
mod day_15_chiton;
mod day_16_packet_decoder;
mod day_17_trick_shot;
mod day_18_snailfish;
mod day_19_beacon_scanner;
mod day_20_trench_map;

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
        day_11_dumbo_octopus::part_1::solve,
        day_11_dumbo_octopus::part_2::solve,
        day_12_passage_pathing::part_1::solve,
        day_12_passage_pathing::part_2::solve,
        day_13_transparent_origami::part_1::solve,
        day_13_transparent_origami::part_2::solve,
        day_14_extended_polymerization::part_1::solve,
        day_14_extended_polymerization::part_2::solve,
        day_15_chiton::part_1::solve,
        day_15_chiton::part_2::solve,
        day_16_packet_decoder::part_1::solve,
        day_16_packet_decoder::part_2::solve,
        day_17_trick_shot::part_1::solve,
        day_17_trick_shot::part_2::solve,
        day_18_snailfish::part_1::solve,
        day_18_snailfish::part_2::solve,
        day_19_beacon_scanner::part_1::solve,
        day_19_beacon_scanner::part_2::solve,
        day_20_trench_map::part_1::solve,
        day_20_trench_map::part_2::solve,
    ];

    for solution in solutions {
        let start = std::time::Instant::now();
        // for _ in 0..999 {
        //     solution();
        // }
        let result = solution();
        let duration = start.elapsed();
        println!(
            "{} Solved on average in {:.3}ms.",
            result,
            duration.as_secs_f64()
        );
    }
}
