use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day8/large_input.txt";

#[derive(Debug)]
struct Entry {
    digits: Vec<Option<i32>>,
}

fn main() {
    let entries_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let valid_digit_count = count_valid_digits(entries_input);

    println!("The result is: {}", valid_digit_count);
}

fn count_valid_digits(entries_input: Vec<String>) -> i32 {
    let entries = create_entries(&entries_input);

    entries.iter().fold(0, |count, entry| {
        count
            + entry.digits.iter().fold(
                0,
                |count, digit| if *digit != None { count + 1 } else { count },
            )
    })
}

fn create_entries(entries_input: &Vec<String>) -> Vec<Entry> {
    let mut entries = Vec::new();

    for input in entries_input {
        let splitted_entry: Vec<&str> = input.split(" | ").collect();
        let entry = Entry {
            digits: splitted_entry[1]
                .split_whitespace()
                .map(|code| {
                    if code.len() == 2 {
                        return Some(1);
                    }

                    if code.len() == 3 {
                        return Some(7);
                    }

                    if code.len() == 4 {
                        return Some(4);
                    }

                    if code.len() == 7 {
                        return Some(8);
                    }

                    None
                })
                .collect(),
        };
        entries.push(entry);
    }

    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day8/small_input.txt";

    #[test]
    fn should_count_digits_with_unique_number_of_segments_for_small_input() {
        let entries_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_valid_digits(entries_input), 26);
    }

    #[test]
    fn should_count_digits_with_unique_number_of_segments_for_large_input() {
        let entries_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_valid_digits(entries_input), 412);
    }
}
