#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day07 {
    map: Vec<Vec<isize>>,
    start: isize,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut splits = 0;
        let mut cur = HashSet::default();
        let mut next = HashSet::default();
        cur.insert(self.start);
        for row in self.map.iter() {
            next.clear();
            for c in cur.iter() {
                if row.contains(c) {
                    next.insert(c - 1);
                    next.insert(c + 1);
                    splits += 1;
                } else {
                    next.insert(*c);
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }
        Ok(splits.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut cur = HashMap::default();
        let mut next = HashMap::default();
        cur.insert(self.start, 1);
        for row in self.map.iter() {
            next.clear();
            for (c, paths) in cur.iter() {
                if row.contains(c) {
                    *next.entry(c - 1).or_insert(0) += paths;
                    *next.entry(c + 1).or_insert(0) += paths;
                } else {
                    *next.entry(*c).or_insert(0) += paths;
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        Ok(cur.values().sum::<usize>().into())
    }
}

impl helper::Runner for Day07 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut lines = lines.iter();
        self.start = lines
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap() as isize;
        for line in lines {
            self.map.push(
                line.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '^' { Some(i as isize) } else { None })
                    .collect(),
            );
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
