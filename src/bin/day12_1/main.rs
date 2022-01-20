use advent_of_code_2021::common::input;
use std::collections::HashMap;

const EXERCISE_INPUT_FILE: &str = "src/assets/day12/exercise_input.txt";

#[derive(Debug)]
enum CaveType {
    START,
    END,
    SMALL,
    BIG,
}

#[derive(Debug)]
struct Cave {
    cave_type: CaveType,
    linked_caves: Vec<String>,
}

const START_TOKEN: &str = "start";
const END_TOKEN: &str = "end";

fn main() {
    let caves_input = input::lines_from_file(EXERCISE_INPUT_FILE).expect("Could not load lines");

    let paths_count = count_possible_paths(caves_input);

    println!("The result is: {}", paths_count);
}

fn count_possible_paths(caves_input: Vec<String>) -> i32 {
    let caves = create_caves_map(&caves_input);

    follow_and_count_paths(&caves, &Vec::new(), START_TOKEN.to_string())
}

fn create_caves_map(caves_input: &Vec<String>) -> HashMap<String, Cave> {
    let mut caves: HashMap<String, Cave> = HashMap::new();

    for input in caves_input {
        let linked_caves: Vec<&str> = input.split("-").collect();

        create_or_update_cave(&mut caves, linked_caves[0], linked_caves[1]);
        create_or_update_cave(&mut caves, linked_caves[1], linked_caves[0]);
    }

    caves
}

fn create_or_update_cave(
    caves: &mut HashMap<String, Cave>,
    cave_name: &str,
    linked_cave_name: &str,
) {
    let first_cave = caves.get_mut(cave_name);
    match first_cave {
        Some(cave) => cave.linked_caves.push(linked_cave_name.to_string()),
        None => {
            caves.insert(
                cave_name.to_string(),
                Cave {
                    cave_type: get_cave_type(cave_name),
                    linked_caves: Vec::from([linked_cave_name.to_string()]),
                },
            );
        }
    }
}

fn get_cave_type(cave_name: &str) -> CaveType {
    let chars: Vec<char> = cave_name.chars().collect();
    if chars[0].to_uppercase().next().unwrap() == chars[0] {
        return CaveType::BIG;
    }

    if cave_name == START_TOKEN {
        return CaveType::START;
    }

    if cave_name == END_TOKEN {
        return CaveType::END;
    }

    CaveType::SMALL
}

fn follow_and_count_paths(
    caves: &HashMap<String, Cave>,
    visited_caves: &Vec<String>,
    current_cave_name: String,
) -> i32 {
    let current_cave = caves.get(&current_cave_name).unwrap();

    if visited_caves.len() > 0 && matches!(current_cave.cave_type, CaveType::START) {
        return 0;
    }

    if visited_caves.contains(&current_cave_name)
        && matches!(current_cave.cave_type, CaveType::SMALL)
    {
        return 0;
    }

    if visited_caves.last() == Some(&current_cave_name) {
        return 0;
    }

    if matches!(current_cave.cave_type, CaveType::END) {
        return 1;
    }

    let mut new_visited_caves = visited_caves.clone();
    new_visited_caves.push(current_cave_name.clone());

    current_cave
        .linked_caves
        .iter()
        .fold(0, |path_count, linked_cave| {
            path_count + follow_and_count_paths(&caves, &new_visited_caves, linked_cave.clone())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_FILE: &str = "src/assets/day12/example_input.txt";

    #[test]
    fn should_count_possible_paths_for_example_input() {
        let caves_input = input::lines_from_file(EXAMPLE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_possible_paths(caves_input), 10);
    }

    #[test]
    fn should_count_possible_paths_for_exercise_input() {
        let caves_input = input::lines_from_file(EXERCISE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_possible_paths(caves_input), 5252);
    }
}
