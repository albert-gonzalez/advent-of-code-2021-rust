use advent_of_code_2021::common::input;
use std::cmp::max;

const LARGE_INPUT_FILE: &str = "src/assets/day5/large_input.txt";

struct Point {
    x: usize,
    y: usize,
}

struct Vent {
    start: Point,
    end: Point,
}

fn main() {
    let vents_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let overlapping_vents = count_overlapping_vents(vents_input);

    println!("The result is: {}", overlapping_vents);
}

fn count_overlapping_vents(vents_input: Vec<String>) -> i32 {
    let mut map = create_map(&vents_input);
    map = count_vents_in_coordinates(&vents_input, map);

    count_overlapping_vents_in_map(map)
}

fn create_map(vents_input: &Vec<String>) -> Vec<Vec<i32>> {
    let mut max_x = 0;
    let mut max_y = 0;
    for coordinates in vents_input {
        let parsed_coordinates: Vec<&str> = coordinates.split(" -> ").collect();
        let vent = create_vent(parsed_coordinates);

        max_x = max(max_x, max(vent.start.x, vent.end.y));
        max_y = max(max_y, max(vent.start.y, vent.end.y));
    }

    return vec![vec![0; max_y + 1]; max_y + 1];
}

fn create_vent(coordinates: Vec<&str>) -> Vent {
    let start_coordinates: Vec<usize> = coordinates[0]
        .split(",")
        .map(|coord| coord.parse().unwrap())
        .collect();
    let end_coordinates: Vec<usize> = coordinates[1]
        .split(",")
        .map(|coord| coord.parse().unwrap())
        .collect();

    Vent {
        start: Point {
            x: start_coordinates[0],
            y: start_coordinates[1],
        },
        end: Point {
            x: end_coordinates[0],
            y: end_coordinates[1],
        },
    }
}

fn count_vents_in_coordinates(vents_input: &Vec<String>, map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut cloned_map = map.clone();
    for coordinates in vents_input {
        let parsed_coordinates: Vec<&str> = coordinates.split(" -> ").collect();
        let vent = create_vent(parsed_coordinates);

        if vent.start.x == vent.end.x {
            check_vertical_vent(&mut cloned_map, &vent);
        } else if vent.start.y == vent.end.y {
            check_horizontal_vent(&mut cloned_map, &vent);
        }
    }

    cloned_map
}

fn check_vertical_vent(map: &mut Vec<Vec<i32>>, vent: &Vent) {
    let difference = vent.end.y as i32 - vent.start.y as i32;
    let start: usize = if difference > 0 {
        vent.start.y
    } else {
        vent.end.y
    };
    let end: usize = if difference > 0 {
        vent.end.y
    } else {
        vent.start.y
    };

    for i in start..=end {
        map[i][vent.start.x] += 1;
    }
}

fn check_horizontal_vent(map: &mut Vec<Vec<i32>>, vent: &Vent) {
    let difference = vent.end.x as i32 - vent.start.x as i32;
    let start: usize = if difference > 0 {
        vent.start.x
    } else {
        vent.end.x
    };
    let end: usize = if difference > 0 {
        vent.end.x
    } else {
        vent.start.x
    };

    for i in start..=end {
        map[vent.start.y][i] += 1;
    }
}

fn count_overlapping_vents_in_map(map: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for line in map {
        for vent_count in line {
            if vent_count > 1 {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day5/small_input.txt";

    #[test]
    fn should_count_who_many_vents_overlap_with_small_input() {
        let vents_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_overlapping_vents(vents_input), 5);
    }

    #[test]
    fn should_find_the_first_winning_bingo_card_with_large_input() {
        let vents_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(count_overlapping_vents(vents_input), 6283);
    }
}
