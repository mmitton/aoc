#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Debug)]
pub enum RunnerError {}

impl From<RunnerError> for Error {
    fn from(e: RunnerError) -> Self {
        Self::Runner(format!("{e:?}"))
    }
}

pub struct Day07 {}

impl Day07 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runner for Day07 {
    fn parse(&mut self, path: &str, _part1: bool) -> Result<(), Error> {
        let _lines = Lines::from_path(path, LinesOpt::RAW)?;
        Ok(())
    }

    fn part1(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Err(Error::Unsolved)
    }
}
