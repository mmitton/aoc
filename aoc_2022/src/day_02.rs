#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day02 {
    rounds: Vec<(char, char)>,
    results: BTreeMap<(Play, Play), RoundResult>,
}

impl Day02 {
    pub fn new() -> Self {
        let mut results = BTreeMap::new();
        results.insert((Play::Rock, Play::Rock), RoundResult::Draw);
        results.insert((Play::Rock, Play::Paper), RoundResult::Win);
        results.insert((Play::Rock, Play::Scissors), RoundResult::Lose);

        results.insert((Play::Paper, Play::Rock), RoundResult::Lose);
        results.insert((Play::Paper, Play::Paper), RoundResult::Draw);
        results.insert((Play::Paper, Play::Scissors), RoundResult::Win);

        results.insert((Play::Scissors, Play::Rock), RoundResult::Win);
        results.insert((Play::Scissors, Play::Paper), RoundResult::Lose);
        results.insert((Play::Scissors, Play::Scissors), RoundResult::Draw);

        Self {
            rounds: Vec::new(),
            results,
        }
    }
}

impl Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let chars: Vec<char> = line.chars().collect();
            if chars.len() != 3 {
                return Err(Error::InvalidInput(line.into()));
            }
            if !matches!(chars[0], 'A' | 'B' | 'C') {
                return Err(Error::InvalidInput(line.into()));
            }
            if !matches!(chars[2], 'X' | 'Y' | 'Z') {
                return Err(Error::InvalidInput(line.into()));
            }
            self.rounds.push((chars[0], chars[2]));
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day02 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let score = self
            .rounds
            .clone()
            .iter()
            .map(|(p1, p2)| {
                let p1 = match p1 {
                    'A' => Play::Rock,
                    'B' => Play::Paper,
                    'C' => Play::Scissors,
                    _ => unreachable!(),
                };
                let p2 = match p2 {
                    'X' => Play::Rock,
                    'Y' => Play::Paper,
                    'Z' => Play::Scissors,
                    _ => unreachable!(),
                };
                let result = self.results.get(&(p1, p2)).unwrap();
                p2.points() + result.points()
            })
            .sum::<usize>();

        Ok(score.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let score = self
            .rounds
            .clone()
            .iter()
            .map(|(p1, p2)| {
                let p1 = match p1 {
                    'A' => Play::Rock,
                    'B' => Play::Paper,
                    'C' => Play::Scissors,
                    _ => unreachable!(),
                };
                let result = match p2 {
                    'X' => RoundResult::Lose,
                    'Y' => RoundResult::Draw,
                    'Z' => RoundResult::Win,
                    _ => unreachable!(),
                };

                let p2 = self
                    .results
                    .iter()
                    .find_map(|((_p1, p2), _result)| {
                        if p1 == *_p1 && result == *_result {
                            Some(*p2)
                        } else {
                            None
                        }
                    })
                    .unwrap();

                // let result = self.results.get(&(p1, p2)).unwrap();
                p2.points() + result.points()
            })
            .sum::<usize>();

        Ok(score.into())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RoundResult {
    Win,
    Lose,
    Draw,
}

impl Play {
    fn points(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl RoundResult {
    fn points(&self) -> usize {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}
