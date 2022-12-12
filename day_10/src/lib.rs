#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Noop,
    AddX(i32),
}

fn parse_line(s: &str) -> Command {
    let mut command = s.split_whitespace();
    match command.next() {
        Some("noop") => Command::Noop,
        Some("addx") => Command::AddX(command.next().and_then(|x| str::parse(x).ok()).unwrap()),
        _ => panic!("Parse error"),
    }
}

pub fn parse(s: &str) -> Vec<Command> {
    s.lines().map(parse_line).collect()
}

pub fn calc_values(commands: &[Command]) -> Vec<i32> {
    commands
        .iter()
        .flat_map(|x| match x {
            Command::Noop => vec![0],
            Command::AddX(n) => vec![0, *n],
        })
        .scan(1, |state, x| {
            let tmp = *state;
            *state += x;
            Some(tmp)
        })
        .collect()
}

pub fn part1(values: &[i32]) -> i32 {
    vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|&n| n as i32 * values[n - 1])
        .sum()
}

pub fn part2(values: &[i32]) -> Vec<String> {
    let lines: String = values
        .iter()
        .enumerate()
        .map(|(n, &x)| {
            let n = n as i32 % 40;
            if n == x - 1 || n == x || n == x + 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect();
    vec![0, 40, 80, 120, 160, 200]
        .iter()
        .map(|&n| String::from(&lines[n..n + 40]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = vec![
            ("noop", Command::Noop),
            ("addx 5", Command::AddX(5)),
            ("addx -25", Command::AddX(-25)),
        ];
        for (have, want) in tests {
            assert_eq!(parse_line(have), want);
        }
    }
}
