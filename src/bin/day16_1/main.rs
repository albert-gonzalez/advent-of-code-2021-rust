use advent_of_code_2021::common::input;

const EXERCISE_INPUT_FILE: &str = "src/assets/day16/exercise_input.txt";

#[derive(Debug)]
#[allow(dead_code)]
struct Packet {
    version: usize,
    type_id: usize,
    length_type: usize,
    sub_packets_length: usize,
    sub_packets: Vec<Packet>,
    literal: u64,
}

const LITERAL_TYPE_ID: usize = 4;
const SUB_PACKET_LENGTH_BITS_TYPE: usize = 0;

const LITERAL_PART_LENGTH: usize = 5;
const TYPE_LENGTH: usize = 3;
const VERSION_LENGTH: usize = 3;
const SUB_PACKET_NUMBER_LENGTH: usize = 11;
const SUB_PACKET_BITS_LENGTH: usize = 15;

fn main() {
    let bits_hex_input = input::lines_from_file(EXERCISE_INPUT_FILE).expect("Could not load lines");

    let version_sum = sum_packets_versions(bits_hex_input);

    println!("The result is: {}", version_sum);
}

fn sum_packets_versions(bits_hex_input: Vec<String>) -> usize {
    let binary_input = transform_hex_string_to_binary(&bits_hex_input);

    let mut packets: Vec<Packet> = Vec::new();

    let (packet, _) = parse_packet(&binary_input, 0);

    packets.push(packet);

    sum_versions(&packets)
}

fn sum_versions(packets: &Vec<Packet>) -> usize {
    packets.iter().fold(0, |sum, packet| {
        sum + packet.version + sum_versions(&packet.sub_packets)
    })
}

fn parse_packet(binary_input: &String, mut current_index: usize) -> (Packet, usize) {
    let version = usize::from_str_radix(
        binary_input
            .get(current_index..current_index + VERSION_LENGTH)
            .unwrap(),
        2,
    )
    .unwrap();

    current_index += VERSION_LENGTH;

    let type_id = usize::from_str_radix(
        binary_input
            .get(current_index..current_index + TYPE_LENGTH)
            .unwrap(),
        2,
    )
    .unwrap();

    current_index += TYPE_LENGTH;

    let mut length_type = 0;
    let mut length = 0;
    let mut sub_packets: Vec<Packet> = Vec::new();
    let mut literal = 0;

    match type_id {
        LITERAL_TYPE_ID => {
            let literal_and_index = parse_literal_packet(binary_input, current_index);
            literal = literal_and_index.0;
            current_index = literal_and_index.1;
        }
        _ => {
            let response = parse_operator_packet(binary_input, current_index);
            sub_packets = response.0;
            length_type = response.1;
            length = response.2;
            current_index = response.3;
        }
    }

    (
        Packet {
            version: version,
            type_id: type_id,
            sub_packets: sub_packets,
            length_type: length_type,
            sub_packets_length: length,
            literal: literal,
        },
        current_index,
    )
}

fn parse_literal_packet(binary_input: &String, mut current_index: usize) -> (u64, usize) {
    let mut has_more_bits = true;
    let mut literal_in_bits = "".to_string();
    while has_more_bits {
        let five_bits = binary_input.get(current_index..current_index + 5).unwrap();

        has_more_bits = five_bits.get(0..1).unwrap() == "1";

        literal_in_bits += five_bits.get(1..LITERAL_PART_LENGTH).unwrap();
        current_index += LITERAL_PART_LENGTH;
    }

    (
        u64::from_str_radix(&literal_in_bits, 2).unwrap(),
        current_index,
    )
}

fn parse_operator_packet(
    binary_input: &String,
    mut current_index: usize,
) -> (Vec<Packet>, usize, usize, usize) {
    let length_type = usize::from_str_radix(
        binary_input.get(current_index..current_index + 1).unwrap(),
        2,
    )
    .unwrap();

    current_index += 1;

    let length;
    let mut sub_packets = Vec::new();

    match length_type {
        SUB_PACKET_LENGTH_BITS_TYPE => {
            length = usize::from_str_radix(
                binary_input
                    .get(current_index..current_index + SUB_PACKET_BITS_LENGTH)
                    .unwrap(),
                2,
            )
            .unwrap();
            current_index += SUB_PACKET_BITS_LENGTH;
            let sub_packets_end_index = current_index + length as usize;
            while current_index < sub_packets_end_index {
                let (sub_packet, new_index) = parse_packet(binary_input, current_index);
                current_index = new_index;
                sub_packets.push(sub_packet);
            }
        }

        _ => {
            length = usize::from_str_radix(
                binary_input
                    .get(current_index..current_index + SUB_PACKET_NUMBER_LENGTH)
                    .unwrap(),
                2,
            )
            .unwrap();
            current_index += SUB_PACKET_NUMBER_LENGTH;
            for _i in 0..length {
                let (sub_packet, new_index) = parse_packet(binary_input, current_index);
                current_index = new_index;
                sub_packets.push(sub_packet);
            }
        }
    }

    (sub_packets, length_type, length, current_index)
}

fn transform_hex_string_to_binary(bits_hex_input: &Vec<String>) -> String {
    bits_hex_input[0].chars().fold("".to_string(), |bits, hex| {
        bits + transform_hex_char_to_binary(hex)
    })
}

fn transform_hex_char_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_the_packet_versions_for_large_input() {
        let hex_input = input::lines_from_file(EXERCISE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(sum_packets_versions(hex_input), 897);
    }
}
