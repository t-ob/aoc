use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

#[derive(Debug)]
pub struct Round(Move, Move);

impl FromStr for Round {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let opponent_move = match chars.nth(0) {
            Some('A') => Move::Rock,
            Some('B') => Move::Paper,
            Some('C') => Move::Scissors,
            _ => return Err(String::from("Unable to parse opponent move")),
        };
        let player_move = match chars.nth(1) {
            Some('X') => Move::Rock,
            Some('Y') => Move::Paper,
            Some('Z') => Move::Scissors,
            _ => return Err(String::from("Unable to parse player move")),
        };

        Ok(Self(opponent_move, player_move))
    }
}

impl Round {
    pub fn score(&self) -> u32 {
        let outcome = match (&self.0, &self.1) {
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock) => 6,
            (x, y) if x == y => 3,
            _ => 0,
        };

        let shape = match &self.1 {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        };

        shape + outcome
    }
}

#[derive(Debug)]
pub struct RoundOutcome(Move, Outcome);

impl FromStr for RoundOutcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let opponent_move = match chars.nth(0) {
            Some('A') => Move::Rock,
            Some('B') => Move::Paper,
            Some('C') => Move::Scissors,
            _ => return Err(String::from("Unable to parse opponent move")),
        };
        let player_outcome = match chars.nth(1) {
            Some('X') => Outcome::Lose,
            Some('Y') => Outcome::Draw,
            Some('Z') => Outcome::Win,
            _ => return Err(String::from("Unable to parse player outcome")),
        };

        Ok(Self(opponent_move, player_outcome))
    }
}

impl RoundOutcome {
    pub fn to_round(&self) -> Round {
        let player_move = match (&self.0, &self.1) {
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (x, _) => *x,
        };

        Round(self.0, player_move)
    }
}

#[derive(Debug)]
pub struct PuzzleInput(pub Round, pub RoundOutcome);

impl FromStr for PuzzleInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?, s.parse()?))
    }
}
