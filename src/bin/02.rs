use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Roshambo {
    Rock,
    Paper,
    Scissors,
}

impl Roshambo {
    fn from_str(str: &str) -> Result<Roshambo> {
        match str {
            "A" | "X" => Ok(Roshambo::Rock),
            "B" | "Y" => Ok(Roshambo::Paper),
            "C" | "Z" => Ok(Roshambo::Scissors),
            _ => Err(anyhow!("Unhandled input: {}", str)),
        }
    }

    fn from_result(opponent_choice: &Roshambo, result: &GameResult) -> Self {
        match (opponent_choice, result) {
            (Roshambo::Rock, GameResult::Win) => Roshambo::Paper,
            (Roshambo::Rock, GameResult::Lose) => Roshambo::Scissors,
            (Roshambo::Rock, GameResult::Draw) => Roshambo::Rock,
            (Roshambo::Paper, GameResult::Win) => Roshambo::Scissors,
            (Roshambo::Paper, GameResult::Lose) => Roshambo::Rock,
            (Roshambo::Paper, GameResult::Draw) => Roshambo::Paper,
            (Roshambo::Scissors, GameResult::Win) => Roshambo::Rock,
            (Roshambo::Scissors, GameResult::Lose) => Roshambo::Paper,
            (Roshambo::Scissors, GameResult::Draw) => Roshambo::Scissors,
        }
    }

    fn point_value(&self) -> i32 {
        match self {
            Roshambo::Rock => 1,
            Roshambo::Paper => 2,
            Roshambo::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn from_round(a: &Roshambo, b: &Roshambo) -> GameResult {
        match (a, b) {
            (Roshambo::Rock, Roshambo::Rock)
            | (Roshambo::Paper, Roshambo::Paper)
            | (Roshambo::Scissors, Roshambo::Scissors) => GameResult::Draw,
            (Roshambo::Rock, Roshambo::Paper)
            | (Roshambo::Paper, Roshambo::Scissors)
            | (Roshambo::Scissors, Roshambo::Rock) => GameResult::Lose,
            (Roshambo::Rock, Roshambo::Scissors)
            | (Roshambo::Paper, Roshambo::Rock)
            | (Roshambo::Scissors, Roshambo::Paper) => GameResult::Win,
        }
    }

    fn point_value(&self) -> i32 {
        match self {
            GameResult::Win => 6,
            GameResult::Lose => 0,
            GameResult::Draw => 3,
        }
    }

    fn from_str(str: &str) -> Result<GameResult> {
        match str {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(anyhow!("Unhandled input: {}", str)),
        }
    }
}

fn parse_input_one(input: &str) -> Result<Vec<Vec<Roshambo>>> {
    input
        .lines()
        .map(|line| line.split(" ").map(|c| Roshambo::from_str(c)).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    parse_input_one(input).map(|turns| {
        turns.iter().map(|turn| {
            let result = GameResult::from_round(&turn[1], &turn[0]);
            result.point_value() + turn[1].point_value()
        }).sum()
    }).ok()
}

fn parse_input_two(input: &str) -> Result<Vec<Vec<Roshambo>>> {
    Ok(input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let opponent_choice = Roshambo::from_str(split.next().unwrap()).unwrap();
            let desired_result = GameResult::from_str(split.next().unwrap()).unwrap();
            vec![opponent_choice.clone(), Roshambo::from_result(&opponent_choice, &desired_result)]
        }).collect())
}

pub fn part_two(input: &str) -> Option<i32> {
    parse_input_two(input).map(|turns| {
        turns.iter().map(|turn| {
            let result = GameResult::from_round(&turn[1], &turn[0]);
            result.point_value() + turn[1].point_value()
        }).sum()
    }).ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
