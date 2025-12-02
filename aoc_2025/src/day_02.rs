#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day02 {
    ranges: Vec<(usize, usize)>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut bad = 0;
        for (low, high) in self.ranges.iter() {
            'scan: for id in *low..=*high {
                let id_str: Vec<char> = id.to_string().chars().collect();
                if id_str.len().is_multiple_of(2) {
                    let a = &id_str[0..id_str.len() / 2];
                    let b = &id_str[id_str.len() / 2..];
                    if a == b {
                        bad += id;
                        continue 'scan;
                    }
                }
            }
        }
        Ok(bad.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut bad = 0;
        for (low, high) in self.ranges.iter() {
            for id in *low..=*high {
                let id_str: Vec<char> = id.to_string().chars().collect();
                'scan: for n in 1..=id_str.len() / 2 {
                    if id_str.len().is_multiple_of(n) {
                        let a = &id_str[0..n];
                        for m in (n..id_str.len()).step_by(n) {
                            let b = &id_str[m..m + n];
                            if a != b {
                                continue 'scan;
                            }
                        }
                        bad += id;
                        break 'scan;
                    }
                }
            }
        }
        Ok(bad.into())
    }
}

impl helper::Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for range in lines[0].split(',') {
            if let Some((low, high)) = range.split_once('-') {
                let low = low.parse()?;
                let high = high.parse()?;
                self.ranges.push((low, high));
            } else {
                return Err(Error::InvalidInput(range.into()));
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
