use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let input = input.parse::<Input>().unwrap();
    let interpretation = PartOneInterpretation;
    let points = input.points(&interpretation);
    println!("{points}");

    let interpretation = PartTwoInterpretation;
    let points = input.points(&interpretation);
    println!("{points}");
}

#[derive(Clone, Debug)]
pub struct Input {
    rounds: Vec<Round>,
}

impl Input {
    pub fn points(&self, interpretation: &impl Interpretation) -> usize {
        let mut sum = 0;
        for round in &self.rounds {
            sum += round.points(interpretation);
        }
        sum
    }
}

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rounds = Vec::new();
        for line in s.lines().filter(|line| !line.is_empty()) {
            rounds.push(line.parse()?);
        }
        Ok(Input { rounds })
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Round {
    opponent: Opponent,
    recommendation: Recommendation,
}

impl Round {
    pub fn points(&self, interpretation: &impl Interpretation) -> usize {
        let opponent = interpretation.rochambeau_for_opponent(self);
        let recommendation = interpretation.rochambeau_for_recommendation(self);
        let outcome = recommendation.play(&opponent);

        recommendation.points() + outcome.points()
    }
}

impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (opponent, recommendation) = s.split_once(' ').ok_or("No space found")?;
        let opponent = opponent.parse()?;
        let recommendation = recommendation.parse()?;
        Ok(Round {
            opponent,
            recommendation,
        })
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Opponent {
    A,
    B,
    C,
}

impl FromStr for Opponent {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            _ => Err("Invalid input"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Recommendation {
    X,
    Y,
    Z,
}

impl FromStr for Recommendation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::X),
            "Y" => Ok(Self::Y),
            "Z" => Ok(Self::Z),
            _ => Err("Invalid input"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Rochambeau {
    Rock,
    Paper,
    Scissors,
}

impl Rochambeau {
    pub fn play(&self, other: &Rochambeau) -> RochambeauOutcome {
        match (self, other) {
            (Rochambeau::Rock, Rochambeau::Rock) => RochambeauOutcome::Tie,
            (Rochambeau::Rock, Rochambeau::Paper) => RochambeauOutcome::Loss,
            (Rochambeau::Rock, Rochambeau::Scissors) => RochambeauOutcome::Win,
            (Rochambeau::Paper, Rochambeau::Rock) => RochambeauOutcome::Win,
            (Rochambeau::Paper, Rochambeau::Paper) => RochambeauOutcome::Tie,
            (Rochambeau::Paper, Rochambeau::Scissors) => RochambeauOutcome::Loss,
            (Rochambeau::Scissors, Rochambeau::Rock) => RochambeauOutcome::Loss,
            (Rochambeau::Scissors, Rochambeau::Paper) => RochambeauOutcome::Win,
            (Rochambeau::Scissors, Rochambeau::Scissors) => RochambeauOutcome::Tie,
        }
    }

    pub fn points(&self) -> usize {
        match self {
            Rochambeau::Rock => 1,
            Rochambeau::Paper => 2,
            Rochambeau::Scissors => 3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum RochambeauOutcome {
    Win,
    Tie,
    Loss,
}

impl RochambeauOutcome {
    fn points(&self) -> usize {
        match self {
            RochambeauOutcome::Win => 6,
            RochambeauOutcome::Tie => 3,
            RochambeauOutcome::Loss => 0,
        }
    }
}

pub trait Interpretation {
    fn rochambeau_for_opponent(&self, round: &Round) -> Rochambeau;
    fn rochambeau_for_recommendation(&self, round: &Round) -> Rochambeau;
}

struct PartOneInterpretation;

impl Interpretation for PartOneInterpretation {
    fn rochambeau_for_opponent(&self, round: &Round) -> Rochambeau {
        match round.opponent {
            Opponent::A => Rochambeau::Rock,
            Opponent::B => Rochambeau::Paper,
            Opponent::C => Rochambeau::Scissors,
        }
    }

    fn rochambeau_for_recommendation(&self, round: &Round) -> Rochambeau {
        match round.recommendation {
            Recommendation::X => Rochambeau::Rock,
            Recommendation::Y => Rochambeau::Paper,
            Recommendation::Z => Rochambeau::Scissors,
        }
    }
}

struct PartTwoInterpretation;

impl Interpretation for PartTwoInterpretation {
    fn rochambeau_for_opponent(&self, round: &Round) -> Rochambeau {
        match round.opponent {
            Opponent::A => Rochambeau::Rock,
            Opponent::B => Rochambeau::Paper,
            Opponent::C => Rochambeau::Scissors,
        }
    }

    fn rochambeau_for_recommendation(&self, round: &Round) -> Rochambeau {
        let expected_outcome = match round.recommendation {
            Recommendation::X => RochambeauOutcome::Loss,
            Recommendation::Y => RochambeauOutcome::Tie,
            Recommendation::Z => RochambeauOutcome::Win,
        };
        let opponent = self.rochambeau_for_opponent(round);

        match (opponent, expected_outcome) {
            (Rochambeau::Rock, RochambeauOutcome::Win) => Rochambeau::Paper,
            (Rochambeau::Rock, RochambeauOutcome::Tie) => Rochambeau::Rock,
            (Rochambeau::Rock, RochambeauOutcome::Loss) => Rochambeau::Scissors,
            (Rochambeau::Paper, RochambeauOutcome::Win) => Rochambeau::Scissors,
            (Rochambeau::Paper, RochambeauOutcome::Tie) => Rochambeau::Paper,
            (Rochambeau::Paper, RochambeauOutcome::Loss) => Rochambeau::Rock,
            (Rochambeau::Scissors, RochambeauOutcome::Win) => Rochambeau::Rock,
            (Rochambeau::Scissors, RochambeauOutcome::Tie) => Rochambeau::Scissors,
            (Rochambeau::Scissors, RochambeauOutcome::Loss) => Rochambeau::Paper,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("example.txt");
        let input = input.parse::<Input>().unwrap();
        let interpretation = PartOneInterpretation;
        let points = input.points(&interpretation);
        assert_eq!(points, 15);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("example.txt");
        let input = input.parse::<Input>().unwrap();
        let interpretation = PartTwoInterpretation;
        let points = input.points(&interpretation);
        assert_eq!(points, 12);
    }
}
