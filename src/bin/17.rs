use std::collections::HashSet;

type Coord = (i64, i64);

#[derive(Debug)]
enum RockShape {
    HorizontalLine,
    Diamond,
    BackwardsL,
    VerticalLine,
    Square,
}

impl RockShape {
    fn covered_coords(&self, coord: &Coord) -> Vec<Coord> {
        let mut coords = Vec::new();
        match self {
            RockShape::HorizontalLine => {
                for (x_delta, y_delta) in [(0, 0), (1, 0), (2, 0), (3, 0)] {
                    coords.push((coord.0 + x_delta, coord.1 + y_delta));
                }
            }
            RockShape::Diamond => {
                for (x_delta, y_delta) in [(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)] {
                    coords.push((coord.0 + x_delta, coord.1 + y_delta));
                }
            }
            RockShape::BackwardsL => {
                for (x_delta, y_delta) in [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)] {
                    coords.push((coord.0 + x_delta, coord.1 + y_delta));
                }
            }
            RockShape::VerticalLine => {
                for (x_delta, y_delta) in [(0, 0), (0, 1), (0, 2), (0, 3)] {
                    coords.push((coord.0 + x_delta, coord.1 + y_delta));
                }
            }
            RockShape::Square => {
                for (x_delta, y_delta) in [(0, 0), (0, 1), (1, 0), (1, 1)] {
                    coords.push((coord.0 + x_delta, coord.1 + y_delta));
                }
            }
        }
        coords
    }

    fn x_max(&self, coord: &Coord) -> i64 {
        self.covered_coords(coord)
            .iter()
            .map(|(x, _)| x)
            .max()
            .copied()
            .unwrap()
    }

    fn y_max(&self, coord: &Coord) -> i64 {
        self.covered_coords(coord)
            .iter()
            .map(|(_, y)| y)
            .max()
            .copied()
            .unwrap()
    }

    fn has_collisions(&self, coord: &Coord, coords: &HashSet<Coord>) -> bool {
        self.covered_coords(coord).iter().any(|c| coords.contains(c))
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

const FALL_ORDER: &[RockShape; 5] = {
    use RockShape::*;
    &[HorizontalLine, Diamond, BackwardsL, VerticalLine, Square]
};

const CHAMBER_WIDTH: i64 = 7;

fn parse(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        })
        .collect()
}

fn simulate(directions: &[Direction], rock_count: i64) -> i64 {
    let mut dirs = directions.iter().cycle();
    let mut shapes = FALL_ORDER.iter().cycle();
    let mut chamber_state: HashSet<Coord> = HashSet::new();
    let mut highest_point: i64 = 0;
    for _ in 0..rock_count {
        let shape = shapes.next().unwrap();
        let mut current_coord: Coord = (2, highest_point + 3);
        loop {
            let jet_direction = dirs.next().unwrap();
            match jet_direction {
                Direction::Left => {
                    if current_coord.0 > 0
                        && !shape.has_collisions(&(current_coord.0 - 1, current_coord.1), &chamber_state)
                    {
                        current_coord = (current_coord.0 - 1, current_coord.1);
                    }
                }
                Direction::Right => {
                    if shape.x_max(&current_coord) < CHAMBER_WIDTH - 1
                        && !shape.has_collisions(&(current_coord.0 + 1, current_coord.1), &chamber_state)
                    {
                        current_coord = (current_coord.0 + 1, current_coord.1);
                    }
                }
            }
            let pre_drop_coverage = shape.covered_coords(&current_coord);
            current_coord.1 -= 1;
            let dropped_coverage = shape.covered_coords(&current_coord);
            if current_coord.1 < 0 || dropped_coverage.iter().any(|c| chamber_state.contains(c)) {
                chamber_state.extend(&pre_drop_coverage);
                highest_point = highest_point.max(shape.y_max(&(current_coord.0, current_coord.1 + 1)) + 1);
                break;
            }
        }
    }
    highest_point
}

pub fn part_one(input: &str) -> Option<i64> {
    let directions = parse(input);
    Some(simulate(&directions, 2022))
}

pub fn part_two(input: &str) -> Option<i64> {
    let directions = parse(input);
    Some(simulate(&directions, 1_000_000_000_000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1_514_285_714_288));
    }
}
