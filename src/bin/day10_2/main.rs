use advent_of_code_2021::common::input;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

const LARGE_INPUT_FILE: &str = "src/assets/day10/large_input.txt";

const START_TOKENS: [char; 4] = ['<', '{', '[', '('];
const END_TOKENS: [char; 4] = ['>', '}', ']', ')'];

struct EndTokenData {
    related_start_token: char,
}

struct StartTokenData {
    completion_score: i64,
}

fn main() {
    let navigation_subsystem_input =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let syntax_score = calculate_syntax_score(navigation_subsystem_input);

    println!("The result is: {:?}", syntax_score);
}

fn calculate_syntax_score(navigation_subsystem_input: Vec<String>) -> i64 {
    let start_token_data_map = get_start_tokens_data();
    let end_token_data_map = get_end_tokens_data();

    let mut scores: Vec<i64> = navigation_subsystem_input
        .iter()
        .fold(Vec::new(), |mut scores, line| {
            scores.push(calculate_line_syntax_score(
                line,
                &start_token_data_map,
                &end_token_data_map,
            ));

            scores
        })
        .iter()
        .filter(|score| score > &&0)
        .cloned()
        .collect();

    scores.sort();

    scores[scores.len() / 2]
}

fn calculate_line_syntax_score(
    line: &String,
    start_token_data_map: &HashMap<char, StartTokenData>,
    end_token_data_map: &HashMap<char, EndTokenData>,
) -> i64 {
    let mut pending_start_tokens: Vec<char> = Vec::new();

    for token in line.chars() {
        if START_TOKENS.contains(&token) {
            pending_start_tokens.push(token);
        }

        if END_TOKENS.contains(&token) {
            let token_data = end_token_data_map.get(&token).unwrap();

            if token_data.related_start_token != pending_start_tokens.pop().unwrap() {
                return 0;
            }
        }
    }

    pending_start_tokens.iter().rev().fold(0, |score, token| {
        score * 5 + start_token_data_map.get(token).unwrap().completion_score
    })
}

fn get_end_tokens_data() -> HashMap<char, EndTokenData> {
    HashMap::from_iter(IntoIter::new([
        (
            '>',
            EndTokenData {
                related_start_token: '<',
            },
        ),
        (
            '}',
            EndTokenData {
                related_start_token: '{',
            },
        ),
        (
            ']',
            EndTokenData {
                related_start_token: '[',
            },
        ),
        (
            ')',
            EndTokenData {
                related_start_token: '(',
            },
        ),
    ]))
}

fn get_start_tokens_data() -> HashMap<char, StartTokenData> {
    HashMap::from_iter(IntoIter::new([
        (
            '<',
            StartTokenData {
                completion_score: 4,
            },
        ),
        (
            '{',
            StartTokenData {
                completion_score: 3,
            },
        ),
        (
            '[',
            StartTokenData {
                completion_score: 2,
            },
        ),
        (
            '(',
            StartTokenData {
                completion_score: 1,
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

        assert_eq!(calculate_syntax_score(navigation_subsystem_input), 288957);
    }

    #[test]
    fn should_calculate_syntax_error_for_large_input() {
        let navigation_subsystem_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_syntax_score(navigation_subsystem_input),
            2995077699
        );
    }
}
