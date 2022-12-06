fn find_marker_position(input: &str, marker_length: usize) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    chars
        .windows(marker_length)
        .enumerate()
        .find_map(|(idx, chars)| {
            let mut all_chars = chars.to_vec();
            all_chars.sort();
            all_chars.dedup();
            if all_chars.len() == marker_length {
                Some(idx)
            } else {
                None
            }
        })
}

pub fn part_one(input: &str) -> Option<usize> {
    let marker_size = 4;
    find_marker_position(input, marker_size).map(|idx| (idx + marker_size))
}

pub fn part_two(input: &str) -> Option<usize> {
    let marker_size = 14;
    find_marker_position(input, marker_size).map(|idx| (idx + marker_size))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26));
    }
}
