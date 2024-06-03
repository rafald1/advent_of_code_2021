struct PacketType;

impl PacketType {
    const SUM: usize = 0;
    const PRODUCT: usize = 1;
    const MINIMUM: usize = 2;
    const MAXIMUM: usize = 3;
    const LITERAL_VALUE: usize = 4;
    const GREATER_THAN: usize = 5;
    const LESS_THAN: usize = 6;
    const EQUAL_TO: usize = 7;
}

struct LengthTypeId;

impl LengthTypeId {
    const SUB_PACKET_LENGTH_STORED_IN_15_BITS: usize = 0;
    const NUMBER_OF_SUB_PACKETS_STORED_IN_11_BITS: usize = 1;
}

fn process_data(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn convert_hexadecimal_sequence_into_binary(hexadecimal_sequence: &str) -> String {
    hexadecimal_sequence
        .chars()
        .fold(String::new(), |mut acc, c| {
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

fn extract_decimal_number_from_literal_value_packet(packet: &mut String) -> usize {
    let mut number = String::new();

    loop {
        let prefix = binary_to_decimal(packet, 1);
        number.push_str(&packet.drain(..4).collect::<String>());

        if prefix == 0 {
            return usize::from_str_radix(&number, 2).expect("Should be valid binary string");
        }
    }
}

fn parse_packet(packet: &mut String) -> usize {
    let _version = binary_to_decimal(packet, 3);
    let packet_type_id = binary_to_decimal(packet, 3);

    if packet_type_id == PacketType::LITERAL_VALUE {
        return extract_decimal_number_from_literal_value_packet(packet);
    }

    let length_type_id = binary_to_decimal(packet, 1);
    let mut numbers: Vec<usize> = Vec::new();

    match length_type_id {
        LengthTypeId::SUB_PACKET_LENGTH_STORED_IN_15_BITS => {
            let sub_packet_length = binary_to_decimal(packet, 15);
            let mut sub_packet: String = packet.drain(..sub_packet_length).collect();

            while !sub_packet.is_empty() {
                numbers.push(parse_packet(&mut sub_packet));
            }
        }
        LengthTypeId::NUMBER_OF_SUB_PACKETS_STORED_IN_11_BITS => {
            let number_of_sub_packets = binary_to_decimal(packet, 11);
            (0..number_of_sub_packets).for_each(|_| numbers.push(parse_packet(packet)));
        }
        _ => unreachable!(),
    }

    match packet_type_id {
        PacketType::SUM => numbers.iter().sum(),
        PacketType::PRODUCT => numbers.iter().product(),
        PacketType::MINIMUM => *numbers.iter().min().unwrap(),
        PacketType::MAXIMUM => *numbers.iter().max().unwrap(),
        PacketType::GREATER_THAN => (numbers.first() > numbers.last()) as usize,
        PacketType::LESS_THAN => (numbers.first() < numbers.last()) as usize,
        PacketType::EQUAL_TO => (numbers.first() == numbers.last()) as usize,
        _ => unreachable!(),
    }
}

pub fn solve() -> String {
    let hexadecimal_sequence = process_data("./input/16.txt");
    let mut binary_sequence = convert_hexadecimal_sequence_into_binary(&hexadecimal_sequence);
    let result = parse_packet(&mut binary_sequence);
    format!("Day 16: Packet Decoder (Part 2) answer: {}", result)
}

#[cfg(test)]
mod tests {
    use crate::day_16_packet_decoder::part_2::*;

    #[test]
    fn read_from_file_to_string() {
        let hexadecimal_sequence = process_data("./test_input/16.txt");
        assert_eq!(hexadecimal_sequence, "A0016C880162017C3686B18A3D4780");
    }

    #[test]
    fn convert_hexadecimal_string_sequence_into_binary_string_sequence() {
        let binary_sequence = convert_hexadecimal_sequence_into_binary("D2FE28");
        assert_eq!(binary_sequence, "110100101111111000101000");
    }

    #[test]
    fn convert_binary_number_in_string_to_decimal_value() {
        let decimal_number = binary_to_decimal(&mut String::from("000000000011011"), 15);
        assert_eq!(decimal_number, 27);
    }

    #[test]
    fn extract_value_from_literal_value_packet() {
        let decimal_number = extract_decimal_number_from_literal_value_packet(&mut String::from(
            "101111111000101000",
        ));
        assert_eq!(decimal_number, 2021);
    }

    #[test]
    fn parse_packets_and_calculate_end_value() {
        assert_eq!(
            parse_packet(&mut "1100001000000000101101000000101010000010".to_owned()),
            3
        );
        assert_eq!(
            parse_packet(&mut "000001000000000001011010110000110011100010010000".to_owned()),
            54
        );
        assert_eq!(
            parse_packet(
                &mut "10001000000000001000011011000011111010001000000100010010".to_owned()
            ),
            7
        );
        assert_eq!(
            parse_packet(
                &mut "11001110000000001100010000111101100010000001000100100000".to_owned()
            ),
            9
        );
        assert_eq!(
            parse_packet(&mut "110110000000000001011010110000101010100011110000".to_owned()),
            1
        );
        assert_eq!(
            parse_packet(&mut "1111011000000000101111000010110110001111".to_owned()),
            0
        );
        assert_eq!(
            parse_packet(&mut "100111000000000001011010110000101111100011110000".to_owned()),
            0
        );
        assert_eq!(
            parse_packet(
                &mut "1001110000000001010000010000100000000010010100000011\
                      0010000011110001100000000010000100000100101000001000"
                    .to_owned()
            ),
            1
        );
    }
}
