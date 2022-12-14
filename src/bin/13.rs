use std::cmp;

type Packet = Option<serde_json::Value>;

fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter_map(|l| (serde_json::from_str(l).ok()))
        .collect()
}

pub fn arr_len(a: &serde_json::Value) -> usize {
    a.as_array().unwrap().len()
}

pub fn wrap_arr(a: &serde_json::Value) -> serde_json::Value {
    serde_json::Value::Array(vec![a.clone()])
}

fn validate_pos(packet_a: &Packet, packet_b: &Packet) -> (bool, bool) {
    match (packet_a, packet_b) {
        (None, None) => unreachable!(),
        (None, Some(_)) => (true, true),
        (Some(_), None) => (false, true),
        (Some(left), Some(right)) => {
            if left.is_number() && right.is_number() {
                let a = left.as_u64().unwrap();
                let b = right.as_u64().unwrap();
                (a <= b, a != b)
            } else if left.is_number() && right.is_array() {
                validate_pos(&Some(wrap_arr(left)), &Some(right.clone()))
            } else if left.is_array() && right.is_number() {
                validate_pos(&Some(left.clone()), &Some(wrap_arr(right)))
            } else if left.is_array() && right.is_array() {
                let mut is_valid = true;
                let mut is_finished = false;
                let mut i = 0;

                let max_len = cmp::max(arr_len(left), arr_len(right));

                while is_valid && !is_finished && i < max_len {
                    (is_valid, is_finished) =
                        validate_pos(&left.get(i).cloned(), &right.get(i).cloned());
                    i += 1;
                }

                (is_valid, is_finished)
            } else {
                unreachable!()
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse(input)
            .chunks(2)
            .into_iter()
            .enumerate()
            .filter(|(_, pair)| validate_pos(&pair[0], &pair[1]).0)
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets = parse(input);
    let dividers = vec!["[[2]]".to_string(), "[[6]]".to_string()];
    packets.extend(dividers.iter().map(|s| serde_json::from_str(s).ok()));

    packets.sort_by(|a, b| {
        let (valid, _) = validate_pos(a, b);
        if valid {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    });

    Some(packets.iter().enumerate().fold(1, |acc, (i, curr)| {
        if dividers.contains(&serde_json::to_string(curr).unwrap()) {
            acc * (i + 1)
        } else {
            acc
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
