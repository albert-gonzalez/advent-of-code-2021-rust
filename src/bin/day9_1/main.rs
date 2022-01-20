use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day9/large_input.txt";

fn main() {
    let lava_tubes_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let risk_levels_sum = sum_risk_levels(lava_tubes_input);

    println!("The result is: {}", risk_levels_sum);
}

fn sum_risk_levels(lava_tubes_input: Vec<String>) -> i32 {
    let map = create_lava_tubes_map(&lava_tubes_input);
    let risk_levels = find_risk_levels(&map);

    risk_levels
        .iter()
        .fold(0, |count, risk_level| count + risk_level)
}

fn create_lava_tubes_map(lava_tubes_input: &Vec<String>) -> Vec<Vec<i32>> {
    lava_tubes_input
        .iter()
        .map(|lave_tube| {
            lave_tube
                .chars()
                .map(|value| value.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn find_risk_levels(map: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut risk_levels = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let current_position = map[i][j];

            if is_the_lowest_adjacent_level(i, j, map) {
                risk_levels.push(current_position + 1);
            }
        }
    }

    risk_levels
}

fn is_the_lowest_vertical_adjacent_level(i: usize, j: usize, map: &Vec<Vec<i32>>) -> bool {
    (i == 0 || i > 0 && map[i - 1][j] > map[i][j])
        && (i == map.len() - 1 || i < map.len() - 1 && map[i + 1][j] > map[i][j])
}

fn is_the_lowest_horizontal_adjacent_level(i: usize, j: usize, map: &Vec<Vec<i32>>) -> bool {
    (j == 0 || j > 0 && map[i][j - 1] > map[i][j])
        && (j == map[i].len() - 1 || j < map[i].len() - 1 && map[i][j + 1] > map[i][j])
}

fn is_the_lowest_adjacent_level(i: usize, j: usize, map: &Vec<Vec<i32>>) -> bool {
    is_the_lowest_vertical_adjacent_level(i, j, map)
        && is_the_lowest_horizontal_adjacent_level(i, j, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day9/small_input.txt";

    #[test]
    fn should_sum_risk_levels_for_small_input() {
        let lava_tubes_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(sum_risk_levels(lava_tubes_input), 15);
    }

    #[test]
    fn should_sum_risk_levels_for_large_input() {
        let lava_tubes_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(sum_risk_levels(lava_tubes_input), 580);
    }
}
