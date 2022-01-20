use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day2/large_input.txt";

fn main() {
    let movements = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let position = calculate_position(movements);

    println!("The result is: {}", position);
}

fn calculate_position(movements: Vec<String>) -> i32 {
    let mut depth = 0;
    let mut horizontal_position = 0;
    for movement in movements {
        let parsed_movement = movement.split_whitespace().collect::<Vec<&str>>();
        let movement_direction = parsed_movement[0];
        let movement_distance: i32 = parsed_movement[1].parse().unwrap();

        match movement_direction {
            "forward" => horizontal_position += movement_distance,
            "up" => depth -= movement_distance,
            "down" => depth += movement_distance,

            _ => {}
        }
    }

    return depth * horizontal_position;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day2/small_input.txt";

    #[test]
    fn should_calculate_position_with_small_input() {
        let movements = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_position(movements), 150);
    }

    #[test]
    fn should_calculate_position_with_large_input() {
        let movements = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_position(movements), 1936494);
    }
}
