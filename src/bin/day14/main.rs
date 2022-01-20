use advent_of_code_2021::common::input;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

const LARGE_INPUT_FILE: &str = "src/assets/day14/large_input.txt";

#[derive(Debug)]
struct InsertionRule {
    element: String,
}

fn main() {
    let polymer_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let insertion_score = calculate_insertion_to_polymer_template(polymer_input, 40);

    println!("The result is: {}", insertion_score);
}

fn calculate_insertion_to_polymer_template(polymer_input: Vec<String>, steps: usize) -> u64 {
    let insertion_rules = get_insertion_rules(&polymer_input);

    let (mut pair_counts, mut element_counts) = init_counters(&polymer_input);

    for _j in 0..steps {
        let (new_pair_counts, new_element_counts) =
            calculate_step(&insertion_rules, &pair_counts, &element_counts);

        pair_counts = new_pair_counts;
        element_counts = new_element_counts;
    }

    let max_count = element_counts
        .values()
        .fold(0, |max_count, count| max(max_count, *count));
    let min_count = element_counts
        .values()
        .fold(max_count, |min_count, count| min(min_count, *count));
    max_count - min_count
}

fn get_polymer_template(polymer_input: &Vec<String>) -> String {
    polymer_input[0].to_string()
}

fn get_insertion_rules(polymer_input: &Vec<String>) -> HashMap<String, InsertionRule> {
    let insertion_rules_input = polymer_input.clone().split_off(2);
    let mut insertion_rules = HashMap::new();

    for input in insertion_rules_input {
        let insertion_rule_parts: Vec<&str> = input.split(" -> ").collect();
        insertion_rules.insert(
            insertion_rule_parts[0].to_string(),
            InsertionRule {
                element: insertion_rule_parts[1].to_string(),
            },
        );
    }

    insertion_rules
}

fn init_counters(polymer_input: &Vec<String>) -> (HashMap<String, u64>, HashMap<String, u64>) {
    let polymer_template = get_polymer_template(&polymer_input);
    let mut pair_counts: HashMap<String, u64> = HashMap::new();
    let mut element_counts: HashMap<String, u64> = HashMap::new();

    for i in 0..polymer_template.len() {
        let element = polymer_template.get(i..i + 1).unwrap();

        element_counts.insert(
            element.to_string(),
            *element_counts.get(element).unwrap_or(&0) + 1,
        );
    }

    for i in 0..polymer_template.len() - 1 {
        let pair = polymer_template.get(i..i + 2).unwrap();

        pair_counts.insert(pair.to_string(), *pair_counts.get(pair).unwrap_or(&0) + 1);
    }

    (pair_counts, element_counts)
}

fn calculate_step(
    insertion_rules: &HashMap<String, InsertionRule>,
    pair_counts: &HashMap<String, u64>,
    element_counts: &HashMap<String, u64>,
) -> (HashMap<String, u64>, HashMap<String, u64>) {
    let mut new_pair_counts: HashMap<String, u64> = HashMap::new();
    let mut new_element_counts = element_counts.clone();

    for (key, pair_count) in pair_counts.iter() {
        let insertion_rule = insertion_rules.get(key).unwrap();

        let element_key_first_letter = key.get(0..1).unwrap().to_string() + &insertion_rule.element;
        let element_key_second_letter = insertion_rule.element.clone() + key.get(1..2).unwrap();

        new_pair_counts.insert(
            element_key_first_letter.clone(),
            *new_pair_counts.get(&element_key_first_letter).unwrap_or(&0) + pair_count,
        );
        new_pair_counts.insert(
            element_key_second_letter.clone(),
            *new_pair_counts
                .get(&element_key_second_letter)
                .unwrap_or(&0)
                + pair_count,
        );
        new_element_counts.insert(
            insertion_rule.element.clone(),
            new_element_counts
                .get(&insertion_rule.element)
                .unwrap_or(&0)
                + pair_count,
        );
    }

    (new_pair_counts, new_element_counts)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day14/small_input.txt";

    #[test]
    fn should_calculate_polymer_insertion_for_10_steps_for_small_input() {
        let polymer_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_insertion_to_polymer_template(polymer_input, 10),
            1588
        );
    }

    #[test]
    fn should_calculate_polymer_insertion_for_10_steps_for_large_input() {
        let polymer_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_insertion_to_polymer_template(polymer_input, 10),
            2988
        );
    }

    #[test]
    fn should_calculate_polymer_insertion_for_40_steps_for_small_input() {
        let polymer_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_insertion_to_polymer_template(polymer_input, 40),
            2188189693529
        );
    }

    #[test]
    fn should_calculate_polymer_insertion_for_40_steps_for_large_input() {
        let polymer_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            calculate_insertion_to_polymer_template(polymer_input, 40),
            3572761917024
        );
    }
}
