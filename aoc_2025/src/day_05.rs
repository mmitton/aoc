use std::ops::RangeInclusive;

#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day05 {
    fresh: Vec<RangeInclusive<usize>>,
    avail: Vec<usize>,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .avail
            .iter()
            .filter(|a| self.fresh.iter().any(|f| f.contains(a)))
            .count()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        'scan: for i in 0..self.fresh.len() {
            for j in i + 1..self.fresh.len() {
                if *self.fresh[j].start() == usize::MAX {
                    continue;
                }
                if self.fresh[i].contains(self.fresh[j].start())
                    && self.fresh[i].contains(self.fresh[j].end())
                {
                    self.fresh[j] = usize::MAX..=usize::MAX;
                }
                if self.fresh[j].contains(self.fresh[i].start())
                    && self.fresh[j].contains(self.fresh[i].end())
                {
                    self.fresh[i] = usize::MAX..=usize::MAX;
                    continue 'scan;
                }
                if self.fresh[i].contains(self.fresh[j].start()) {
                    self.fresh[j] = *self.fresh[i].end() + 1..=*self.fresh[j].end();
                }
                if self.fresh[j].contains(self.fresh[i].start()) {
                    self.fresh[i] = *self.fresh[j].end() + 1..=*self.fresh[i].end();
                }
            }
        }
        Ok(self
            .fresh
            .iter()
            .map(|f| {
                if *f.start() == usize::MAX {
                    0
                } else {
                    f.end() - f.start() + 1
                }
            })
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day05 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (low, high) = line
                .split_once('-')
                .ok_or(Error::InvalidInput(line.into()))?;

            let low = low.parse()?;
            let high = high.parse()?;
            self.fresh.push(low..=high);
        }

        for line in lines {
            let avail = line.parse()?;
            self.avail.push(avail);
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
