use advent_of_code_2021::common::input;
use std::cmp::max;

const LARGE_INPUT_FILE: &str = "src/assets/day13/large_input.txt";

#[derive(Debug)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct FoldInstruction {
    position: usize,
    fold_type: FoldType,
}

#[derive(Debug)]
enum FoldType {
    Horizontal,
    Vertical,
}

fn main() {
    let transparent_paper_input =
        input::lines_from_file(LARGE_INPUT_FILE).expect("Could not load lines");

    let syntax_score = count_dots_in_folded_paper(transparent_paper_input, true);

    println!("The result is: {}", syntax_score);
}

fn count_dots_in_folded_paper(transparent_paper_input: Vec<String>, print: bool) -> i32 {
    let mut dots_paper = create_dots_paper(&transparent_paper_input);
    let folds = create_fold_instructions(&transparent_paper_input);

    dots_paper = fold_paper(&mut dots_paper, &folds);

    if print {
        println!(
            "{}",
            dots_paper
                .iter()
                .map(|line| line
                    .iter()
                    .map(|is_dot| if *is_dot { "*" } else { "-" })
                    .collect::<Vec<&str>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    dots_paper.iter().fold(0, |count, line| {
        count
            + line
                .iter()
                .fold(0, |count, is_dot| if *is_dot { count + 1 } else { count })
    })
}

fn create_dots_paper(transparent_paper_input: &Vec<String>) -> Vec<Vec<bool>> {
    let mut max_x = 0;
    let mut max_y = 0;

    let dots: Vec<Dot> = transparent_paper_input
        .iter()
        .filter(|line| line.contains(","))
        .map(|line| {
            let coordinates: Vec<usize> = line
                .split(",")
                .map(|positions| positions.parse().unwrap())
                .collect();

            Dot {
                x: coordinates[0],
                y: coordinates[1],
            }
        })
        .collect();

    for dot in &dots {
        max_x = max(max_x, dot.x);
        max_y = max(max_y, dot.y);
    }

    let mut map = vec![vec![false; max_x + 1]; max_y + 1];

    for dot in dots {
        map[dot.y][dot.x] = true;
    }

    map
}

fn create_fold_instructions(transparent_paper_input: &Vec<String>) -> Vec<FoldInstruction> {
    transparent_paper_input
        .iter()
        .filter(|line| line.contains("="))
        .map(|line| {
            let fold: Vec<&str> = line.split("=").collect();

            FoldInstruction {
                position: fold[1].parse().unwrap(),
                fold_type: if fold[0].contains("x") {
                    FoldType::Vertical
                } else {
                    FoldType::Horizontal
                },
            }
        })
        .collect()
}

fn fold_paper(dots_paper: &mut Vec<Vec<bool>>, folds: &Vec<FoldInstruction>) -> Vec<Vec<bool>> {
    let mut folded_paper = dots_paper.clone();

    for fold in folds {
        if matches!(fold.fold_type, FoldType::Horizontal) {
            folded_paper = fold_horizontal(&folded_paper, fold.position);
        } else {
            folded_paper = fold_vertical(&folded_paper, fold.position);
        }
    }

    folded_paper
}

fn fold_horizontal(dots_paper: &Vec<Vec<bool>>, fold_position: usize) -> Vec<Vec<bool>> {
    let mut folded_paper_first_part = dots_paper.clone();
    let folded_paper_second_part = folded_paper_first_part.split_off(fold_position + 1);

    for (y, line) in folded_paper_second_part.iter().rev().enumerate() {
        for (x, is_dot) in line.iter().enumerate() {
            folded_paper_first_part[y][x] |= *is_dot;
        }
    }

    folded_paper_first_part.pop();

    folded_paper_first_part
}

fn fold_vertical(dots_paper: &Vec<Vec<bool>>, fold_position: usize) -> Vec<Vec<bool>> {
    dots_paper
        .iter()
        .map(|line| {
            let mut line_first_part = line.clone();
            let line_second_part = line_first_part.split_off(fold_position + 1);

            for (x, is_dot) in line_second_part.iter().rev().enumerate() {
                line_first_part[x] |= is_dot;
            }

            line_first_part.pop();

            line_first_part
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT_FILE: &str = "src/assets/day13/small_input.txt";

    #[test]
    fn should_count_dots_for_small_input() {
        let transparent_paper_input = input::lines_from_file(SMALL_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            count_dots_in_folded_paper(transparent_paper_input, false),
            16
        );
    }

    #[test]
    fn should_count_dots_for_large_input() {
        let transparent_paper_input = input::lines_from_file(LARGE_INPUT_FILE)
            .expect("Something went wrong reading the file");

        assert_eq!(
            count_dots_in_folded_paper(transparent_paper_input, false),
            95
        );
    }
}
