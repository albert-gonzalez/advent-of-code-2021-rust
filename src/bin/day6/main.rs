use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day6/large_input.txt";

fn main() {
    let lanternfish_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let fish_count = count_fishes_in_n_days(lanternfish_input, 256);

    println!("The result is: {}", fish_count);
}

fn count_fishes_in_n_days(lanternfish_input: Vec<String>, days: u32) -> i64 {
    let mut lanterfishes_state = create_laternfishes_state(&lanternfish_input);
    let mut fishes_to_spawn_on_next_day = 0;

    for _i in 1..=days {
        let mut new_state: Vec<i64> = vec![0; 9];

        if fishes_to_spawn_on_next_day > 0 {
            new_state[8] = fishes_to_spawn_on_next_day;
            fishes_to_spawn_on_next_day = 0;
        }

        for i in (0..=8).rev() {
            if i == 0 {
                fishes_to_spawn_on_next_day = new_state[0];
                new_state[6] += lanterfishes_state[0];
                break;
            }

            new_state[i - 1] = lanterfishes_state[i];
        }

        lanterfishes_state = new_state;
    }

    lanterfishes_state
        .iter()
        .fold(0, |count, fishes| count + fishes)
}

fn create_laternfishes_state(lanternfish_input: &Vec<String>) -> Vec<i64> {
    let mut state = vec![0; 9];

    for timer in lanternfish_input[0]
        .split(",")
        .map(|state| state.parse::<usize>().unwrap())
    {
        state[timer] += 1;
    }

    state
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day6/small_input.txt";

    #[test]
    fn should_count_lanternfishes_spawned_in_80_days_with_small_input() {
        let lanternfishes_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_fishes_in_n_days(lanternfishes_input, 80), 5934);
    }

    #[test]
    fn should_count_lanternfishes_spawned_in_80_days_with_large_input() {
        let lanternfishes_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_fishes_in_n_days(lanternfishes_input, 80), 359999);
    }

    #[test]
    fn should_count_lanternfishes_spawned_in_256_days_with_small_input() {
        let lanternfishes_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            count_fishes_in_n_days(lanternfishes_input, 256),
            26984457539
        );
    }

    #[test]
    fn should_count_lanternfishes_spawned_in_256_days_with_large_input() {
        let lanternfishes_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            count_fishes_in_n_days(lanternfishes_input, 256),
            1631647919273
        );
    }
}
