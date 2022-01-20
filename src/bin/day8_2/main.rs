use advent_of_code_2021::common::input;
use std::collections::HashMap;

const LARGE_INPUT_FILE: &str = "src/assets/day8/large_input.txt";

#[derive(Debug)]
struct Entry {
    digits: i32,
}

#[derive(Debug)]
struct Signal {
    code: String,
    value: String,
}

fn main() {
    let entries_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let valid_digit_count = sum_digits(entries_input);

    println!("The result is: {}", valid_digit_count);
}

fn sum_digits(entries_input: Vec<String>) -> i32 {
    let entries = create_entries(&entries_input);
    sum_entries_digits(entries)
}

fn create_entries(entries_input: &Vec<String>) -> Vec<Entry> {
    let mut entries = Vec::new();

    for input in entries_input {
        let splitted_entry: Vec<&str> = input.split(" | ").collect();

        let signals: Vec<&str> = splitted_entry[0].split_whitespace().collect();
        let mut decoded_signals = HashMap::new();

        decode_digits_with_unique_number_of_segments(&mut decoded_signals, &signals);

        decode_digit_3(&mut decoded_signals, &signals);

        decode_digits_with_6_segments(&mut decoded_signals, &signals);

        decode_digits_2_and_5(&mut decoded_signals, &signals);

        let digits = transform_digits_to_integer(splitted_entry[1], &decoded_signals);

        entries.push(Entry { digits: digits });
    }

    entries
}

fn decode_digits_with_unique_number_of_segments(
    decoded_signals: &mut HashMap<String, Signal>,
    signals: &Vec<&str>,
) {
    for signal_code in signals {
        if signal_code.len() == 2 {
            let value = String::from("1");
            decoded_signals.insert(
                value.clone(),
                Signal {
                    code: signal_code.to_string(),
                    value: value,
                },
            );
        }

        if signal_code.len() == 3 {
            let value = String::from("7");

            decoded_signals.insert(
                value.clone(),
                Signal {
                    code: signal_code.to_string(),
                    value: value,
                },
            );
        }

        if signal_code.len() == 4 {
            let value = String::from("4");

            decoded_signals.insert(
                value.clone(),
                Signal {
                    code: signal_code.to_string(),
                    value: value,
                },
            );
        }

        if signal_code.len() == 7 {
            let value = String::from("8");

            decoded_signals.insert(
                value.clone(),
                Signal {
                    code: signal_code.to_string(),
                    value: value,
                },
            );
        }
    }
}

fn decode_digit_3(decoded_signals: &mut HashMap<String, Signal>, signals: &Vec<&str>) {
    for signal_code in signals {
        if signal_code.len() == 5 {
            let contains_1_letters = decoded_signals
                .get("1")
                .unwrap()
                .code
                .chars()
                .all(|letter| signal_code.contains(letter));
            let value;

            if contains_1_letters {
                value = String::from("3");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );

                continue;
            }
        }
    }
}

fn decode_digits_with_6_segments(
    decoded_signals: &mut HashMap<String, Signal>,
    signals: &Vec<&str>,
) {
    for signal_code in signals {
        if signal_code.len() == 6 {
            let contains_3_letters = decoded_signals
                .get("3")
                .unwrap()
                .code
                .chars()
                .all(|letter| signal_code.contains(letter));
            let contains_1_letters = decoded_signals
                .get("1")
                .unwrap()
                .code
                .chars()
                .all(|letter| signal_code.contains(letter));

            let value;

            if contains_3_letters {
                value = String::from("9");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );
            } else if contains_1_letters {
                value = String::from("0");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );
            } else {
                value = String::from("6");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );
            }
        }
    }
}

fn decode_digits_2_and_5(decoded_signals: &mut HashMap<String, Signal>, signals: &Vec<&str>) {
    let missing_letter_in_9 = get_missing_letter_in_9(decoded_signals);
    for signal_code in signals {
        if signal_code.len() == 5 {
            let contains_1_letters = decoded_signals
                .get("1")
                .unwrap()
                .code
                .chars()
                .all(|letter| signal_code.contains(letter));

            if contains_1_letters {
                continue;
            }

            if signal_code.contains(&missing_letter_in_9) {
                let value = String::from("2");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );
            } else {
                let value = String::from("5");
                decoded_signals.insert(
                    value.clone(),
                    Signal {
                        code: signal_code.to_string(),
                        value: value,
                    },
                );
            }
        }
    }
}

fn get_missing_letter_in_9(decoded_signals: &HashMap<String, Signal>) -> String {
    let signal_code = &decoded_signals.get("9").unwrap().code;

    decoded_signals
        .get("8")
        .unwrap()
        .code
        .chars()
        .find(|letter| !signal_code.contains(&letter.to_string()))
        .unwrap()
        .to_string()
}

fn transform_digits_to_integer(
    coded_digits: &str,
    decoded_signals: &HashMap<String, Signal>,
) -> i32 {
    coded_digits
        .split_whitespace()
        .fold(String::from(""), |digits, code| {
            digits
                + &decoded_signals
                    .values()
                    .find(|signal| {
                        code.len() == signal.code.len()
                            && code.chars().all(|letter| signal.code.contains(letter))
                    })
                    .unwrap()
                    .value
        })
        .parse()
        .unwrap()
}

fn sum_entries_digits(entries: Vec<Entry>) -> i32 {
    entries.iter().fold(0, |count, entry| count + entry.digits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day8/small_input.txt";

    #[test]
    fn should_sum_entries_digits_for_small_input() {
        let entries_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(sum_digits(entries_input), 61229);
    }

    #[test]
    fn should_sum_entries_digits_for_large_input() {
        let entries_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(sum_digits(entries_input), 978171);
    }
}
