use std::{collections::HashSet, num::ParseIntError};

use itertools::Itertools;

type TreeGrid = Vec<Vec<u32>>;
type Coords = (usize, usize);
type VisibilitySet = HashSet<Coords>;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn parse(input: &str) -> Result<TreeGrid, ParseIntError> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_string().parse()).collect())
        .collect()
}

fn count_visible(grid: &TreeGrid) -> usize {
    let mut visible_set = VisibilitySet::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let neighbors: Vec<u32> = match dir {
                    Direction::North => (0..y).map(|y| grid[y][x]).collect(),
                    Direction::South => (y + 1..grid.len()).map(|y| grid[y][x]).collect(),
                    Direction::East => (x + 1..grid[y].len()).map(|x| grid[y][x]).collect(),
                    Direction::West => (0..x).map(|x| grid[y][x]).collect(),
                };
                let blocking = neighbors.iter().find(|n| *n >= val);
                if blocking.is_none() {
                    visible_set.insert((x, y));
                }
            }
        }
    }

    visible_set.len()
}

fn scenic_score(grid: &TreeGrid) -> Option<u32> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, val)| {
                [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .iter()
                .map(|dir| {
                    let neighbors = match (dir, x, y) {
                        (Direction::North, _, y) if y == 0 => vec![],
                        (Direction::North, _, _) => (0..y).map(|y| grid[y][x]).rev().collect(),
                        (Direction::South, _, y) if y == grid.len() - 1 => vec![],
                        (Direction::South, _, _) => {
                            (y + 1..grid.len()).map(|y| grid[y][x]).collect()
                        }
                        (Direction::East, x, _) if x == grid[y].len() - 1 => vec![],
                        (Direction::East, _, _) => {
                            (x + 1..grid[y].len()).map(|x| grid[y][x]).collect()
                        }
                        (Direction::West, x, _) if x == 0 => vec![],
                        (Direction::West, _, _) => (0..x).map(|x| grid[y][x]).rev().collect(),
                    };
                    neighbors
                        .iter()
                        .find_position(|n| *n >= val)
                        .map(|(idx, _)| (idx + 1) as u32)
                        .unwrap_or_else(|| neighbors.len() as u32)
                })
                .product()
            })
        })
        .max()
}

pub fn part_one(input: &str) -> Option<usize> {
    parse(input).map(|grid| count_visible(&grid)).ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    parse(input).map(|grid| scenic_score(&grid)).ok().unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
