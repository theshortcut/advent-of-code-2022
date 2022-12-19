use regex::Regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct Point2(i64, i64);

impl Point2 {
    fn distance_from(&self, other: &Point2) -> i64 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as i64
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Reach {
    from: i64,
    to: i64,
}

impl Reach {
    fn contains(&self, value: i64) -> bool {
        value >= self.from && value <= self.to
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point2,
    beacon: Point2,
    reach: i64,
}

type Edge = (Point2, Point2);

fn parse(input: &str) -> Vec<Sensor> {
    let re = Regex::new(
        r".*x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+).*x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)",
    ).unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let position = Point2(
                captures["sensor_x"].parse().unwrap(),
                captures["sensor_y"].parse().unwrap(),
            );
            let beacon = Point2(
                captures["beacon_x"].parse().unwrap(),
                captures["beacon_y"].parse().unwrap(),
            );
            let reach = position.distance_from(&beacon);
            Sensor {
                position,
                beacon,
                reach,
            }
        })
        .collect()
}

fn sensor_reaches_at_row(sensors: &[Sensor], y: i64) -> Vec<Reach> {
    sensors
        .iter()
        .filter_map(
            |Sensor {
                 position,
                 beacon: _,
                 reach,
             }| {
                let half_size = reach - (y - position.1).abs();
                let range_size = half_size * 2 + 1;
                if range_size > 0 {
                    Some(Reach {
                        from: position.0 - half_size,
                        to: position.0 + half_size,
                    })
                } else {
                    None
                }
            },
        )
        .collect()
}

fn get_row_coverage(input: &str, row: i64) -> u64 {
    let sensors = parse(input);
    let mut reaches = sensor_reaches_at_row(&sensors, row);
    reaches.sort_unstable_by_key(|r| r.from);
    let merged = merge_reaches(&reaches);

    // Sum the sizes of all these ranges, taking care of removing
    // each beacon known to be on this line
    let mut included_beacons = Vec::with_capacity(sensors.len());
    merged
        .iter()
        .map(|reach| {
            // there can be multiple beacons on a line
            beacons_in_reach(&sensors, reach, row, &mut included_beacons);
            (reach.to - reach.from + 1) as usize - included_beacons.len()
        })
        .sum::<usize>() as u64
}

fn merge_reaches(sorted_reaches: &[Reach]) -> Vec<Reach> {
    let mut result = Vec::with_capacity(sorted_reaches.len());
    let mut index = 0;
    let mut current = sorted_reaches.get(index).copied();
    loop {
        let next = sorted_reaches.get(index);
        index += 1;
        match (current, next) {
            (Some(r1), None) => {
                result.push(r1);
                return result;
            }
            (Some(r1), Some(r2)) if r1.contains(r2.from) => {
                current = Some(Reach {
                    to: r1.to.max(r2.to),
                    ..r1
                })
            }
            (Some(r1), Some(&r2)) => {
                if r1.to + 1 == r2.from {
                    current = Some(Reach {
                        to: r1.to.max(r2.to),
                        ..r1
                    })
                } else {
                    current = Some(r2);
                    result.push(r1);
                }
            }
            (None, _) => return result,
        }
    }
}

fn beacons_in_reach(sensors: &[Sensor], reach: &Reach, y: i64, into_vec: &mut Vec<Point2>) {
    into_vec.clear();
    for s in sensors {
        if s.beacon.1 == y && reach.contains(s.beacon.0) && !into_vec.contains(&s.beacon) {
            into_vec.push(s.beacon);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let row_coverage = get_row_coverage(input, 2_000_000);
    Some(row_coverage)
}

fn find_signal_freq(input: &str, max_pos: u64) -> u64 {
    let sensors = parse(input);
    let edges = edges(&sensors);
    let mut interesting_ys = Vec::with_capacity(edges.len() * edges.len() * 4);
    for i in 0..edges.len() {
        for j in 0..edges.len() {
            points_of_interest(&edges, i, j)
                .into_iter()
                .flatten()
                .for_each(|y| {
                    if y >= 0 && y <= max_pos as i64 {
                        interesting_ys.push(y);
                    }
                });
        }
    }
    let mut last_y = -1;
    for y in interesting_ys.into_iter() {
        if y == last_y {
            continue;
        }
        last_y = y;
        let mut ranges = sensor_reaches_at_row(&sensors, y);
        ranges.sort_unstable_by_key(|r| r.from);
        let merged = merge_reaches(&ranges);
        if merged.len() > 1 {
            let x = merged[0].to as u64 + 1;
            let y = y as u64;
            return 4000000 * x + y;
        }
    }
    unreachable!()
}

fn edges(sensors: &[Sensor]) -> Vec<Edge> {
    let mut edges = Vec::with_capacity(sensors.len() * 4);
    for Sensor {
        position: p,
        reach,
        beacon: _,
    } in sensors
    {
        let left = Point2(p.0 - reach, p.1);
        let right = Point2(p.0 + reach, p.1);
        let top = Point2(p.0, p.1 - reach);
        let bottom = Point2(p.0, p.1 + reach);
        edges.push((left, top));
        edges.push((bottom, right));
        edges.push((top, right));
        edges.push((left, bottom));
    }
    edges
}
fn points_of_interest(edges: &[Edge], index1: usize, index2: usize) -> [Option<i64>; 2] {
    if index1 == index2 {
        return [None; 2];
    }
    // we've been adding edges two by two having the same direction
    let edge1 = edges[index1];
    let edge2 = edges[index2];
    let params1 = line_params(edge1.0, edge1.1);
    let params2 = line_params(edge2.0, edge2.1);
    if params1.0 == params2.0 {
        // parallels
        return [None; 2];
    }
    let [y1, y2] = y_intersection(line_params(edge1.0, edge1.1), line_params(edge2.0, edge2.1));
    [
        y1.and_then(|y| (edge_contains_y(edge1, y) && edge_contains_y(edge2, y)).then_some(y)),
        y2.and_then(|y| (edge_contains_y(edge1, y) && edge_contains_y(edge2, y)).then_some(y)),
    ]
}

fn edge_contains_y((Point2(_, y1), Point2(_, y2)): Edge, y: i64) -> bool {
    y >= y1.min(y2) && y <= y1.max(y2)
}

fn line_params(p1: Point2, p2: Point2) -> (i64, i64) {
    let a = (p2.1 - p1.1) / (p2.0 - p1.0);
    debug_assert!(a == 1 || a == -1);
    let b = p1.1 - a * p1.0;
    (a, b)
}

fn y_intersection((a1, b1): (i64, i64), (a2, b2): (i64, i64)) -> [Option<i64>; 2] {
    let top = a1 * b2 - a2 * b1;
    let bottom = a1 - a2;
    if top % bottom == 0 {
        [Some(top / bottom), None]
    } else {
        let result = top / bottom;
        [Some(result), Some(result + 1)]
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(find_signal_freq(input, 4_000_000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        println!("{:?}", get_row_coverage(&input, 10));
        assert_eq!(get_row_coverage(&input, 10), 26);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(find_signal_freq(&input, 20), 56000011);
    }
}
