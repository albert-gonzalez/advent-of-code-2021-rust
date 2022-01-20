use advent_of_code_2021::common::input;

const LARGE_INPUT_FILE: &str = "src/assets/day4/large_input.txt";

#[derive(Debug)]
struct BingoCard {
    lines: Vec<Vec<BingoCell>>,
    winner: bool,
}

#[derive(Debug)]
struct BingoCell {
    number: String,
    checked: bool,
}

fn main() {
    let bingo_input = input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let bingo_winner = play_bingo(bingo_input);

    println!("The result is: {}", bingo_winner);
}

fn play_bingo(bingo_input: Vec<String>) -> i32 {
    let bingo_numbers: Vec<&str> = parse_bingo_numbers(&bingo_input);

    let mut bingo_cards = create_bingo_cards(&bingo_input);
    let mut last_winner_result = 0;

    for bingo_number in bingo_numbers {
        check_card_number(bingo_number, &mut bingo_cards);

        for bingo_card in &mut bingo_cards {
            if !bingo_card.winner && is_a_winner_card(bingo_card) {
                bingo_card.winner = true;
                last_winner_result = calculate_winner_card_result(bingo_number, bingo_card);
            }
        }
    }

    last_winner_result
}

fn parse_bingo_numbers(bingo_input: &Vec<String>) -> Vec<&str> {
    bingo_input[0].split(",").collect()
}

fn create_bingo_cards(bingo_input: &Vec<String>) -> Vec<BingoCard> {
    let mut bingo_card = BingoCard {
        lines: Vec::new(),
        winner: false,
    };
    let mut bingo_cards = Vec::new();

    for i in 2..bingo_input.len() {
        let line = &bingo_input[i];

        if line.len() == 0 {
            bingo_cards.push(bingo_card);
            bingo_card = BingoCard {
                lines: Vec::new(),
                winner: false,
            };
        } else {
            let line_numbers = line.split_whitespace();
            let mut cells = Vec::new();
            for line_number in line_numbers {
                cells.push(BingoCell {
                    number: line_number.to_string(),
                    checked: false,
                });
            }
            bingo_card.lines.push(cells);
        }
    }

    bingo_cards.push(bingo_card);

    bingo_cards
}

fn check_card_number(bingo_number: &str, bingo_cards: &mut Vec<BingoCard>) {
    for bingo_card in bingo_cards {
        for bingo_line in &mut bingo_card.lines {
            for mut bingo_cell in &mut *bingo_line {
                if bingo_cell.number == bingo_number {
                    bingo_cell.checked = true;
                }
            }
        }
    }
}

fn calculate_winner_card_result(bingo_number: &str, bingo_card: &BingoCard) -> i32 {
    let mut sum: i32 = 0;

    for bingo_line in &bingo_card.lines {
        for bingo_cell in bingo_line {
            if !bingo_cell.checked {
                let number: i32 = bingo_cell.number.parse().unwrap();
                sum += number;
            }
        }
    }

    let integer_bingo_number: i32 = bingo_number.parse().unwrap();

    sum * integer_bingo_number
}

fn has_card_a_winner_line(bingo_card: &BingoCard) -> bool {
    let mut all_checked = true;

    for bingo_line in &bingo_card.lines {
        for bingo_cell in bingo_line {
            all_checked = bingo_cell.checked;

            if !all_checked {
                break;
            }
        }

        if all_checked {
            return true;
        }
    }

    false
}

fn has_card_a_winner_column(bingo_card: &BingoCard) -> bool {
    let mut all_checked = true;
    let line_length = bingo_card.lines[0].len();

    for i in 0..line_length {
        for bingo_line in &bingo_card.lines {
            let bingo_cell = &bingo_line[i];
            all_checked = bingo_cell.checked;

            if !all_checked {
                break;
            }
        }

        if all_checked {
            return true;
        }
    }

    false
}

fn is_a_winner_card(bingo_card: &BingoCard) -> bool {
    has_card_a_winner_line(bingo_card) || has_card_a_winner_column(bingo_card)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day4/small_input.txt";

    #[test]
    fn should_find_the_last_winning_bingo_card_with_small_input() {
        let bingo_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(play_bingo(bingo_input), 1924);
    }

    #[test]
    fn should_find_the_last_winning_bingo_card_with_large_input() {
        let bingo_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(play_bingo(bingo_input), 12635);
    }
}
