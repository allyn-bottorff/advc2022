use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}
/// Read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Play the rock paper scissors game using day 2 part 1 rules.
fn rock_paper_scissors(file: &str) -> i32 {
    let mut score: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_string) = line {
                let opp_char = line_string.as_str().chars().nth(0).unwrap();
                let you_char = line_string.as_str().chars().nth(2).unwrap();

                let opp_choice = match opp_char {
                    'A' => RPSChoice::Rock,
                    'B' => RPSChoice::Paper,
                    'C' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                let your_choice = match you_char {
                    'X' => RPSChoice::Rock,
                    'Y' => RPSChoice::Paper,
                    'Z' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                score += score_rps(opp_choice, your_choice);
            }
        }
    }

    println!("Your score: {}", score);
    score
}

/// Play the rock paper scissors game using day 2 part 2 rules.
fn rock_paper_scissors2(file: &str) -> i32 {
    let mut score: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_string) = line {
                // Using unwrap() here because if this fails, it' better to
                // crash.
                let opp_char = line_string.as_str().chars().nth(0).unwrap();
                let you_char = line_string.as_str().chars().nth(2).unwrap();

                let opp_choice = match opp_char {
                    'A' => RPSChoice::Rock,
                    'B' => RPSChoice::Paper,
                    'C' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                let your_choice = match you_char {
                    'X' => rps_pick_lose(&opp_choice),
                    'Y' => opp_choice,
                    'Z' => rps_pick_win(&opp_choice),
                    _ => panic!("Failed to parse move"),
                };
                score += score_rps(opp_choice, your_choice);
            }
        }
    }

    println!("Your score: {}", score);
    score
}

/// Pick a winning choice based on the opponent's choice
fn rps_pick_win(c1: &RPSChoice) -> RPSChoice {
    let win = match c1 {
        RPSChoice::Rock => RPSChoice::Paper,
        RPSChoice::Paper => RPSChoice::Scissors,
        RPSChoice::Scissors => RPSChoice::Rock,
    };
    win
}

/// Pick a losing choice based on the opponent's choice
fn rps_pick_lose(c1: &RPSChoice) -> RPSChoice {
    let lose = match c1 {
        RPSChoice::Rock => RPSChoice::Scissors,
        RPSChoice::Paper => RPSChoice::Rock,
        RPSChoice::Scissors => RPSChoice::Paper,
    };
    lose
}

/// Calculate the score of a Rock Paper Scissors game.
fn score_rps(c1: RPSChoice, c2: RPSChoice) -> i32 {
    let score = match (c1, c2) {
        (RPSChoice::Rock, RPSChoice::Rock) => 1 + 3,
        (RPSChoice::Rock, RPSChoice::Paper) => 2 + 6,
        (RPSChoice::Rock, RPSChoice::Scissors) => 3 + 0,

        (RPSChoice::Paper, RPSChoice::Rock) => 1 + 0,
        (RPSChoice::Paper, RPSChoice::Paper) => 2 + 3,
        (RPSChoice::Paper, RPSChoice::Scissors) => 3 + 6,

        (RPSChoice::Scissors, RPSChoice::Rock) => 1 + 6,
        (RPSChoice::Scissors, RPSChoice::Paper) => 2 + 0,
        (RPSChoice::Scissors, RPSChoice::Scissors) => 3 + 3,
    };

    score
}
fn main() {
    rock_paper_scissors("./day2.txt");
    rock_paper_scissors2("./day2.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_1() {
        let score = rock_paper_scissors("./tests/day2.txt");

        assert_eq!(score, 15)
    }
    #[test]
    fn test_day2_2() {
        let score = rock_paper_scissors2("./tests/day2.txt");

        assert_eq!(score, 12)
    }
}
