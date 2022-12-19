use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use symbol::Symbol;

pub type Vertex = Symbol;
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Valve {
    vertex: Vertex,
    flow_rate: i32,
    targets: Vec<Vertex>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Valves {
    valves: HashMap<Vertex, Valve>,
    candidates: HashSet<Vertex>,
    distances: HashMap<(Vertex, Vertex), i32>,
}

impl FromStr for Valve {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Valve ").ok_or("Must start with 'Valve '")?;
        let (vertex, s) = s
            .split_once(" has flow rate=")
            .ok_or("First split failed")?;
        let vertex = vertex.into();
        let (flow_rate, s) = s
            .split_once("; tunnels lead to valves ")
            .or_else(|| s.split_once("; tunnel leads to valve "))
            .ok_or("Second split failed")?;
        let flow_rate = flow_rate.parse().map_err(|_| "Parsing flow rate failed")?;
        let targets = s.split(", ").map(|s| s.into()).collect();
        Ok(Valve {
            vertex,
            flow_rate,
            targets,
        })
    }
}

pub fn parse(s: &str) -> Valves {
    let valves: HashMap<Vertex, Valve> = s
        .lines()
        .map(|s| s.parse().expect("Parse error"))
        .map(|valve: Valve| (valve.vertex, valve))
        .collect();
    let candidates: HashSet<Vertex> = valves
        .values()
        .filter(|valve| valve.flow_rate > 0)
        .map(|valve| valve.vertex)
        .collect();
    let distances: HashMap<(Vertex, Vertex), i32> = candidates
        .union(&HashSet::from(["AA".into()]))
        .flat_map(|&candidate| {
            find_shortest_paths(candidate, &valves)
                .iter()
                .filter(|(vert, _)| candidates.contains(vert))
                .map(|(&vert, &distance)| ((candidate, vert), distance))
                .collect::<Vec<((Vertex, Vertex), i32)>>()
        })
        .collect();
    Valves {
        valves,
        candidates,
        distances,
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct State {
    positions: Vec<(Vertex, i32)>,
    valves_open: HashSet<Vertex>,
    valves_opened: Vec<(i32, i32)>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, _: &Self) -> Ordering {
        Ordering::Equal
    }
}

impl State {
    fn new(positions: Vec<(Vertex, i32)>) -> Self {
        Self {
            positions,
            valves_open: HashSet::new(),
            valves_opened: Vec::new(),
        }
    }

    fn calc(&self) -> i32 {
        self.valves_opened
            .iter()
            .map(|&(flow_rate, minutes_left)| flow_rate * minutes_left)
            .sum()
    }

    fn theoretical_max(&self, valves: &Valves) -> i32 {
        let current = self.calc();
        let max_minutes_left = self
            .positions
            .iter()
            .map(|&(_, minutes_left)| minutes_left)
            .min()
            .unwrap_or(0);
        let remaining_valves: i32 = valves
            .candidates
            .difference(&self.valves_open)
            .map(|candidate| valves.valves[candidate].flow_rate)
            .sum();
        current + max_minutes_left * remaining_valves
    }
}

fn find_shortest_paths(from: Symbol, valves: &HashMap<Vertex, Valve>) -> HashMap<Symbol, i32> {
    let mut queue = BinaryHeap::new();
    let mut distances = HashMap::from([(from, 0)]);
    queue.push((Reverse(0), from));
    while let Some((_, check)) = queue.pop() {
        let next_distance = distances[&check] + 1;
        for target in valves[&check].targets.iter() {
            if distances.get(target).map_or(true, |&n| n > next_distance) {
                distances
                    .entry(*target)
                    .and_modify(|x| *x = next_distance)
                    .or_insert(next_distance);
                queue.push((Reverse(next_distance), *target));
            }
        }
    }
    distances
}

fn get_max(start: State, valves: &Valves) -> i32 {
    let mut queue: BinaryHeap<(i32, State)> = BinaryHeap::new();
    let mut current_best = 0;
    queue.push((start.theoretical_max(valves), start));
    while let Some((theoretical_max, state)) = queue.pop() {
        if theoretical_max < current_best {
            return current_best;
        }
        if state.calc() > current_best {
            current_best = state.calc();
        }
        for position in 0..state.positions.len() {
            for &valve in valves.candidates.difference(&state.valves_open) {
                let mut next = state.clone();
                let distance = valves.distances[&(next.positions[position].0, valve)];
                let minutes_left = next.positions[position].1 - distance - 1;
                next.positions[position] = (valve, minutes_left.max(0));
                if minutes_left >= 0 {
                    next.valves_open.insert(valve);
                    next.valves_opened
                        .push((valves.valves[&valve].flow_rate, minutes_left));
                }
                let theoretical_max = next.theoretical_max(valves);
                if theoretical_max > current_best {
                    queue.push((next.theoretical_max(valves), next));
                }
            }
        }
    }
    current_best
}

pub fn part1(valves: &Valves) -> i32 {
    get_max(State::new(vec![("AA".into(), 30)]), valves)
}

pub fn part2(valves: &Valves) -> i32 {
    get_max(
        State::new(vec![("AA".into(), 26), ("AA".into(), 26)]),
        valves,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let have = "Valve HK has flow rate=10; tunnels lead to valves DF, AA";
        let want = Valve {
            vertex: "HK".into(),
            flow_rate: 10,
            targets: vec!["DF".into(), "AA".into()],
        };
        assert_eq!(have.parse(), Ok(want));
    }

    fn get_data() -> Valves {
        let have = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
            Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
            Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
            Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
            Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
            Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
            Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
            Valve HH has flow rate=22; tunnel leads to valve GG\n\
            Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
            Valve JJ has flow rate=21; tunnel leads to valve II";
        parse(have)
    }

    #[test]
    fn can_find_shortest_path() {
        let data = get_data();
        let result: HashMap<Symbol, i32> = find_shortest_paths("AA".into(), &data.valves);
        assert_eq!(result[&Symbol::from("JJ")], 2);
    }

    #[test]
    fn test_part1() {
        let data = get_data();
        assert_eq!(part1(&data), 1651);
    }

    #[test]
    fn test_part2() {
        let data = get_data();
        assert_eq!(part2(&data), 1707);
    }
}
