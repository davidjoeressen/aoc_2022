use core::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

fn choose_losing(hand: &Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn choose_winning(hand: &Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => Err("Invalid string"),
        }
    }
}

fn parse_line(s: &str) -> Result<(Hand, Hand), &'static str> {
    let hands: Vec<Hand> = s
        .split_whitespace()
        .take(2)
        .map(|x| str::parse::<Hand>(x))
        .collect::<Result<Vec<Hand>, &str>>()?;
    if let [oponent, player] = hands.as_slice() {
        Ok((*oponent, *player))
    } else {
        Err("Couldn't parse line")
    }
}

fn hand_score(hand: &Hand) -> i64 {
    match hand {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }
}

fn calc_score(oponent: &Hand, player: &Hand) -> i64 {
    let player_score = hand_score(player);
    let oponent_score = hand_score(oponent);

    let game_score = if player_score == oponent_score {
        3
    } else if oponent_score % 3 + 1 == player_score {
        6
    } else {
        0
    };

    player_score + game_score
}

fn calc_p2_score(oponent: &Hand, player: &Hand) -> i64 {
    match player {
        Hand::Rock => calc_score(oponent, &choose_losing(oponent)),
        Hand::Paper => calc_score(oponent, oponent),
        Hand::Scissors => calc_score(oponent, &choose_winning(oponent)),
    }
}

pub fn parse_file(s: String) -> Vec<(Hand, Hand)> {
    s.lines().filter_map(|l| parse_line(l).ok()).collect()
}

pub fn part1(game: &Vec<(Hand, Hand)>) -> i64 {
    game.iter()
        .map(|(oponent, player)| calc_score(&oponent, &player))
        .sum()
}

pub fn part2(game: &Vec<(Hand, Hand)>) -> i64 {
    game.iter()
        .map(|(oponent, player)| calc_p2_score(&oponent, &player))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores() {
        let tests = vec![
            ((Hand::Rock, Hand::Paper), 8),
            ((Hand::Paper, Hand::Rock), 1),
            ((Hand::Scissors, Hand::Scissors), 6),
        ];

        for ((player, oponent), want) in tests {
            assert_eq!(calc_score(&player, &oponent), want);
        }
    }

    #[test]
    fn parse_hand() {
        let tests = vec![
            ("A", Hand::Rock),
            ("X", Hand::Rock),
            ("B", Hand::Paper),
            ("Y", Hand::Paper),
            ("C", Hand::Scissors),
            ("Z", Hand::Scissors),
        ];

        for (s, hand) in tests {
            assert_eq!(str::parse::<Hand>(s), Ok(hand));
        }
    }

    #[test]
    fn can_parse_line() {
        assert_eq!(parse_line("A Y"), Ok((Hand::Rock, Hand::Paper)));
    }

    #[test]
    fn can_run_game() {
        let file = String::from(
            "A Y\n\
            B X\n\
            C Z\n",
        );
        let game = parse_file(file);
        assert_eq!(part1(&game), 15);
        assert_eq!(part2(&game), 12);
    }
}
