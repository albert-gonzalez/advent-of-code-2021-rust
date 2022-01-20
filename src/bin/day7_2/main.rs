use advent_of_code_2021::common::input;
use std::cmp::max;

const LARGE_INPUT_FILE: &str = "src/assets/day7/large_input.txt";

fn main() {
    let crabs_positions_input =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let minimum_spent_fuel = calculate_minimum_spent_fuel(crabs_positions_input);

    println!("The result is: {}", minimum_spent_fuel);
}

fn calculate_minimum_spent_fuel(lanternfish_input: Vec<String>) -> i32 {
    let crabs_positions = create_crabs_positions(&lanternfish_input);

    let max_position = crabs_positions
        .iter()
        .fold(0, |max_position, position| max(max_position, *position));
    let mut minimum_fuel = None;

    for i in 0..=max_position {
        let spent_fuel = crabs_positions.iter().fold(0, |spent_fuel, position| {
            let summation = (i32::pow(i32::abs(position - i), 2) + i32::abs(position - i)) / 2;
            spent_fuel + summation
        });

        if minimum_fuel == None || minimum_fuel.unwrap() > spent_fuel {
            minimum_fuel = Some(spent_fuel);
        } else if minimum_fuel.unwrap() < spent_fuel {
            break;
        }
    }
    minimum_fuel.unwrap()
}

fn create_crabs_positions(crabs_positions_input: &Vec<String>) -> Vec<i32> {
    crabs_positions_input[0]
        .split(",")
        .map(|state| state.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day7/small_input.txt";

    #[test]
    fn should_calculate_minimum_spent_fuel_for_small_input() {
        let crabs_positions_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_minimum_spent_fuel(crabs_positions_input), 168);
    }

    #[test]
    fn should_calculate_minimum_spent_fuel_for_large_input() {
        let crabs_positions_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_minimum_spent_fuel(crabs_positions_input),
            98231647
        );
    }
}
