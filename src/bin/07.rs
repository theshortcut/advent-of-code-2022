use indextree::{Arena, NodeId};

#[derive(Clone)]
pub struct Node<'a> {
    name: &'a str,
    size: u32,
}

fn parse(input: &str) -> Arena<Node>{
    let mut arena = Arena::new();
    let mut current_id = arena.new_node(Node { name: "/", size: 0 });

    input
        .split("$ ")
        .skip(2)
        .map(|chunk| {
            let (cmd, rest) = chunk.split_at(2);
            (cmd, rest.trim())
        })
        .try_for_each(|cmd| {
            match cmd {
                ("cd", "..") => {
                    current_id = arena.get(current_id)?.parent()?;
                }
                ("cd", dir) => {
                    current_id = current_id
                        .children(&arena)
                        .find(|id| arena.get(*id).unwrap().get().name == dir)?;
                }
                ("ls", rest) => {
                    rest.lines().try_for_each(|l| {
                        let (size, name) = l.split_once(' ')?;
                        if size == "dir" {
                            let id = arena.new_node(Node { name, size: 0 });
                            current_id.append(id, &mut arena);
                        } else {
                            let size = size.parse::<u32>().ok()?;
                            current_id
                                .ancestors(&arena)
                                .collect::<Vec<NodeId>>()
                                .into_iter()
                                .for_each(|id| {
                                    arena.get_mut(id).unwrap().get_mut().size += size;
                                })
                        }
                        Some(())
                    });
                }
                _ => unreachable!(),
            }

            Some(())
        });

    arena
}

pub fn part_one(input: &str) -> Option<u32> {
    let arena = parse(input);
    Some(
        arena
            .iter()
            .map(|entry| entry.get().size)
            .filter(|size| *size < 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let arena = parse(input);
    let mut values = arena.iter().map(|entry| entry.get().size);
    let total_size = values.next()?;
    let needed = total_size + 30000000 - 70000000;
    values.filter(|x| *x >= needed).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
