use std::collections::HashSet;

use itertools::Itertools;

type Coord = (usize, usize);
enum DropDirection {
    Down,
    DownLeft,
    DownRight,
}

#[derive(Debug, Clone)]
struct CoordSet {
    wall_set: HashSet<Coord>,
    sand_set: HashSet<Coord>,
}

impl CoordSet {
    fn valid_drop_destination(&self, start: &Coord) -> Option<Coord> {
        use DropDirection::*;
        for dir in [Down, DownLeft, DownRight] {
            let destination = match dir {
                Down => (start.0, start.1 + 1),
                DownLeft => (start.0 - 1, start.1 + 1),
                DownRight => (start.0 + 1, start.1 + 1),
            };
            if !self.wall_set.contains(&destination) && !self.sand_set.contains(&destination) {
                return Some(destination);
            }
        }
        None
    }

    fn valid_drop_destination_with_floor(&self, start: &Coord, floor: usize) -> Option<Coord> {
        use DropDirection::*;
        for dir in [Down, DownLeft, DownRight] {
            let destination = match dir {
                Down => (start.0, start.1 + 1),
                DownLeft => (start.0 - 1, start.1 + 1),
                DownRight => (start.0 + 1, start.1 + 1),
            };
            if !self.wall_set.contains(&destination)
                && !self.sand_set.contains(&destination)
                && !destination.1 >= floor
            {
                return Some(destination);
            }
        }
        None
    }
}

fn parse(input: &str) -> CoordSet {
    let mut wall_set = HashSet::new();
    for line in input.lines() {
        let mut points = vec![];
        for point in line.split("->") {
            let coord: Coord = point
                .split(',')
                .map(|d| d.trim().parse().unwrap())
                .collect_tuple()
                .unwrap();
            points.push(coord);
        }
        for point_set in points.windows(2) {
            let (start_x, end_x) = if point_set[0].0 < point_set[1].0 {
                (point_set[0].0, point_set[1].0)
            } else {
                (point_set[1].0, point_set[0].0)
            };
            let (start_y, end_y) = if point_set[0].1 < point_set[1].1 {
                (point_set[0].1, point_set[1].1)
            } else {
                (point_set[1].1, point_set[0].1)
            };
            for x in start_x..=end_x {
                wall_set.insert((x, start_y));
            }
            for y in start_y..=end_y {
                wall_set.insert((start_x, y));
            }
        }
    }
    CoordSet {
        wall_set,
        sand_set: HashSet::new(),
    }
}

fn simulate_sandfall(coord_set: &CoordSet) -> HashSet<Coord> {
    let mut coord_set = coord_set.clone();
    let mut sand_coord: Coord = (500, 0);
    let mut last_sand_coord = (0, 0);
    let y_max = coord_set.wall_set.iter().map(|(_, y)| y).max().unwrap();
    while last_sand_coord != sand_coord && sand_coord.1 < *y_max {
        last_sand_coord = sand_coord;
        match coord_set.valid_drop_destination(&sand_coord) {
            Some(destination) => {
                sand_coord = destination;
            }
            None => {
                // sand has come to rest
                coord_set.sand_set.insert(sand_coord);
                sand_coord = (500, 0);
            }
        }
    }
    coord_set.sand_set
}

fn simulate_sandfall_with_floor(coord_set: &CoordSet) -> HashSet<Coord> {
    let mut coord_set = coord_set.clone();
    let mut sand_coord: Coord = (500, 0);
    let mut last_sand_coord = (0, 0);
    let y_max = coord_set.wall_set.iter().map(|(_, y)| y).max().unwrap();
    while last_sand_coord != (500, 0) || sand_coord != (500, 0) {
        last_sand_coord = sand_coord;
        match coord_set.valid_drop_destination_with_floor(&sand_coord, y_max + 2) {
            Some(destination) => {
                sand_coord = destination;
            }
            None => {
                // sand has come to rest
                coord_set.sand_set.insert(sand_coord);
                sand_coord = (500, 0);
            }
        }
    }
    coord_set.sand_set
}

pub fn part_one(input: &str) -> Option<usize> {
    let coord_set = parse(input);
    let sand_set = simulate_sandfall(&coord_set);
    Some(sand_set.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let coord_set = parse(input);
    let sand_set = simulate_sandfall_with_floor(&coord_set);
    Some(sand_set.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
