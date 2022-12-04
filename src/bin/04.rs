use itertools::Itertools;

fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|elf| {
                    elf.split('-')
                        .map(|s| s.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    let count = input
        .iter()
        .filter(|(first_elf, second_elf)| {
            let (first_start, first_end) = first_elf;
            let (second_start, second_end) = second_elf;
            (first_start <= second_start && first_end >= second_end)
                || (second_start <= first_start && second_end >= first_end)
        })
        .count();
    count.try_into().ok()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    let count = input
        .iter()
        .filter(|(first_elf, second_elf)| {
            let (first_start, first_end) = first_elf;
            let (second_start, second_end) = second_elf;
            (first_end >= second_start && first_start <= second_start)
                || (second_end >= first_start && second_start <= first_start)
        })
        .count();
    count.try_into().ok()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
