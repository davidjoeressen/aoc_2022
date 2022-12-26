use std::collections::HashMap;
use std::str::FromStr;
use symbol::Symbol;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Op {
    Value(i64),
    Add(Symbol, Symbol),
    Subtract(Symbol, Symbol),
    Divide(Symbol, Symbol),
    Multiply(Symbol, Symbol),
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() == 1 {
            return Ok(Op::Value(parts[0].parse().map_err(|_| ())?));
        }
        if parts.len() != 3 {
            return Err(());
        }
        let lhs = Symbol::from(parts[0]);
        let rhs = Symbol::from(parts[2]);
        match parts[1] {
            "+" => Ok(Op::Add(lhs, rhs)),
            "-" => Ok(Op::Subtract(lhs, rhs)),
            "*" => Ok(Op::Multiply(lhs, rhs)),
            "/" => Ok(Op::Divide(lhs, rhs)),
            _ => Err(()),
        }
    }
}

impl Op {
    fn operands(&self) -> Option<(Symbol, Symbol)> {
        use Op::*;
        match self {
            Add(lhs, rhs) => Some((*lhs, *rhs)),
            Subtract(lhs, rhs) => Some((*lhs, *rhs)),
            Divide(lhs, rhs) => Some((*lhs, *rhs)),
            Multiply(lhs, rhs) => Some((*lhs, *rhs)),
            Value(_) => None,
        }
    }

    fn get_value(&self) -> Option<i64> {
        if let Op::Value(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    fn is_value(&self) -> bool {
        matches!(self, Op::Value(_))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Operations(HashMap<Symbol, Op>);

fn parse_line(s: &str) -> (Symbol, Op) {
    let (key, op) = s
        .split_once(": ")
        .expect("line should always contain a colon");
    (
        key.into(),
        op.parse().expect("parsing op should never fail"),
    )
}

impl FromStr for Operations {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ops = Operations(s.lines().map(parse_line).collect());
        ops.reduce();
        Ok(ops)
    }
}

impl Operations {
    fn calc(&self, key: Symbol) -> i64 {
        match self.0[&key] {
            Op::Value(n) => n,
            Op::Add(lhs, rhs) => self.calc(lhs) + self.calc(rhs),
            Op::Subtract(lhs, rhs) => self.calc(lhs) - self.calc(rhs),
            Op::Divide(lhs, rhs) => self.calc(lhs) / self.calc(rhs),
            Op::Multiply(lhs, rhs) => self.calc(lhs) * self.calc(rhs),
        }
    }

    fn calc_immediate(&self, key: Symbol) -> Option<i64> {
        let op = self.0[&key];
        if let Some((lhs, rhs)) = op.operands() {
            if lhs == "humn" || rhs == "humn" {
                return None;
            }
            let lhs = self.0[&lhs].get_value()?;
            let rhs = self.0[&rhs].get_value()?;
            match op {
                Op::Value(n) => Some(n),
                Op::Add(_, _) => Some(lhs + rhs),
                Op::Subtract(_, _) => Some(lhs - rhs),
                Op::Divide(_, _) => Some(lhs / rhs),
                Op::Multiply(_, _) => Some(lhs * rhs),
            }
        } else {
            op.get_value()
        }
    }

    fn reducable_keys(&self) -> Vec<Symbol> {
        let ignore: [Symbol; 2] = ["root".into(), "humn".into()];
        self.0
            .iter()
            .filter(|(k, v)| !ignore.contains(k) && !v.is_value())
            .map(|(k, _)| *k)
            .collect()
    }

    fn reduce(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for key in self.reducable_keys().iter() {
                if let Some(n) = self.calc_immediate(*key) {
                    self.0.entry(*key).and_modify(|it| *it = Op::Value(n));
                    changed = true;
                }
            }
        }
    }

    fn get_value(&self, key: Symbol) -> Option<i64> {
        if key == "humn" {
            None
        } else {
            self.0[&key].get_value()
        }
    }

    fn equate(&self, key: Symbol, value: i64) -> i64 {
        // println!("Checking {} == {}", key, value);
        if key == "humn" {
            return value;
        }
        let op = self.0[&key];
        match op {
            Op::Add(lhs, rhs) => {
                if let Some(n) = self.get_value(lhs) {
                    self.equate(rhs, value - n)
                } else if let Some(n) = self.get_value(rhs) {
                    self.equate(lhs, value - n)
                } else {
                    panic!("Neither lhs nor rhs are a value");
                }
            }
            Op::Subtract(lhs, rhs) => {
                if let Some(n) = self.get_value(lhs) {
                    self.equate(rhs, n - value)
                } else if let Some(n) = self.get_value(rhs) {
                    self.equate(lhs, value + n)
                } else {
                    panic!("Neither lhs nor rhs are a value");
                }
            }
            Op::Divide(lhs, rhs) => {
                if let Some(n) = self.get_value(lhs) {
                    self.equate(rhs, n / value)
                } else if let Some(n) = self.get_value(rhs) {
                    self.equate(lhs, value * n)
                } else {
                    panic!("Neither lhs nor rhs are a value");
                }
            }
            Op::Multiply(lhs, rhs) => {
                if let Some(n) = self.get_value(lhs) {
                    self.equate(rhs, value / n)
                } else if let Some(n) = self.get_value(rhs) {
                    self.equate(lhs, value / n)
                } else {
                    panic!("Neither lhs nor rhs are a value");
                }
            }
            Op::Value(n) => panic!("Can't equate Value({})", n),
        }
    }
}

pub fn part1(ops: &Operations) -> i64 {
    ops.calc("root".into())
}

pub fn part2(ops: Operations) -> i64 {
    let (lhs, rhs) = ops.0[&"root".into()]
        .operands()
        .expect("root should never be a Value");
    if let Some(n) = ops.0[&lhs].get_value() {
        ops.equate(rhs, n)
    } else if let Some(n) = ops.0[&rhs].get_value() {
        ops.equate(lhs, n)
    } else {
        panic!("Neither lhs nor rhs are a value");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        use Op::*;
        let have = "root: pppw + lgvd\n\
        dbpl: 5\n\
        ptdq: humn - pppw\n\
        pppw: ptdq / lgvd\n\
        lgvd: pppw * ptdq";
        let want = HashMap::from([
            ("root".into(), Add("pppw".into(), "lgvd".into())),
            ("dbpl".into(), Value(5)),
            ("ptdq".into(), Subtract("humn".into(), "pppw".into())),
            ("pppw".into(), Divide("ptdq".into(), "lgvd".into())),
            ("lgvd".into(), Multiply("pppw".into(), "ptdq".into())),
        ]);
        assert_eq!(have.parse(), Ok(Operations(want)));
    }

    #[test]
    fn solve_part1() {
        let have = "root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32";
        let data = have.parse().unwrap();
        assert_eq!(part1(&data), 152);
    }

    #[test]
    fn solve_part2() {
        let have = "root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32";
        let data = have.parse().unwrap();
        assert_eq!(part2(data), 301);
    }
}
