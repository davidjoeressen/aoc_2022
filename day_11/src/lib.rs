use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Operation {
    Plus(i64),
    Times(i64),
    Square,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str> = s.split_whitespace().skip(1).collect();
        if pieces[1] == "old" {
            return Ok(Operation::Square);
        }
        let amount = pieces[1].parse().or(Err("Parse Error"))?;
        match pieces[0] {
            "*" => Ok(Operation::Times(amount)),
            "+" => Ok(Operation::Plus(amount)),
            _ => Err("Parse Error"),
        }
    }
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    divisible_by: i64,
    if_true: usize,
    if_false: usize,
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.lines().skip(1);
        let items = iter
            .next()
            .ok_or("Parse error")?
            .split_whitespace()
            .skip(2)
            .map(|x| x.trim().trim_end_matches(',').parse().unwrap())
            .collect();
        let operation = iter
            .next()
            .ok_or("Parse error")?
            .trim()
            .trim_start_matches("Operation: new = ")
            .parse()?;
        let divisible_by = iter
            .next()
            .and_then(|x| x.split_whitespace().last())
            .and_then(|x| x.parse().ok())
            .ok_or("Parse error")?;
        let if_true = iter
            .next()
            .and_then(|x| x.split_whitespace().last())
            .and_then(|x| x.parse().ok())
            .ok_or("Parse error")?;
        let if_false = iter
            .next()
            .and_then(|x| x.split_whitespace().last())
            .and_then(|x| x.parse().ok())
            .ok_or("Parse error")?;

        Ok(Monkey {
            items,
            operation,
            divisible_by,
            if_true,
            if_false,
        })
    }
}

pub fn parse(s: &str) -> Vec<Monkey> {
    s.split("\n\n").filter_map(|s| s.parse().ok()).collect()
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
    fn test_parse_monkey() {
        let have = "\
Monkey 0:
  Starting items: 98, 70, 75, 80, 84, 89, 55, 98
  Operation: new = old * 2
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 4";
        let want = Monkey {
            items: VecDeque::from([98, 70, 75, 80, 84, 89, 55, 98]),
            operation: Operation::Times(2),
            divisible_by: 11,
            if_true: 1,
            if_false: 4,
        };
        assert_eq!(have.parse(), Ok(want));
    }

    #[test]
    fn test_parse() {
        let have = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        assert_eq!(parse(have), get_test_monkeys());
    }

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
