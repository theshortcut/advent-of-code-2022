use std::collections::HashSet;

use itertools::Itertools;

type Coord = (i32, i32);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str) -> Vec<Direction> {
    input
        .lines()
        .flat_map(|line| {
            let (direction_str, distance_str) = line.split(' ').collect_tuple().unwrap();
            match (direction_str, distance_str.parse()) {
                ("L", Ok(count)) => (0..count).map(|_| Direction::Left).collect_vec(),
                ("R", Ok(count)) => (0..count).map(|_| Direction::Right).collect_vec(),
                ("U", Ok(count)) => (0..count).map(|_| Direction::Up).collect_vec(),
                ("D", Ok(count)) => (0..count).map(|_| Direction::Down).collect_vec(),
                _ => unreachable!("unhandled input"),
            }
        })
        .collect()
}

fn apply_direction(current_coord: &Coord, dir: &Direction) -> Coord {
    match dir {
        Direction::Left => (current_coord.0 - 1, current_coord.1),
        Direction::Right => (current_coord.0 + 1, current_coord.1),
        Direction::Up => (current_coord.0, current_coord.1 + 1),
        Direction::Down => (current_coord.0, current_coord.1 - 1),
    }
}

fn follow_head(head_coord: &Coord, tail_coord: &Coord) -> Coord {
    let (mut x, mut y) = tail_coord;
    if tail_coord.0 != head_coord.0
        && tail_coord.1 != head_coord.1
        && (head_coord.0.abs_diff(tail_coord.0) > 1 || head_coord.1.abs_diff(tail_coord.1) > 1)
    {
        // not touching in same row and same column so tail moves diagonally
        if tail_coord.0 > head_coord.0 {
            x -= 1
        } else {
            x += 1
        }
        if tail_coord.1 > head_coord.1 {
            y -= 1
        } else {
            y += 1
        }
    } else if head_coord.0 - tail_coord.0 > 1 {
        x += 1;
    } else if tail_coord.0 - head_coord.0 > 1 {
        x -= 1;
    } else if head_coord.1 - tail_coord.1 > 1 {
        y += 1;
    } else if tail_coord.1 - head_coord.1 > 1 {
        y -= 1;
    }
    (x, y)
}

fn generate_tail_set(directions: &Vec<Direction>, knot_count: usize) -> HashSet<Coord> {
    let mut coords: Vec<Coord> = (0..knot_count).map(|_| (0, 0)).collect();
    let mut tail_set = HashSet::new();
    for dir in directions {
        for knot_idx in 0..knot_count {
            if knot_idx == 0 {
                coords[knot_idx] = apply_direction(&coords[knot_idx], dir);
            } else {
                coords[knot_idx] = follow_head(&coords[knot_idx - 1], &coords[knot_idx]);
            }
            if knot_idx == knot_count - 1 {
                tail_set.insert(coords[knot_idx]);
            }
        }
    }
    tail_set
}

pub fn part_one(input: &str) -> Option<u32> {
    let directions = parse(input);
    let tail_set = generate_tail_set(&directions, 2);
    Some(tail_set.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let directions = parse(input);
    let tail_set = generate_tail_set(&directions, 10);
    Some(tail_set.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
