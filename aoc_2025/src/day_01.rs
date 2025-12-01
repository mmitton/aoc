#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day01 {
    turns: Vec<Turn>,
}

#[derive(Debug)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl Day01 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .turns
            .iter()
            .fold((50isize, 0), |(at, zeros), turn| {
                let new_at = match turn {
                    Turn::Left(c) => (at - c).rem_euclid(100),
                    Turn::Right(c) => (at + c).rem_euclid(100),
                };
                if new_at == 0 {
                    (new_at, zeros + 1)
                } else {
                    (new_at, zeros)
                }
            })
            .1
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .turns
            .iter()
            .fold((50isize, 0), |(at, zeros), turn| {
                let new_at = match turn {
                    Turn::Left(c) => at - *c,
                    Turn::Right(c) => at + *c,
                };
                let new_zeros = zeros + new_at.div_euclid(100).abs();
                (new_at.rem_euclid(100), new_zeros)
            })
            .1
            .into())
    }
}

impl helper::Runner for Day01 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some(rem) = line.strip_prefix("L") {
                self.turns.push(Turn::Left(rem.parse()?))
            } else if let Some(rem) = line.strip_prefix("R") {
                self.turns.push(Turn::Right(rem.parse()?))
            } else {
                return Err(Error::InvalidInput(line.into()));
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}
