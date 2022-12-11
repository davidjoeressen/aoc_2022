fn parse_crates(s: &str) -> Vec<Vec<char>> {
    let chars: Vec<Vec<char>> = s
        .lines()
        .map(|l| l.chars().skip(1).step_by(4).collect())
        .rev()
        .collect();
    let mut crates: Vec<Vec<char>> = Vec::new();
    for _ in &chars[1] {
        crates.push(Vec::new());
    }
    for row in chars.iter().skip(1) {
        for (n, c) in row.iter().enumerate() {
            if *c != ' ' {
                crates[n].push(*c);
            }
        }
    }
    crates
}

type Command = (i32, i32, i32);
fn parse_commands(s: &str) -> Vec<Command> {
    s.lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .step_by(2)
                .filter_map(|n| str::parse(n).ok())
                .collect::<Vec<i32>>()
        })
        .map(|a| {
            if let [x, y, z] = a.as_slice() {
                (*x, *y, *z)
            } else {
                (0, 0, 0)
            }
        })
        .collect()
}

pub fn parse(s: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    if let Some((crates, commands)) = s.split_once("\n\n") {
        (parse_crates(crates), parse_commands(commands))
    } else {
        (vec![], vec![])
    }
}

pub fn part1(mut crates: Vec<Vec<char>>, commands: &Vec<Command>) -> String {
    for (times, from, to) in commands {
        for _ in 0..*times {
            let tmp = crates[*from as usize - 1].pop().unwrap();
            crates[*to as usize - 1].push(tmp);
        }
    }
    crates.iter().map(|x| x.last().unwrap()).collect()
}

pub fn part2(mut crates: Vec<Vec<char>>, commands: &Vec<Command>) -> String {
    for (times, from, to) in commands {
        let from = *from as usize - 1;
        let to = *to as usize - 1;
        let split_point = crates[from].len() - *times as usize;
        let mut tmp = crates[from].split_off(split_point);
        crates[to].append(&mut tmp);
    }
    crates.iter().map(|x| x.last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_crates() {
        let input = "                    [L]     [H] [W]
                [J] [Z] [J] [Q] [Q]
[S]             [M] [C] [T] [F] [B]
[P]     [H]     [B] [D] [G] [B] [P]
[W]     [L] [D] [D] [J] [W] [T] [C]
[N] [T] [R] [T] [T] [T] [M] [M] [G]
[J] [S] [Q] [S] [Z] [W] [P] [G] [D]
[Z] [G] [V] [V] [Q] [M] [L] [N] [R]
 1   2   3   4   5   6   7   8   9";
        let want = vec![
            vec!['Z', 'J', 'N', 'W', 'P', 'S'],
            vec!['G', 'S', 'T'],
            vec!['V', 'Q', 'R', 'L', 'H'],
            vec!['V', 'S', 'T', 'D'],
            vec!['Q', 'Z', 'T', 'D', 'B', 'M', 'J'],
            vec!['M', 'W', 'T', 'J', 'D', 'C', 'Z', 'L'],
            vec!['L', 'P', 'M', 'W', 'G', 'T', 'J'],
            vec!['N', 'G', 'M', 'T', 'B', 'F', 'Q', 'H'],
            vec!['R', 'D', 'G', 'C', 'P', 'B', 'Q', 'W'],
        ];

        assert_eq!(parse_crates(input), want);
    }

    #[test]
    fn test_parse_command() {
        let input = "move 1 from 2 to 1\n\
                     move 3 from 1 to 3\n\
                     move 2 from 2 to 1\n\
                     move 1 from 1 to 2";
        let want = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        assert_eq!(parse_commands(input), want);
    }

    #[test]
    fn test_part1() {
        let crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let commands = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        assert_eq!(part1(crates, &commands), "CMZ");
    }

    #[test]
    fn test_part2() {
        let crates = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let commands = vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)];
        assert_eq!(part2(crates, &commands), "MCD");
    }
}
