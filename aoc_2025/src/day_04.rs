#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day04 {
    rolls: Vec<Vec<bool>>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }

    fn can_remove(&self, mut to_remove: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        to_remove.clear();
        for (y, row) in self
            .rolls
            .iter()
            .enumerate()
            .skip(1)
            .take(self.rolls.len() - 2)
        {
            for (x, active) in row.iter().enumerate().skip(1).take(row.len() - 2) {
                if !active {
                    continue;
                }
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
                    let x1 = (x as i32 + dx) as usize;
                    let y1 = (y as i32 + dy) as usize;
                    if self.rolls[y1][x1] {
                        adjacent += 1;
                    }
                }
                if adjacent < 4 {
                    to_remove.push((x, y));
                }
            }
        }

        to_remove
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.can_remove(Vec::new()).len().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut answer = 0;
        let mut to_remove = Vec::new();
        loop {
            to_remove = self.can_remove(to_remove);
            if to_remove.is_empty() {
                break;
            }
            answer += to_remove.len();
            for (x, y) in to_remove.iter().copied() {
                self.rolls[y][x] = false;
            }
        }
        Ok(answer.into())
    }
}

impl helper::Runner for Day04 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.rolls.push(line.chars().map(|c| c == '@').collect());
        }
        for rolls in self.rolls.iter_mut() {
            rolls.insert(0, false);
            rolls.push(false);
        }
        let empty = vec![false; self.rolls[0].len()];
        self.rolls.insert(0, empty.clone());
        self.rolls.push(empty);
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
