use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Move {
    count: i32,
    from: usize,
    to: usize,
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (stacks_str, moves_str) = input.split("\n\n").collect_tuple().unwrap();

    let mut stacks = Vec::new();
    for line in stacks_str.lines().rev().skip(1) {
        for (stack_idx, char_chunk) in line.chars().chunks(4).into_iter().enumerate() {
            if stacks.len() <= stack_idx {
                stacks.push(Vec::new());
            }
            let mut char_chunk = char_chunk;
            let stack = stacks.get_mut(stack_idx);
            match (char_chunk.nth(1), stack) {
                (Some(ch), Some(stack)) if ch != ' ' => stack.push(ch),
                _ => (),
            }
        }
    }

    let mut moves = Vec::new();
    for line in moves_str.lines() {
        let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let captures = re.captures(line).unwrap();
        moves.push(Move {
            count: captures["count"].parse().unwrap(),
            from: captures["from"].parse().unwrap(),
            to: captures["to"].parse().unwrap(),
        });
    }

    (stacks, moves)
}

fn apply_moves_one(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for Move { count, from, to } in moves {
        for _ in 0..*count {
            let val = {
                let from_stack = stacks.get_mut(from - 1).unwrap();
                from_stack.pop().unwrap()
            };
            let to_stack = stacks.get_mut(to - 1).unwrap();
            to_stack.push(val);
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);
    apply_moves_one(&mut stacks, &moves);
    Some(stacks.iter().filter_map(|s| s.last()).join(""))
}

fn apply_moves_two(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for Move { count, from, to } in moves {
        let mut moving = {
            let from_stack = stacks.get_mut(from - 1).unwrap();
            let split_point = from_stack.len() - *count as usize;
            from_stack.drain(split_point..).collect_vec()
        };
        let to_stack = stacks.get_mut(to - 1).unwrap();
        to_stack.append(&mut moving);
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse(input);
    apply_moves_two(&mut stacks, &moves);
    Some(stacks.iter().filter_map(|s| s.last()).join(""))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
