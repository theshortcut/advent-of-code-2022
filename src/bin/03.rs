use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug)]
struct Compartment(Vec<char>);
#[derive(Debug)]
struct Rucksack(Vec<Compartment>);

#[derive(Debug)]
struct Priority(usize);
impl From<&char> for Priority {
    fn from(ch: &char) -> Priority {
        let lowercase = 'a'..='z';
        let uppercase = 'A'..='Z';
        Priority(
            lowercase
                .chain(uppercase)
                .find_position(|c| c == ch)
                .map(|(i, _)| i)
                .unwrap_or(0)
                + 1,
        )
    }
}

fn parse(input: &str) -> Vec<Rucksack> {
    input
        .lines()
        .map(|l| {
            Rucksack(
                l.chars()
                    .chunks(l.len() / 2)
                    .into_iter()
                    .map(|c| Compartment(c.collect()))
                    .collect(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut priority_sum = 0;
    for Rucksack(compartments) in parse(input) {
        let char_sets: Vec<HashSet<char>> = compartments
            .iter()
            .map(|Compartment(chars)| HashSet::from_iter(chars.iter().cloned()))
            .collect();
        let first = char_sets.first().unwrap();
        let second = char_sets.last().unwrap();
        let intersection = first.intersection(second);
        for common in intersection {
            let Priority(val) = common.into();
            priority_sum += val;
        }
    }
    Some(priority_sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut priority_sum = 0;
    for elf_group in parse(input).chunks(3) {
        let mut char_sets = Vec::new();
        for Rucksack(compartments) in elf_group {
            let char_set: HashSet<char> = HashSet::from_iter(
                compartments
                    .iter()
                    .flat_map(|Compartment(chars)| chars.iter().cloned()),
            );
            char_sets.push(char_set);
        }
        let intersection = char_sets
            .iter()
            .cloned()
            .reduce(|a, b| a.intersection(&b).copied().collect())
            .unwrap();
        for badge in intersection {
            let Priority(val) = (&badge).into();
            priority_sum += val;
        }
    }
    Some(priority_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
