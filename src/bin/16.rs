use std::{collections::HashMap, cell::RefCell, cmp};

use regex::Regex;

struct Valve {
    flow: i64,
    mask: i64,
    tunnels: Vec<String>,
}

type ValveMap = HashMap<String, Valve>;

fn parse(input: &str) -> ValveMap {
    let valve_regex = Regex::new(r"Valve (?P<name>-?[A-Z]{2}) has flow rate=(?P<flow>-?\d+); tunnels? leads? to valves? (?P<tunnels>-?.+)").unwrap();
    let mut valve_map = ValveMap::new();
    let mut i: u32 = 0;
    input.to_string().lines().for_each (|line| {
        if ! valve_regex.is_match(&line) {
            panic!("parse_input(): unexpected input: {}", line);
        } else {
            let captures = valve_regex.captures(&line).unwrap();
            let name = captures["name"].to_string();
            let flow = captures["flow"].parse().unwrap();
            let tunnels = captures["tunnels"].split(", ").map(|s| s.to_string()).collect();
            let valve: Valve = Valve { flow, mask: i64::pow(2, i), tunnels };
            valve_map.insert(name, valve);
            i += 1;
        }
    });
    return valve_map;
}

fn calc_distances(valve_map: &ValveMap) -> HashMap<(String, String), RefCell<i64>> {
    let mut distances: HashMap<(String, String), RefCell<i64>> = HashMap::new();
    valve_map.keys().for_each(|x| {
        valve_map.keys().for_each(|y| {
            if valve_map.get(x).unwrap().tunnels.contains(y) {
                distances.entry((x.clone(), y.clone())).or_insert(RefCell::new(1));
            } else {
                distances.entry((x.clone(), y.clone())).or_insert(RefCell::new(i64::MAX));
            }
        });
    });
    valve_map.keys().for_each(|k| {
        valve_map.keys().for_each(|i| {
            valve_map.keys().for_each(|j| {
                let ij: i64;
                let ik: i64;
                let kj: i64;
                let tmp: i64;
                {
                    ij = *distances.get(&(i.clone(), j.clone())).unwrap().borrow();
                }
                {
                    ik = *distances.get(&(i.clone(), k.clone())).unwrap().borrow();
                }
                {
                    kj = *distances.get(&(k.clone(), j.clone())).unwrap().borrow();
                }
                if ik == i64::MAX || kj == i64::MAX {  // workaround to avoid overflow on addition
                    tmp = cmp::min(ij, i64::MAX);
                } else {
                    tmp = cmp::min(ij, ik + kj);
                }
                {
                    distances.insert((i.clone(), j.clone()), RefCell::new(tmp));
                }
            });
        });
    });
    return distances;
}

fn visit<'a>(
        valve: String,
        budget: i64,
        state: i64,
        valve_map: &ValveMap,
        distances: &HashMap<(String, String), RefCell<i64>>,
        flow: i64,
        answer: &'a mut HashMap<i64, i64>
        ) -> &'a mut HashMap<i64, i64> {
    let n: i64;
    if ! answer.contains_key(&state) {
        n = 0
    } else {
        n = *answer.get(&state).unwrap();
    }
    answer.insert(state, cmp::max(n, flow));
    for k in valve_map.iter().filter(|(_, cv)| cv.flow > 0).map(|(ck, _)| ck) {
        let dist: i64;
        {
            dist = *distances.get(&(valve.clone(), k.clone())).unwrap().borrow();
        }
        let new_budget = budget - dist - 1;
        let mask = valve_map.get(k).unwrap().mask;
        if (state & mask) != 0 || new_budget < 0 {
            continue;
        } else {
            let flow_here = valve_map.get(k).unwrap().flow;
            let _ = visit(k.clone(), new_budget, state | mask, &valve_map, &distances, flow + (new_budget * flow_here), answer);
        }
    }
    return answer;
}

pub fn part_one(input: &str) -> Option<i64> {
    let valve_map = parse(input);
    let distances = calc_distances(&valve_map);
    let state = 0;
    let mut answer = HashMap::new();
    let final_answer = visit(String::from("AA"), 30, state, &valve_map, &distances, 0, &mut answer);
    final_answer.values().cloned().max()
}

pub fn part_two(input: &str) -> Option<i64> {
    let valve_map = parse(&input);
    let distances = calc_distances(&valve_map);
    let state: i64 = 0;
    let mut answer = HashMap::new();
    let final_answer = visit(String::from("AA"), 26, state, &valve_map, &distances, 0, &mut answer);
    let mut total = 0;
    for (k1, v1) in final_answer.iter() {
        for (k2, v2) in final_answer.iter() {
            if (k1 & k2) == 0 {
                if v1 + v2 > total {
                    total = v1 + v2;
                }
            }
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
