fn process_data(path: &str) -> String {
    let file_content = std::fs::read_to_string(path).unwrap();

    file_content.chars().fold(String::new(), |mut acc, c| {
        let value = c.to_digit(16).expect("Should be hexadecimal character");
        use std::fmt::Write;
        write!(&mut acc, "{:04b}", value).unwrap();
        acc
    })
}

fn binary_to_decimal(packet: &mut String, length: usize) -> usize {
    let version: String = packet.drain(..length).collect();
    usize::from_str_radix(&version, 2).expect("Should be valid binary string")
}

fn parse_packet(packet: &mut String) -> usize {
    let mut total_version = binary_to_decimal(packet, 3);
    let packet_type_id = binary_to_decimal(packet, 3);

    match packet_type_id {
        4 => loop {
            let prefix = binary_to_decimal(packet, 1);
            let _number: String = packet.drain(..4).collect();

            if prefix == 0 {
                break;
            }
        },
        _ => {
            let length_type_id = binary_to_decimal(packet, 1);

            match length_type_id {
                0 => {
                    let sub_packet_length = binary_to_decimal(packet, 15);
                    let mut sub_packet: String = packet.drain(..sub_packet_length).collect();

                    while !sub_packet.is_empty() {
                        total_version += parse_packet(&mut sub_packet);
                    }
                }
                1 => {
                    let number_of_sub_packets = binary_to_decimal(packet, 11);
                    (0..number_of_sub_packets).for_each(|_| total_version += parse_packet(packet));
                }
                _ => unreachable!(),
            }
        }
    }

    total_version
}

pub fn solve() -> String {
    let mut binary_sequence = process_data("./input/16.txt");
    let result = parse_packet(&mut binary_sequence);
    format!("Day 16: Packet Decoder (Part 1) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_16_packet_decoder::part_1::*;

    #[test]
    fn solve_with_test_data() {
        let mut binary_sequence = process_data("./test_input/16.txt");
        let result = parse_packet(&mut binary_sequence);
        assert_eq!(result, 31);
    }
}
