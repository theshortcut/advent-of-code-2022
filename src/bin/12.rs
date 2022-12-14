use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
enum Tile {
    Path(u32),
    Start,
    End,
}

#[derive(Debug)]
struct HeightMap {
    tiles: Vec<Vec<Tile>>,
}

impl HeightMap {
    fn bfs(&self, start: (usize, usize)) -> Option<usize> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, 0));

        while let Some((current_coord, dist)) = queue.pop_front() {
            if current_coord == self.get_end() {
                return Some(dist);
            }

            for neighbor in self.neighbors(current_coord) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        None
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let tile = self.tiles.get(y).unwrap().get(x).unwrap();
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .filter_map(|dir| {
            if *dir == Direction::Up && y == 0 {
                return None;
            }
            if *dir == Direction::Left && x == 0 {
                return None;
            }
            let (nx, ny) = match dir {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
            match self.tiles.get(ny).map(|row| row.get(nx)) {
                Some(neighbor) => match (&tile, neighbor) {
                    (Tile::Start, Some(Tile::Path(a))) if a < &2 => Some((nx, ny)),
                    (Tile::Path(a), Some(Tile::Path(b))) if b < a || a.abs_diff(*b) < 2 => {
                        Some((nx, ny))
                    }
                    (Tile::Path(a), Some(Tile::End)) if a > &24 => Some((nx, ny)),
                    _ => None,
                },
                _ => None,
            }
        })
        .collect()
    }

    fn get_start(&self) -> (usize, usize) {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if tile == &Tile::Start {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn get_end(&self) -> (usize, usize) {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if tile == &Tile::End {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn get_lowest_points(&self) -> Vec<(usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, tile)| match tile {
                        Tile::Start => Some((x, y)),
                        Tile::Path(0) => Some((x, y)),
                        _ => None,
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect()
    }
}

fn parse(input: &str) -> HeightMap {
    let tiles = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    c => Tile::Path("abcdefghijklmnopqrstuvwxyz".find(c).unwrap() as u32),
                })
                .collect()
        })
        .collect();
    HeightMap { tiles }
}

pub fn part_one(input: &str) -> Option<usize> {
    let height_map = parse(input);
    height_map.bfs(height_map.get_start())
}

pub fn part_two(input: &str) -> Option<usize> {
    let height_map = parse(input);
    height_map
        .get_lowest_points()
        .iter()
        .map(|s| height_map.bfs(*s))
        .reduce(|acc, steps| match (acc, steps) {
            (None, Some(s)) => Some(s),
            (Some(a), Some(s)) if s < a => Some(s),
            _ => acc,
        })
        .unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
