#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}