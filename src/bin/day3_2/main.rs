use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day3/large_input.txt";

fn main() {
    let binary_numbers = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let report = calculate_report(binary_numbers);

    println!("The result is: {}", report);
}

fn calculate_report(binary_numbers: Vec<String>) -> i32 {
    let oxygen_rating =
        calculate_rating_by_bit_comparison(binary_numbers.clone(), |one_count, zero_count| {
            one_count >= zero_count
        });
    let scrubber_rating =
        calculate_rating_by_bit_comparison(binary_numbers.clone(), |one_count, zero_count| {
            one_count < zero_count
        });

    return oxygen_rating * scrubber_rating;
}

fn calculate_rating_by_bit_comparison(
    binary_numbers: Vec<String>,
    comparison_fn: fn(i32, i32) -> bool,
) -> i32 {
    let mut rating_candidates = binary_numbers.clone();

    for i in 0..binary_numbers[0].chars().count() {
        let mut zero_counts = Vec::new();
        let mut one_counts = Vec::new();

        for _ in rating_candidates[0].chars() {
            zero_counts.push(0);
            one_counts.push(0);
        }

        for binary_number in &rating_candidates {
            for (i, digit) in binary_number.chars().enumerate() {
                match digit {
                    '1' => one_counts[i] += 1,
                    _ => zero_counts[i] += 1,
                }
            }
        }

        let mut new_rating_candidates: Vec<String> = Vec::new();
        let most_common_value = if comparison_fn(one_counts[i], zero_counts[i]) {
            '1'
        } else {
            '0'
        };

        for binary_number in rating_candidates {
            if binary_number.chars().nth(i).unwrap() == most_common_value {
                new_rating_candidates.push(binary_number.to_owned());
            }
        }
        rating_candidates = new_rating_candidates;

        if rating_candidates.len() == 1 {
            return i32::from_str_radix(&rating_candidates[0], 2).unwrap();
        }
    }

    panic!("Rating not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day3/small_input.txt";

    #[test]
    fn should_calculate_report_with_small_input() {
        let binary_numbers = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_report(binary_numbers), 230);
    }

    #[test]
    fn should_calculate_report_with_large_input() {
        let binary_numbers = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_report(binary_numbers), 1007985);
    }
}
