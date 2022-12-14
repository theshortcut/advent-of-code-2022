#![feature(map_many_mut)]

use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Operation {
    Add(u64),
    Mul(u64),
    Pow(u32),
}

#[derive(Debug, Clone, Copy)]
struct Item {
    worry: u64,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    decision_divisible_by: u64,
    true_target: usize,
    false_target: usize,
    item_inspection_count: usize,
}

fn parse(input: &str) -> HashMap<usize, Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let mut iter = m.lines().skip(1);
            let items = iter
                .next()
                .unwrap()
                .replace("Starting items:", "")
                .trim()
                .split(", ")
                .map(|s| Item {
                    worry: s.parse().unwrap(),
                })
                .collect();
            let operation = match iter
                .next()
                .unwrap()
                .replace("Operation: new = old", "")
                .trim()
                .split(' ')
                .collect_tuple()
                .unwrap()
            {
                ("*", "old") => Operation::Pow(2),
                ("*", s) => Operation::Mul(s.parse().unwrap()),
                ("+", "old") => Operation::Mul(2),
                ("+", s) => Operation::Add(s.parse().unwrap()),
                _ => unimplemented!(),
            };
            let decision_divisible_by = iter
                .next()
                .unwrap()
                .replace("Test: divisible by", "")
                .trim()
                .parse()
                .unwrap();
            let true_target = iter
                .next()
                .unwrap()
                .replace("If true: throw to monkey", "")
                .trim()
                .parse()
                .unwrap();
            let false_target = iter
                .next()
                .unwrap()
                .replace("If false: throw to monkey", "")
                .trim()
                .parse()
                .unwrap();
            Monkey {
                items,
                operation,
                decision_divisible_by,
                true_target,
                false_target,
                item_inspection_count: 0,
            }
        })
        .enumerate()
        .fold(HashMap::new(), |mut hm, (idx, monkey)| {
            hm.insert(idx, monkey);
            hm
        })
}

fn execute(monkeys: &mut HashMap<usize, Monkey>, rounds: usize, worry_decrease: bool) {
    let cd: u64 = monkeys
        .iter()
        .map(|(_, m)| m.decision_divisible_by)
        .product();
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let (true_target, false_target) = {
                let monkey = monkeys.get(&monkey_idx).unwrap();
                (monkey.true_target, monkey.false_target)
            };
            let [monkey, true_target, false_target] = monkeys
                .get_many_mut([&monkey_idx, &true_target, &false_target])
                .unwrap();
            for item in monkey.items.iter_mut() {
                use Operation::*;
                match monkey.operation {
                    Pow(x) => item.worry = item.worry.pow(x),
                    Mul(x) => item.worry *= x,
                    Add(x) => item.worry += x,
                }
                monkey.item_inspection_count += 1;
                if worry_decrease {
                    item.worry /= 3;
                } else {
                    item.worry %= cd;
                }
                if item.worry % monkey.decision_divisible_by == 0 {
                    true_target.items.push(*item);
                } else {
                    false_target.items.push(*item);
                }
            }
            monkey.items.clear();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);
    execute(&mut monkeys, 20, true);
    monkeys
        .values()
        .map(|m| m.item_inspection_count)
        .sorted()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);
    execute(&mut monkeys, 10000, false);
    monkeys
        .values()
        .map(|m| m.item_inspection_count)
        .sorted()
        .rev()
        .take(2)
        .reduce(|a, b| a * b)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
