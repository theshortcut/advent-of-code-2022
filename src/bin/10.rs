use itertools::Itertools;

#[derive(Debug)]
enum Command {
    Noop,
    AddX(i32),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            if let Some((cmd, val)) = line.split(' ').collect_tuple() {
                match (cmd, val) {
                    ("addx", v) => Command::AddX(v.parse().unwrap()),
                    _ => unreachable!(),
                }
            } else {
                Command::Noop
            }
        })
        .collect()
}

fn execute(cmds: &Vec<Command>) -> Vec<i32> {
    let mut x = 1;
    let mut results = vec![x];
    for cmd in cmds {
        match cmd {
            Command::Noop => {
                results.push(x);
            }
            Command::AddX(v) => {
                results.push(x);
                x += v;
                results.push(x);
            }
        }
    }
    results
}

fn draw(register: &[i32]) -> String {
    (0..6)
        .map(|y| {
            (0..40)
                .map(|x| {
                    let tick = x + (y * 40);
                    let x_val = register[tick];
                    if x_val.abs_diff(x as i32) < 2 {
                        "#"
                    } else {
                        "."
                    }
                })
                .collect_vec()
                .join("")
        })
        .collect_vec()
        .join("\n")
}

pub fn part_one(input: &str) -> Option<i32> {
    let parsed = parse(input);
    Some(
        execute(&parsed)
            .iter()
            .enumerate()
            .skip(19)
            .filter(|(i, _)| (i + 1) == 20 || (i + 1 - 20) % 40 == 0)
            .map(|(i, v)| (i + 1) as i32 * v)
            .sum::<i32>(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let parsed = parse(input);
    let register = execute(&parsed);
    Some(draw(&register))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                    .into()
            )
        );
    }
}
