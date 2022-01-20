use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day3/large_input.txt";

fn main() {
    let binary_numbers = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let report = calculate_report(binary_numbers);

    println!("The result is: {}", report);
}

fn calculate_report(binary_numbers: Vec<String>) -> i32 {
    let mut zero_counts = Vec::new();
    let mut one_counts = Vec::new();

    for _ in binary_numbers[0].chars() {
        zero_counts.push(0);
        one_counts.push(0);
    }

    for binary_number in binary_numbers {
        for (i, digit) in binary_number.chars().enumerate() {
            match digit {
                '1' => one_counts[i] += 1,
                _ => zero_counts[i] += 1,
            }
        }
    }

    let mut gamma_rate_vec = Vec::new();
    let mut epsilon_rate_vec = Vec::new();

    for i in 0..zero_counts.len() {
        if zero_counts[i] > one_counts[i] {
            gamma_rate_vec.push("0");
            epsilon_rate_vec.push("1");
        } else {
            gamma_rate_vec.push("1");
            epsilon_rate_vec.push("0");
        }
    }

    let gamma_rate = i32::from_str_radix(&gamma_rate_vec.join(""), 2).unwrap();
    let epsilon_rate = i32::from_str_radix(&epsilon_rate_vec.join(""), 2).unwrap();

    return gamma_rate * epsilon_rate;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day3/small_input.txt";

    #[test]
    fn should_calculate_report_with_small_input() {
        let binary_numbers = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_report(binary_numbers), 198);
    }

    #[test]
    fn should_calculate_report_with_large_input() {
        let binary_numbers = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_report(binary_numbers), 852500);
    }
}
