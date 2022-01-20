use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day1/large_input.txt";

fn main() {
    let distances = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let distance_increased_count = count_3_window_measurement_increased_distances(distances);

    println!("The result is: {}", distance_increased_count);
}

fn count_3_window_measurement_increased_distances(distances: Vec<String>) -> i32 {
    let three_measurements_window_distances = generate_3_measurement_window_distances(distances);

    return count_increased_distances(three_measurements_window_distances);
}

fn generate_3_measurement_window_distances(distances: Vec<String>) -> Vec<i32> {
    let mut three_measurements_window_distances = Vec::new();
    let default_distance = String::from("0");

    for (i, distance_string) in distances.iter().enumerate() {
        let distance: i32 = distance_string.parse().unwrap();
        let next_distance: i32 = distances
            .get(i + 1)
            .unwrap_or(&default_distance)
            .parse()
            .unwrap();
        let next_next_distance: i32 = distances
            .get(i + 2)
            .unwrap_or(&default_distance)
            .parse()
            .unwrap();

        three_measurements_window_distances.push(distance + next_distance + next_next_distance);
    }

    return three_measurements_window_distances;
}

fn count_increased_distances(distances: Vec<i32>) -> i32 {
    let mut previous_distance = None;
    let mut distance_increased_count = 0;

    for distance in distances {
        if let Some(v) = previous_distance {
            if distance > v {
                distance_increased_count += 1;
            }
        }

        previous_distance = Some(distance);
    }

    return distance_increased_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day1/small_input.txt";

    #[test]
    fn should_count_increased_distances_with_small_input() {
        let distances = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_3_window_measurement_increased_distances(distances), 5);
    }

    #[test]
    fn should_count_increased_distances_with_large_input() {
        let distances = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            count_3_window_measurement_increased_distances(distances),
            1311
        );
    }
}
