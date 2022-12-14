use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

#[derive(Clone, Debug)]
pub enum Packet {
    List(Vec<Packet>),
    Int(i32),
}

impl PartialEq<Packet> for Packet {
    fn eq(&self, other: &Packet) -> bool {
        match (self, other) {
            (Packet::List(x), Packet::List(y)) => x.eq(y),
            (Packet::Int(x), Packet::Int(y)) => x.eq(y),
            (Packet::List(x), y) => x.eq(&vec![y.clone()]),
            (x, Packet::List(y)) => vec![x.clone()].eq(y),
        }
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::List(x), Packet::List(y)) => x.cmp(y),
            (Packet::Int(x), Packet::Int(y)) => x.cmp(y),
            (Packet::List(x), y) => x.cmp(&vec![y.clone()]),
            (x, Packet::List(y)) => vec![x.clone()].cmp(y),
        }
    }
}

fn skip(tokens: &mut Peekable<Tokens<'_>>, token: Token) -> Result<(), String> {
    if let Some(t) = tokens.next() {
        if t == token {
            Ok(())
        } else {
            Err(format!("Unexpected token {:?}, expected {:?}", t, token))
        }
    } else {
        Err(String::from("Unexpected end of input"))
    }
}

impl TryFrom<&mut Peekable<Tokens<'_>>> for Packet {
    type Error = String;
    fn try_from(tokens: &mut Peekable<Tokens>) -> Result<Self, Self::Error> {
        match tokens.next() {
            Some(Token::Int(n)) => Ok(Packet::Int(n)),
            Some(Token::BracketOpen) => {
                let mut list = Vec::new();
                while tokens.peek().map_or(false, |t| *t != Token::BracketClose) {
                    list.push(Packet::try_from(&mut *tokens)?);
                    if tokens.peek().map_or(true, |t| *t == Token::BracketClose) {
                        break;
                    }
                    skip(&mut *tokens, Token::Comma)?;
                }
                skip(&mut *tokens, Token::BracketClose)?;
                Ok(Packet::List(list))
            }
            Some(t) => Err(format!(
                "Unexpected token {:?}. Expected Int or BracketOpen.",
                t
            )),
            None => Err(String::from("Unexpected end of input")),
        }
    }
}

impl TryFrom<Tokens<'_>> for Packet {
    type Error = String;
    fn try_from(tokens: Tokens) -> Result<Self, Self::Error> {
        Packet::try_from(&mut tokens.peekable())
    }
}

impl FromStr for Packet {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Packet::try_from(tokens(s))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Token {
    BracketOpen,
    BracketClose,
    Comma,
    Int(i32),
}

#[derive(Debug, Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
struct Tokens<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.peek()? {
            '[' => {
                self.iter.next();
                Some(Token::BracketOpen)
            }
            ']' => {
                self.iter.next();
                Some(Token::BracketClose)
            }
            ',' => {
                self.iter.next();
                Some(Token::Comma)
            }
            '0'..='9' => {
                let mut number = String::new();
                while let Some(c) = self.iter.next_if(|c| ('0'..='9').contains(c)) {
                    number.push(c);
                }
                number.parse().ok().map(Token::Int)
            }
            _ => None,
        }
    }
}

fn tokens(s: &str) -> Tokens {
    Tokens {
        iter: s.chars().peekable(),
    }
}

pub fn parse(s: &str) -> Vec<Packet> {
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .collect::<Result<Vec<Packet>, String>>()
        .expect("Parse error")
}

pub fn part1(packets: &[Packet]) -> usize {
    let mut sum = 0;
    let mut iter = packets.iter().enumerate();
    while let Some((n, p)) = iter.next() {
        if *p <= *iter.next().unwrap().1 {
            sum += n / 2 + 1;
        }
    }
    sum
}

pub fn part2(mut packets: Vec<Packet>) -> usize {
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);
    packets.push(divider1.clone());
    packets.push(divider2.clone());
    packets.sort();
    let mut key = 1;
    for (n, packet) in packets.iter().enumerate() {
        if *packet == divider1 {
            key *= n + 1;
        }
        if *packet == divider2 {
            key *= n + 1;
            break;
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        use Packet::*;
        let tests = vec![
            ("[]", List(vec![])),
            ("[3]", List(vec![Int(3)])),
            ("[[[]]]", List(vec![List(vec![List(vec![])])])),
            (
                "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                List(vec![
                    Int(1),
                    List(vec![
                        Int(2),
                        List(vec![
                            Int(3),
                            List(vec![Int(4), List(vec![Int(5), Int(6), Int(7)])]),
                        ]),
                    ]),
                    Int(8),
                    Int(9),
                ]),
            ),
        ];
        for (have, want) in tests {
            assert_eq!(have.parse(), Ok(want));
        }
    }

    #[test]
    fn test_part1() {
        let have = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let data = parse(have);
        assert_eq!(part1(&data), 13);
        assert_eq!(part2(data), 140);
    }
}
