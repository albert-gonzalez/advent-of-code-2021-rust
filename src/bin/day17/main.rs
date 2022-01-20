use advent_of_code_2021::common::input;
use std::cmp::max;
use std::cmp::min;

use std::collections::HashMap;

const EXERCISE_INPUT_FILE: &str = "src/assets/day17/exercise_input.txt";

#[derive(Debug)]
struct TargetArea {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
}

fn main() {
    let target_area_input =
        input::lines_from_file(EXERCISE_INPUT_FILE).expect("Could not load lines");

    let (max_y, possible_velocities_count) =
        find_max_y_and_count_starting_velocities_reaching_the_target(target_area_input);

    println!(
        "The Max Y is : {} and the possible starting velocities count is: {}",
        max_y, possible_velocities_count
    );
}

fn find_max_y_and_count_starting_velocities_reaching_the_target(
    target_area_input: Vec<String>,
) -> (i32, usize) {
    let target_area = parse_target_area(&target_area_input);

    let mut possible_start_velocities: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut max_y = 0;

    for start_x_velocity in 0..=target_area.end_x {
        for start_y_velocity in min(target_area.start_y, target_area.end_y)
            ..max(i32::abs(target_area.start_y), i32::abs(target_area.end_y))
        {
            let mut y_for_step = 0;
            let mut x_for_step = 0;
            let mut current_y_velocity = start_y_velocity;
            let mut current_x_velocity;
            let mut current_step = 0;
            let mut max_y_for_step = 0;

            if current_y_velocity == 0 {
                max_y_for_step = y_for_step;
            }
            while current_y_velocity >= 0
                || (current_y_velocity < 0 && y_for_step > target_area.start_y)
            {
                current_step += 1;
                current_y_velocity = start_y_velocity - current_step + 1;
                current_x_velocity = max(start_x_velocity - (current_step - 1), 0);
                y_for_step += start_y_velocity - current_step + 1;
                x_for_step += current_x_velocity;

                if current_y_velocity == 0 {
                    max_y_for_step = y_for_step;
                }

                if y_for_step >= target_area.start_y
                    && y_for_step <= target_area.end_y
                    && x_for_step >= target_area.start_x
                    && x_for_step <= target_area.end_x
                {
                    possible_start_velocities.insert(
                        (start_x_velocity, start_y_velocity),
                        (x_for_step, y_for_step),
                    );
                    max_y = max(max_y, max_y_for_step);
                }
            }
        }
    }

    (max_y, possible_start_velocities.len())
}

fn parse_target_area(target_area_input: &Vec<String>) -> TargetArea {
    let x_and_y: Vec<&str> = target_area_input[0].split(":").collect::<Vec<&str>>()[1]
        .split(",")
        .collect();
    let x_range: Vec<i32> = x_and_y[0].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|value| value.parse().unwrap())
        .collect();
    let y_range: Vec<i32> = x_and_y[1].split("=").collect::<Vec<&str>>()[1]
        .split("..")
        .map(|value| value.parse().unwrap())
        .collect();

    TargetArea {
        start_x: x_range[0],
        end_x: x_range[1],
        start_y: y_range[0],
        end_y: y_range[1],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_FILE: &str = "src/assets/day17/example_input.txt";

    #[test]
    fn should_find_the_max_y_and_count_the_possible_start_velocities_for_example_input() {
        let target_area_input = input::lines_from_file(EXAMPLE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            find_max_y_and_count_starting_velocities_reaching_the_target(target_area_input),
            (45, 112)
        );
    }

    #[test]
    fn should_find_the_max_y_and_count_the_possible_start_velocities_for_exercise_input() {
        let target_area_input = input::lines_from_file(EXERCISE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            find_max_y_and_count_starting_velocities_reaching_the_target(target_area_input),
            (11175, 3540)
        );
    }
}
