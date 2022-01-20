use advent_of_code_2021::common::input;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

const LARGE_INPUT_FILE: &str = "src/assets/day15/large_input.txt";
const MAX_RISK: i32 = 9;

fn main() {
    let chiton_caves_input =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let minimum_risk = find_path_with_minimum_risk(chiton_caves_input);

    println!("The result is: {}", minimum_risk);
}

fn find_path_with_minimum_risk(chiton_caves_input: Vec<String>) -> i32 {
    let cave_risk_map = create_cave_risk_map(&chiton_caves_input);
    let mut cave_minimum_risk_cache: Vec<Vec<Option<i32>>> =
        vec![vec![None; cave_risk_map[0].len()]; cave_risk_map.len()];

    for y in (0..cave_risk_map.len()).rev() {
        for x in (0..cave_risk_map.len()).rev() {
            let visited_caves: HashMap<(i32, i32), bool> = HashMap::new();

            cave_minimum_risk_cache[y][x] = Some(explore_cave_to_get_risk_from_point(
                &cave_risk_map,
                &mut cave_minimum_risk_cache,
                &visited_caves,
                x as i32,
                y as i32,
                x as i32,
                y as i32,
                0,
            ));
        }
    }

    cave_minimum_risk_cache[0][0].unwrap()
}

fn create_cave_risk_map(chiton_caves_input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut cave_risk_map: Vec<Vec<i32>> = Vec::new();

    for line in chiton_caves_input {
        cave_risk_map.push(
            line.chars()
                .map(|cave_risk| cave_risk.to_string().parse().unwrap())
                .collect(),
        )
    }

    let cave_length = cave_risk_map.len();

    let mut extended_cave_risk_map: Vec<Vec<i32>> = vec![vec![0; cave_length * 5]; cave_length * 5];

    for i in 0..5 {
        for j in 0..5 {
            for y in 0..cave_length {
                for x in 0..cave_length {
                    let current_y_pos = y + i * cave_length;
                    let current_x_pos = x + j * cave_length;

                    extended_cave_risk_map[current_y_pos][current_x_pos] =
                        cave_risk_map[y][x] + j as i32 + i as i32;

                    if extended_cave_risk_map[current_y_pos][current_x_pos] > MAX_RISK {
                        extended_cave_risk_map[current_y_pos][current_x_pos] =
                            extended_cave_risk_map[current_y_pos][current_x_pos] - MAX_RISK;
                    }
                }
            }
        }
    }

    extended_cave_risk_map
}

fn explore_cave_to_get_risk_from_point(
    cave_risk_map: &Vec<Vec<i32>>,
    cave_min_risk_cache: &mut Vec<Vec<Option<i32>>>,
    previous_visited_caves: &HashMap<(i32, i32), bool>,
    start_x: i32,
    start_y: i32,
    x: i32,
    y: i32,
    previous_risk: i32,
) -> i32 {
    if x < 0
        || y < 0
        || y as usize == cave_risk_map.len()
        || x as usize == cave_risk_map[y as usize].len()
    {
        return i32::MAX;
    }

    if start_x < 0
        || start_y < 0
        || start_y as usize == cave_risk_map.len()
        || start_x as usize == cave_risk_map[start_y as usize].len()
    {
        return i32::MAX;
    }

    if previous_visited_caves.get(&(x, y)).unwrap_or(&false) == &true {
        return i32::MAX;
    }

    let current_risk = if x == start_x && y == start_y {
        0
    } else {
        cave_risk_map[y as usize][x as usize] + previous_risk
    };

    let difference_x = max(start_x - x, 0);
    let difference_y = max(start_y - y, 0);
    let max_difference = max(difference_x, difference_y);

    if (difference_x > 0 && difference_y > 0)
        || (max_difference > 0
            && cave_risk_map[y as usize][x as usize] > MAX_RISK - (max_difference * 4))
    {
        //println!("{} {} {} {}", start_x, x, start_y, y);
        return i32::MAX;
    }

    if y as usize == cave_risk_map.len() - 1 && x as usize == cave_risk_map[y as usize].len() - 1 {
        return current_risk;
    }

    match cave_min_risk_cache[y as usize][x as usize] {
        Some(risk) => {
            return risk + current_risk;
        }
        None => {
            let mut current_visited_caves = previous_visited_caves.clone();
            current_visited_caves.insert((x, y), true);
            let mut minimum_risk: i32;

            minimum_risk = explore_cave_to_get_risk_from_point(
                cave_risk_map,
                cave_min_risk_cache,
                &current_visited_caves,
                start_x,
                start_y,
                x + 1,
                y,
                current_risk,
            );

            minimum_risk = min(
                minimum_risk,
                explore_cave_to_get_risk_from_point(
                    cave_risk_map,
                    cave_min_risk_cache,
                    &current_visited_caves,
                    start_x,
                    start_y,
                    x,
                    y + 1,
                    current_risk,
                ),
            );

            minimum_risk = min(
                minimum_risk,
                explore_cave_to_get_risk_from_point(
                    cave_risk_map,
                    cave_min_risk_cache,
                    &current_visited_caves,
                    start_x,
                    start_y,
                    x - 1,
                    y,
                    current_risk,
                ),
            );
            minimum_risk = min(
                minimum_risk,
                explore_cave_to_get_risk_from_point(
                    cave_risk_map,
                    cave_min_risk_cache,
                    &current_visited_caves,
                    start_x,
                    start_y,
                    x,
                    y - 1,
                    current_risk,
                ),
            );

            return minimum_risk;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day15/small_input.txt";

    #[test]
    fn should_calculate_minimum_risk_for_small_input() {
        let chiton_caves_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(find_path_with_minimum_risk(chiton_caves_input), 315);
    }

    //#[test]
    // Disabled due to performance issues
    #[allow(dead_code)]
    fn should_calculate_minimum_risk_for_large_input() {
        let chiton_caves_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(find_path_with_minimum_risk(chiton_caves_input), 3016);
    }
}
