#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day25 {}

impl Day25 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Day25 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_path(path, LinesOpt::RAW)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Skipped)
    }
}
