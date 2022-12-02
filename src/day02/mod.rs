use phf::phf_map;

use crate::utils::file_read_lines;

#[derive(Debug)]
enum Result {
    Win = 6,
    Loss = 0,
    Even = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

static LEFT: phf::Map<&'static str, RPS> = phf_map! {
    "A" => RPS::Rock,
    "B" => RPS::Paper,
    "C" => RPS::Scissors
};

static RIGHT: phf::Map<&'static str, RPS> = phf_map! {
    "X" => RPS::Rock,
    "Y" => RPS::Paper,
    "Z" => RPS::Scissors
};

static RIGHT_RESULT: phf::Map<&'static str, Result> = phf_map! {
    "X" => Result::Loss,
    "Y" => Result::Even,
    "Z" => Result::Win,
};

#[derive(Clone, Copy)]
struct Game {
    left: RPS,
    right: RPS,
}

impl Game {
    fn from_game_line(line: &str) -> Game {
        let elements: Vec<&str> = line.split(" ").collect();
        Game {
            left: LEFT.get(elements[0]).unwrap().clone(),
            right: RIGHT.get(elements[1]).unwrap().clone(),
        }
    }

    fn from_result_line(line: &str) -> Game {
        let elements: Vec<&str> = line.split(" ").collect();
        let left_move = LEFT.get(elements[0]).unwrap().clone();
        let right_result = RIGHT_RESULT.get(elements[1]).unwrap().clone();
        let right_move = match right_result {
            Result::Even => left_move,
            Result::Loss => match left_move {
                RPS::Paper => RPS::Rock,
                RPS::Rock => RPS::Scissors,
                RPS::Scissors => RPS::Paper,
            },
            Result::Win => match left_move {
                RPS::Paper => RPS::Scissors,
                RPS::Rock => RPS::Paper,
                RPS::Scissors => RPS::Rock,
            },
        };
        Game { left: left_move, right: right_move }
    }

    fn score_right(&self) -> u32 {
        let move_score = self.right as u32;
        let result_score = self.result_right() as u32;
        move_score + result_score
    }

    fn result_right(&self) -> Result {
        let r = (3 + (self.left as i8) - (self.right as i8)) % 3;
        match r {
            0 => Result::Even,
            1 => Result::Loss,
            2 => Result::Win,
            _ => panic!("This should never happen"),
        }
    }
}

fn calculate_points<T: Iterator<Item = Game>>(games: T) -> u32 {
    let mut score: u32 = 0;
    for game in games {
        score += game.score_right();
    }
    score
}

pub fn run() {
    let lines: Vec<String> = file_read_lines("./src/day02/input.txt")
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    let points = calculate_points(lines.iter().map(|s| Game::from_game_line(s.as_str())));
    let result = calculate_points(lines.iter().map(|s| Game::from_result_line(s.as_str())));

    println!("Day02: Points games {}", points);
    println!("Day02: Points results {}", result);
}

#[cfg(test)]
mod tests {
    use super::{calculate_points, Game, RPS};

    static INPUT: &str = "A Y\n\
        B X\n\
        C Z\n";

    #[test]
    fn test_simple_guide() {
        let lines = INPUT.lines().map(Game::from_game_line);
        let score = calculate_points(lines);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_single_game() {
        let game = Game::from_game_line("A Y");
        assert_eq!(game.left, RPS::Rock);
        assert_eq!(game.right, RPS::Paper);
        let score = game.score_right();
        assert_eq!(score, 8);
    }

    #[test]
    fn test_result() {
        let lines = INPUT.lines().map(Game::from_result_line);
        let score = calculate_points(lines);
        assert_eq!(score, 12);
    }

    #[test]
    fn test_single_result() {
        let game = Game::from_result_line("A Y");
        assert_eq!(game.left, RPS::Rock);
        assert_eq!(game.right, RPS::Rock);
        assert_eq!(4, game.score_right());
    }
}
