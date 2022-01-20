use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day9/large_input.txt";

struct RiskLevelPosition {
    x: i32,
    y: i32,
}

fn main() {
    let lava_tubes_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let risk_levels_sum = multiply_the_three_largest_basins(lava_tubes_input);

    println!("The result is: {}", risk_levels_sum);
}

fn multiply_the_three_largest_basins(lava_tubes_input: Vec<String>) -> i32 {
    let map = create_lava_tubes_map(&lava_tubes_input);
    let risk_levels_positions = find_risk_levels_positions(&map);

    let mut basins_sizes = find_basins_sizes(&map, &risk_levels_positions);
    basins_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());

    basins_sizes[0..=2]
        .iter()
        .fold(1, |count, basin_size| count * basin_size)
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

fn find_risk_levels_positions(map: &Vec<Vec<i32>>) -> Vec<RiskLevelPosition> {
    let mut risk_level_positions = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if is_the_lowest_adjacent_level(i, j, map) {
                risk_level_positions.push(RiskLevelPosition {
                    x: j as i32,
                    y: i as i32,
                });
            }
        }
    }

    risk_level_positions
}

fn find_basins_sizes(
    map: &Vec<Vec<i32>>,
    risk_levels_positions: &Vec<RiskLevelPosition>,
) -> Vec<i32> {
    risk_levels_positions
        .iter()
        .map(|risk_level_position| {
            let x = risk_level_position.x;
            let y = risk_level_position.y;
            let mut already_added_positions = vec![vec![false; map[0].len()]; map.len()];

            find_basin_size(y, x, map, &mut already_added_positions)
        })
        .collect()
}

fn find_basin_size(
    y: i32,
    x: i32,
    map: &Vec<Vec<i32>>,
    already_added_positions: &mut Vec<Vec<bool>>,
) -> i32 {
    if y < 0 || y as usize == map.len() {
        return 0;
    }

    if x < 0 || x as usize == map[y as usize].len() {
        return 0;
    }

    if map[y as usize][x as usize] == 9 {
        return 0;
    }

    if already_added_positions[y as usize][x as usize] == true {
        return 0;
    }

    already_added_positions[y as usize][x as usize] = true;

    find_basin_size(y - 1, x, map, already_added_positions)
        + find_basin_size(y, x - 1, map, already_added_positions)
        + find_basin_size(y + 1, x, map, already_added_positions)
        + find_basin_size(y, x + 1, map, already_added_positions)
        + 1
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
    fn should_multiply_the_three_largest_basins_for_small_input() {
        let lava_tubes_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(multiply_the_three_largest_basins(lava_tubes_input), 1134);
    }

    #[test]
    fn should_multiply_the_three_largest_basins_for_large_input() {
        let lava_tubes_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(multiply_the_three_largest_basins(lava_tubes_input), 856716);
    }
}
