#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day02 {
    ranges: Vec<(usize, usize)>,
}

struct ID {
    slice: Vec<u8>,
}

impl ID {
    fn new() -> Self {
        Self {
            slice: Vec::with_capacity(64),
        }
    }

    fn initialize(&mut self, mut id: usize) {
        self.slice.clear();
        while id != 0 {
            self.slice.push((id % 10) as u8);
            id /= 10;
        }
    }

    fn inc(&mut self) {
        for n in self.slice.iter_mut() {
            *n += 1;
            if *n == 10 {
                *n = 0;
            } else {
                return;
            }
        }
        self.slice.push(1);
    }

    fn len(&self) -> usize {
        self.slice.len()
    }

    fn check_bad(&self, n: usize) -> bool {
        if self.slice.len().is_multiple_of(n) {
            let a = &self.slice[0..n];
            for b in self.slice.chunks(n).skip(1) {
                if a != b {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut bad = 0;
        let mut id_slice = ID::new();
        for (low, high) in self.ranges.iter() {
            id_slice.initialize(*low);
            for id in *low..=*high {
                if id_slice.len().is_multiple_of(2) && id_slice.check_bad(id_slice.len() / 2) {
                    bad += id;
                }
                id_slice.inc();
            }
        }
        Ok(bad.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut bad = 0;
        let mut id_slice = ID::new();
        for (low, high) in self.ranges.iter() {
            id_slice.initialize(*low);
            for id in *low..=*high {
                for n in 1..=id_slice.len() / 2 {
                    if id_slice.check_bad(n) {
                        bad += id;
                        break;
                    }
                }
                id_slice.inc();
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
