#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day04 {
    rolls: HashSet<(isize, isize)>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn can_remove(&self) -> Vec<(isize, isize)> {
        let mut to_remove = Vec::new();
        for (x, y) in self.rolls.iter() {
            let mut adjacent = 0;
            for (dx, dy) in &[
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                if self.rolls.contains(&(x + dx, y + dy)) {
                    adjacent += 1;
                }
            }
            if adjacent < 4 {
                to_remove.push((*x, *y));
            }
        }

        to_remove
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.can_remove().len().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut answer = 0;
        loop {
            let to_remove = self.can_remove();
            if to_remove.is_empty() {
                break;
            }
            answer += to_remove.len();
            for roll in to_remove.iter() {
                self.rolls.remove(roll);
            }
        }
        Ok(answer.into())
    }
}

impl helper::Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    self.rolls.insert((x as isize, y as isize));
                }
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
