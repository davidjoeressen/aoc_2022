use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Plus(i64),
    Times(i64),
    Square,
}

struct ModDiv(HashMap<i64, i64>);

impl ModDiv {
    fn from(n: i64, divisors: &[i64]) -> ModDiv {
        ModDiv(divisors.iter().map(|&d| (d, n % d)).collect())
    }

    fn plus(&mut self, y: i64) {
        for (d, n) in self.0.iter_mut() {
            *n = (*n + (y % d)) % d;
        }
    }
    fn times(&mut self, y: i64) {
        for (d, n) in self.0.iter_mut() {
            *n = (*n * (y % d)) % d;
        }
    }
    fn square(&mut self) {
        for (d, n) in self.0.iter_mut() {
            *n = (*n * *n) % d;
        }
    }
}

impl Operation {
    fn apply(&self, other: i64) -> i64 {
        match self {
            Operation::Plus(n) => other + *n,
            Operation::Times(n) => other * *n,
            Operation::Square => other * other,
        }
    }

    fn apply_mod(&self, other: &mut ModDiv) {
        match self {
            Operation::Plus(n) => other.plus(*n),
            Operation::Times(n) => other.times(*n),
            Operation::Square => other.square(),
        };
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    divisible_by: i64,
    if_true: usize,
    if_false: usize,
}

pub fn get_monkeys() -> Vec<Monkey> {
    // Todo: parse
    vec![
        Monkey {
            items: VecDeque::from([98, 70, 75, 80, 84, 89, 55, 98]),
            operation: Operation::Times(2),
            divisible_by: 11,
            if_true: 1,
            if_false: 4,
        },
        Monkey {
            items: VecDeque::from([59]),
            operation: Operation::Square,
            divisible_by: 19,
            if_true: 7,
            if_false: 3,
        },
        Monkey {
            items: VecDeque::from([77, 95, 54, 65, 89]),
            operation: Operation::Plus(6),
            divisible_by: 7,
            if_true: 0,
            if_false: 5,
        },
        Monkey {
            items: VecDeque::from([71, 64, 75]),
            operation: Operation::Plus(2),
            divisible_by: 17,
            if_true: 6,
            if_false: 2,
        },
        Monkey {
            items: VecDeque::from([74, 55, 87, 98]),
            operation: Operation::Times(11),
            divisible_by: 3,
            if_true: 1,
            if_false: 7,
        },
        Monkey {
            items: VecDeque::from([90, 98, 85, 52, 91, 60]),
            operation: Operation::Plus(7),
            divisible_by: 5,
            if_true: 0,
            if_false: 4,
        },
        Monkey {
            items: VecDeque::from([99, 51]),
            operation: Operation::Plus(1),
            divisible_by: 13,
            if_true: 5,
            if_false: 2,
        },
        Monkey {
            items: VecDeque::from([98, 94, 59, 76, 51, 65, 75]),
            operation: Operation::Plus(5),
            divisible_by: 2,
            if_true: 3,
            if_false: 6,
        },
    ]
}

pub fn part1(mut monkeys: Vec<Monkey>) -> i64 {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..20 {
        for n in 0..monkeys.len() {
            while let Some(item) = monkeys[n].items.pop_front() {
                inspections[n] += 1;
                let item = monkeys[n].operation.apply(item) / 3;
                let next_monkey = if item % monkeys[n].divisible_by == 0 {
                    monkeys[n].if_true
                } else {
                    monkeys[n].if_false
                };
                monkeys[next_monkey].items.push_back(item);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

pub fn part2(monkeys: Vec<Monkey>) -> i64 {
    let mut inspections = vec![0; monkeys.len()];
    let divisors: Vec<i64> = monkeys.iter().map(|m| m.divisible_by).collect();
    let mut items: Vec<VecDeque<ModDiv>> = monkeys
        .iter()
        .map(|m| {
            m.items
                .iter()
                .map(|i| ModDiv::from(*i, &divisors))
                .collect()
        })
        .collect();
    for _ in 0..10_000 {
        for n in 0..monkeys.len() {
            while let Some(mut item) = items[n].pop_front() {
                inspections[n] += 1;
                monkeys[n].operation.apply_mod(&mut item);
                let next_monkey = if item.0[&monkeys[n].divisible_by] == 0 {
                    monkeys[n].if_true
                } else {
                    monkeys[n].if_false
                };
                items[next_monkey].push_back(item);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_test_monkeys()), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_test_monkeys()), 2713310158);
    }

    fn get_test_monkeys() -> Vec<Monkey> {
        vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                operation: Operation::Times(19),
                divisible_by: 23,
                if_true: 2,
                if_false: 3,
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operation: Operation::Plus(6),
                divisible_by: 19,
                if_true: 2,
                if_false: 0,
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operation: Operation::Square,
                divisible_by: 13,
                if_true: 1,
                if_false: 3,
            },
            Monkey {
                items: VecDeque::from([74]),
                operation: Operation::Plus(3),
                divisible_by: 17,
                if_true: 0,
                if_false: 1,
            },
        ]
    }
}
