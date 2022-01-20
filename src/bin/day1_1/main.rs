use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day1/large_input.txt";

fn main() {
    let distances =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Something went wrong reading the file");

    let distance_increased_count = count_increased_distances(distances);

    println!("The result is: {}", distance_increased_count);
}

fn count_increased_distances(distances: Vec<String>) -> i32 {
    let mut previous_distance = None;
    let mut distance_increased_count = 0;
    for distance_string in distances {
        let distance: i32 = distance_string.parse().unwrap();
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

        assert_eq!(count_increased_distances(distances), 7);
    }

    #[test]
    fn should_count_increased_distances_with_large_input() {
        let distances = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_increased_distances(distances), 1288);
    }
}
