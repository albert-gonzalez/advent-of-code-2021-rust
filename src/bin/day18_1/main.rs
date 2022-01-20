use advent_of_code_2021::common::input;
use std::cmp::max;

const EXERCISE_INPUT_FILE: &str = "src/assets/day18/exercise_input.txt";

#[derive(Debug, Clone)]
struct SnailFishNumber {
    x: SnailFishNumberOption,
    y: SnailFishNumberOption,
}

#[derive(Debug)]
enum SnailFishNumberOption {
    Complex(Box<SnailFishNumber>),
    Literal(i32),
}

impl Clone for SnailFishNumberOption {
    fn clone(&self) -> Self {
        match self {
            SnailFishNumberOption::Complex(boxed) => {
                SnailFishNumberOption::Complex(Box::new(SnailFishNumber {
                    x: boxed.x.clone(),
                    y: boxed.y.clone(),
                }))
            }
            SnailFishNumberOption::Literal(literal) => SnailFishNumberOption::Literal(*literal),
        }
    }
}

impl SnailFishNumber {
    fn magnitude(&self) -> i32 {
        let magnitude_x: i32;
        let magnitude_y: i32;
        magnitude_x = 3 * match &self.x {
            SnailFishNumberOption::Complex(number) => number.magnitude(),
            SnailFishNumberOption::Literal(number) => *number,
        };

        magnitude_y = 2 * match &self.y {
            SnailFishNumberOption::Complex(number) => number.magnitude(),
            SnailFishNumberOption::Literal(number) => *number,
        };

        magnitude_x + magnitude_y
    }

    fn add(&self, other: &SnailFishNumber) -> SnailFishNumber {
        let mut snailfish_number = SnailFishNumber {
            x: SnailFishNumberOption::Complex(Box::new(self.clone())),
            y: SnailFishNumberOption::Complex(Box::new(other.clone())),
        };
        snailfish_number.reduce();
        snailfish_number
    }

    fn reduce(&mut self) {
        let mut iterate = true;
        while iterate {
            let (_, _, exploded) = self.explode(0);
            let mut splitted = false;
            if !exploded {
                splitted = self.split();
            }
            iterate = exploded || splitted;
        }
    }

    fn explode(&mut self, depth: i32) -> (Option<i32>, Option<i32>, bool) {
        if depth == 4 {
            if let SnailFishNumberOption::Literal(x) = self.x {
                if let SnailFishNumberOption::Literal(y) = self.y {
                    return (Some(x), Some(y), true);
                }
            }
            panic!("Not literals in depth 4");
        }
        let mut exploded_x: Option<i32> = None;
        let mut exploded_y: Option<i32> = None;
        let mut exploded = false;
        if let SnailFishNumberOption::Complex(box_number) = &mut self.x {
            let number: &mut SnailFishNumber = &mut *box_number;
            let (exploded_x_i, exploded_y_i, exploded_i) = number.explode(depth + 1);
            exploded_x = exploded_x_i;
            exploded_y = exploded_y_i;
            exploded = exploded_i;
        }

        if exploded_x != None && exploded_y != None {
            self.x = SnailFishNumberOption::Literal(0);
        }

        if let Some(number) = exploded_y {
            add_to_first_left_number(number, &mut self.y);
            return (exploded_x, None, exploded);
        }

        if exploded {
            return (exploded_x, exploded_y, exploded);
        }

        if let SnailFishNumberOption::Complex(box_number) = &mut self.y {
            let number: &mut SnailFishNumber = &mut *box_number;
            let (exploded_x_i, exploded_y_i, exploded_i) = number.explode(depth + 1);
            exploded_x = exploded_x_i;
            exploded_y = exploded_y_i;
            exploded = exploded_i;
        }

        if exploded_x != None && exploded_y != None {
            self.y = SnailFishNumberOption::Literal(0);
        }

        if let Some(number) = exploded_x {
            add_to_first_right_number(number, &mut self.x);
            return (None, exploded_y, exploded);
        }
        (exploded_x, exploded_y, exploded)
    }

    fn split(&mut self) -> bool {
        let mut splitted = false;

        match &mut self.x {
            SnailFishNumberOption::Complex(box_number) => {
                let number: &mut SnailFishNumber = &mut *box_number;
                splitted = number.split();
            }
            SnailFishNumberOption::Literal(literal) => {
                if *literal > 9 {
                    self.x = SnailFishNumberOption::Complex(Box::new(SnailFishNumber {
                        x: SnailFishNumberOption::Literal(*literal / 2),
                        y: SnailFishNumberOption::Literal(f32::ceil(*literal as f32 / 2_f32) as i32),
                    }));

                    return true;
                }
            }
        }

        if splitted {
            return splitted;
        }

        match &mut self.y {
            SnailFishNumberOption::Complex(box_number) => {
                let number: &mut SnailFishNumber = &mut *box_number;
                splitted = number.split();
            }
            SnailFishNumberOption::Literal(literal) => {
                if *literal > 9 {
                    self.y = SnailFishNumberOption::Complex(Box::new(SnailFishNumber {
                        x: SnailFishNumberOption::Literal(*literal / 2),
                        y: SnailFishNumberOption::Literal(f32::ceil(*literal as f32 / 2_f32) as i32),
                    }));

                    return true;
                }
            }
        }

        splitted
    }
}

fn main() {
    let snailfish_input =
        input::lines_from_file(EXERCISE_INPUT_FILE).expect("Could not load lines");

    let (magnitude_all_numbers, max_magnitude) =
        calculate_snailfish_assignment_magnitude(snailfish_input);

    println!(
        "The Magnitude for all numbers is: {} and the max magnitude is: {}",
        magnitude_all_numbers, max_magnitude
    );
}

fn calculate_snailfish_assignment_magnitude(snailfish_input: Vec<String>) -> (i32, i32) {
    let snailfish_numbers: Vec<SnailFishNumber> = parse_snailfish_input(&snailfish_input);

    let magnitude_all_numbers = calculate_magnitude_of_all_numbers(&snailfish_numbers);
    let max_magnitude = find_max_magnitude_of_sums(&snailfish_numbers);

    (magnitude_all_numbers, max_magnitude)
}

fn calculate_magnitude_of_all_numbers(snailfish_numbers: &Vec<SnailFishNumber>) -> i32 {
    let mut result = snailfish_numbers[0].clone();

    for i in 1..snailfish_numbers.len() {
        result = result.add(&snailfish_numbers[i].clone());
    }

    result.magnitude()
}

fn find_max_magnitude_of_sums(snailfish_numbers: &Vec<SnailFishNumber>) -> i32 {
    let mut max_magnitude = 0;

    for i in 0..snailfish_numbers.len() {
        for j in 0..snailfish_numbers.len() {
            if i == j {
                continue;
            }

            max_magnitude = max(
                max_magnitude,
                snailfish_numbers[i].add(&snailfish_numbers[j]).magnitude(),
            );
        }
    }

    max_magnitude
}

fn parse_snailfish_input(snailfish_input: &Vec<String>) -> Vec<SnailFishNumber> {
    snailfish_input
        .iter()
        .map(|line| parse_input_line(line, 0).0)
        .collect()
}

fn parse_input_line(line: &str, mut current_index: usize) -> (SnailFishNumber, usize) {
    let x: SnailFishNumberOption;
    let y: SnailFishNumberOption;

    if line.get(current_index..current_index + 1).unwrap() == "[" {
        let first_part = parse_input_line(line, current_index + 1);
        x = SnailFishNumberOption::Complex(Box::new(first_part.0));
        current_index = first_part.1;
    } else {
        x = SnailFishNumberOption::Literal(
            line.get(current_index..current_index + 1)
                .unwrap()
                .parse()
                .unwrap(),
        );
        current_index += 1;
    }

    if current_index == line.len() {
        if let SnailFishNumberOption::Complex(number) = x {
            return (*number, current_index);
        }
    }

    if line.get(current_index..current_index + 1).unwrap() != "," {
        panic!("parsing error");
    }

    current_index += 1;

    if line.get(current_index..current_index + 1).unwrap() == "[" {
        let first_part = parse_input_line(line, current_index + 1);
        y = SnailFishNumberOption::Complex(Box::new(first_part.0));
        current_index = first_part.1;
    } else {
        y = SnailFishNumberOption::Literal(
            line.get(current_index..current_index + 1)
                .unwrap()
                .parse()
                .unwrap(),
        );
        current_index += 1;
    }

    current_index += 1;

    (SnailFishNumber { x: x, y: y }, current_index)
}

fn add_to_first_left_number(number: i32, snailfish_number: &mut SnailFishNumberOption) {
    match snailfish_number {
        SnailFishNumberOption::Complex(snailfish_number) => {
            add_to_first_left_number(number, &mut snailfish_number.x);
        }
        SnailFishNumberOption::Literal(literal) => {
            *literal = *literal + number;
        }
    }
}

fn add_to_first_right_number(number: i32, snailfish_number: &mut SnailFishNumberOption) {
    match snailfish_number {
        SnailFishNumberOption::Complex(snailfish_number) => {
            add_to_first_right_number(number, &mut snailfish_number.y);
        }
        SnailFishNumberOption::Literal(literal) => {
            *literal = *literal + number;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_FILE: &str = "src/assets/day18/example_input.txt";

    #[test]
    fn should_calculate_the_magnitude_and_find_max_magnitude_for_example_input() {
        let snailfish_input = input::lines_from_file(EXAMPLE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_snailfish_assignment_magnitude(snailfish_input),
            (4140, 3993),
        );
    }

    #[test]
    fn should_calculate_the_magnitude_and_find_max_magnitude_for_exercise_input() {
        let snailfish_input = input::lines_from_file(EXERCISE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_snailfish_assignment_magnitude(snailfish_input),
            (4184, 4731)
        );
    }
}
