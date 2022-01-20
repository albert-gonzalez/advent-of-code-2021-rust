use advent_of_code_2021::common::input;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

const LARGE_INPUT_FILE: &str = "src/assets/day10/large_input.txt";

const START_TOKENS: [char; 4] = ['<', '{', '[', '('];
const END_TOKENS: [char; 4] = ['>', '}', ']', ')'];

struct EndTokenData {
    related_start_token: char,
    syntax_error_score: i32,
}

fn main() {
    let navigation_subsystem_input =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let syntax_score = calculate_syntax_score(navigation_subsystem_input);

    println!("The result is: {}", syntax_score);
}

fn calculate_syntax_score(navigation_subsystem_input: Vec<String>) -> i32 {
    let end_token_data_map = get_end_tokens_data();

    navigation_subsystem_input.iter().fold(0, |score, line| {
        score + calculate_line_syntax_score(line, &end_token_data_map)
    })
}

fn calculate_line_syntax_score(
    line: &String,
    end_token_data_map: &HashMap<char, EndTokenData>,
) -> i32 {
    let mut pending_start_tokens: Vec<char> = Vec::new();

    for token in line.chars() {
        if START_TOKENS.contains(&token) {
            pending_start_tokens.push(token);
        }

        if END_TOKENS.contains(&token) {
            let token_data = end_token_data_map.get(&token).unwrap();

            if token_data.related_start_token != pending_start_tokens.pop().unwrap() {
                return token_data.syntax_error_score;
            }
        }
    }

    0
}

fn get_end_tokens_data() -> HashMap<char, EndTokenData> {
    HashMap::from_iter(IntoIter::new([
        (
            '>',
            EndTokenData {
                related_start_token: '<',
                syntax_error_score: 25137,
            },
        ),
        (
            '}',
            EndTokenData {
                related_start_token: '{',
                syntax_error_score: 1197,
            },
        ),
        (
            ']',
            EndTokenData {
                related_start_token: '[',
                syntax_error_score: 57,
            },
        ),
        (
            ')',
            EndTokenData {
                related_start_token: '(',
                syntax_error_score: 3,
            },
        ),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day10/small_input.txt";

    #[test]
    fn should_calculate_syntax_error_for_small_input() {
        let navigation_subsystem_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_syntax_score(navigation_subsystem_input), 26397);
    }

    #[test]
    fn should_calculate_syntax_error_for_large_input() {
        let navigation_subsystem_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(calculate_syntax_score(navigation_subsystem_input), 399153);
    }
}
