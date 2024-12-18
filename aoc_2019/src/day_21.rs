use crate::intcode::{IntCode, State};
#[allow(unused_imports)]
use helper::{print, println, Error, Lines, LinesOpt, Output, RunOutput, Runner};

pub struct Day21 {
    intcode: IntCode<isize>,
}

impl Day21 {
    pub fn new() -> Self {
        Self {
            intcode: IntCode::default(),
        }
    }

    pub fn run(&mut self, commands: &[&str]) -> isize {
        for cmd in commands.iter() {
            self.intcode.input.extend(cmd.chars().map(|c| c as isize));
            self.intcode.input.push_back(b'\n' as isize);
        }

        let mut last_damage = 0;
        loop {
            match self.intcode.run() {
                State::HasOutput(v) => last_damage = v,
                State::Stopped => break,
                x => unreachable!("Unexpected state: {x:?}"),
            }
        }

        last_damage
    }
}

impl Runner for Day21 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        self.intcode.load(Lines::from_bufread(file, LinesOpt::RAW)?)
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day21 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .run(&[
                "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK",
            ])
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(self
            .run(&[
                "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T",
                "NOT T T", "OR H T", "AND T J", "RUN",
            ])
            .into())
    }
}
