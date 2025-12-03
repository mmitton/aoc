#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day02 {
    ranges: Vec<IDRange>,
}

#[derive(Clone)]
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
        self.slice.reverse();
    }

    fn inc(&mut self, len: usize) -> bool {
        for pos in (0..len).rev() {
            if self.slice[pos] != 9 {
                self.slice[pos] += 1;
                return true;
            } else {
                self.slice[pos] = 0;
            }
        }
        false
    }

    fn splat(&mut self, len: usize) {
        for i in len..self.slice.len() {
            self.slice[i] = self.slice[i % len];
        }
    }
}

impl From<usize> for ID {
    fn from(value: usize) -> Self {
        let mut id = Self::new();
        id.initialize(value);
        id
    }
}

impl From<&ID> for usize {
    fn from(value: &ID) -> Self {
        value.slice.iter().fold(0, |v, c| (v * 10) + *c as usize)
    }
}

#[derive(Debug)]
struct IDRange {
    low: usize,
    high: usize,
}

impl IDRange {
    fn new(mut low: usize, high: usize) -> Vec<Self> {
        let mut ranges = Vec::new();

        while low.ilog10() != high.ilog10() {
            let low_high = 10usize.pow(low.ilog10() + 1) - 1;
            ranges.push(Self {
                low,
                high: low_high,
            });
            low = low_high + 1;
        }
        ranges.push(Self { low, high });

        ranges
    }

    fn calc_bad_ids(&self, part1: bool) -> Vec<usize> {
        let total_len = self.low.ilog10() as usize + 1;
        let min_len = match part1 {
            false => 1,
            true if total_len.is_multiple_of(2) => total_len / 2,
            true => return Vec::new(),
        };

        let low_slice: ID = self.low.into();
        let mut cur_slice: ID = low_slice.clone();

        let mut bad_ids = HashSet::default();
        for len in min_len..=total_len / 2 {
            if total_len.is_multiple_of(len) {
                cur_slice.slice.copy_from_slice(&low_slice.slice);
                loop {
                    cur_slice.splat(len);

                    let bad_id: usize = (&cur_slice).into();
                    if bad_id > self.high {
                        break;
                    }
                    if bad_id >= self.low {
                        bad_ids.insert(bad_id);
                    }

                    if !cur_slice.inc(len) {
                        break;
                    }
                }
            }
        }

        bad_ids.iter().copied().collect()
    }
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .ranges
            .iter()
            .map(|range| range.calc_bad_ids(true).iter().sum::<usize>())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .ranges
            .iter()
            .map(|range| range.calc_bad_ids(false).iter().sum::<usize>())
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day02 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for range in lines[0].split(',') {
            if let Some((low, high)) = range.split_once('-') {
                let low = low.parse()?;
                let high = high.parse()?;
                self.ranges.extend(IDRange::new(low, high));
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
