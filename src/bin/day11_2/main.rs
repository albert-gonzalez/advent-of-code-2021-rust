use advent_of_code_2021::common::input;

const EXERCISE_INPUT_FILE: &str = "src/assets/day11/exercise_input.txt";

fn main() {
    let dumbo_octopuses_input =
        input::lines_from_file(EXERCISE_INPUT_FILE).expect("Could not load lines");

    let step = find_step_when_all_octopuses_flash(dumbo_octopuses_input);

    println!("The result is: {}", step);
}

fn find_step_when_all_octopuses_flash(dumbo_octopuses_input: Vec<String>) -> i32 {
    let mut energy_map = create_energy_map(&dumbo_octopuses_input);

    let mut step = 1;
    while simulate_step(&mut energy_map) < 100 {
        step += 1;
    }
    step
}

fn create_energy_map(dumbo_octopuses_input: &Vec<String>) -> Vec<Vec<i32>> {
    dumbo_octopuses_input
        .iter()
        .map(|line| {
            line.chars()
                .map(|level| level.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn simulate_step(energy_map: &mut Vec<Vec<i32>>) -> i32 {
    let mut flashed_octopus: Vec<Vec<bool>> =
        vec![vec![false; energy_map[0].len()]; energy_map.len()];
    let mut flashes_count = 0;

    for y in 0..energy_map.len() {
        for x in 0..energy_map[y].len() {
            flashes_count += increase_energy_and_flash_if_needed(
                energy_map,
                &mut flashed_octopus,
                x as i32,
                y as i32,
            );
        }
    }

    flashes_count
}

fn increase_energy_and_flash_if_needed(
    energy_map: &mut Vec<Vec<i32>>,
    flashed_octopus: &mut Vec<Vec<bool>>,
    x: i32,
    y: i32,
) -> i32 {
    if y < 0 || y as usize == energy_map.len() {
        return 0;
    }

    if x < 0 || x as usize == energy_map[y as usize].len() {
        return 0;
    }

    if flashed_octopus[y as usize][x as usize] == true {
        return 0;
    }

    energy_map[y as usize][x as usize] += 1;

    if energy_map[y as usize][x as usize] > 9 {
        energy_map[y as usize][x as usize] = 0;
        flashed_octopus[y as usize][x as usize] = true;

        return increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x - 1, y)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x - 1, y - 1)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x, y - 1)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x + 1, y - 1)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x + 1, y)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x + 1, y + 1)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x, y + 1)
            + increase_energy_and_flash_if_needed(energy_map, flashed_octopus, x - 1, y + 1)
            + 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_FILE: &str = "src/assets/day11/example_input.txt";

    #[test]
    fn should_calculate_octopus_flashes_after_100_steps_for_example_input() {
        let dumbo_octopuses_input = input::lines_from_file(EXAMPLE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            find_step_when_all_octopuses_flash(dumbo_octopuses_input),
            195
        );
    }

    #[test]
    fn should_calculate_octopus_flashes_after_100_steps_for_exercise_input() {
        let dumbo_octopuses_input = input::lines_from_file(EXERCISE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            find_step_when_all_octopuses_flash(dumbo_octopuses_input),
            298
        );
    }
}
