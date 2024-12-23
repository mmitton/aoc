#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
struct Generator<const MULT: usize, const MASK: usize> {
    num: usize,
}

impl<const MULT: usize, const MASK: usize> Generator<MULT, MASK> {
    fn new(num: usize) -> Self {
        Self { num }
    }
}

impl<const MULT: usize, const MASK: usize> Iterator for Generator<MULT, MASK> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let prod = self.num * MULT;
            let num = (prod & 0x7fffffff) + (prod >> 31);
            if num >> 31 != 0 {
                self.num = num - 0x7fffffff;
            } else {
                self.num = num;
            }
            if self.num & MASK == 0 {
                break;
            }
        }
        Some(self.num)
    }
}

#[derive(Default)]
pub struct Day15 {
    gen_a: usize,
    gen_b: usize,
}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day15 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 2);
        self.gen_a = lines[0][24..].parse()?;
        self.gen_b = lines[1][24..].parse()?;

        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<RunOutput, Error> {
        match part {
            1 => self.part1(),
            2 => self.part2(),
            _ => Err(Error::Skipped),
        }
    }
}

impl Day15 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let gen_a = Generator::<16807, 0>::new(self.gen_a);
        let gen_b = Generator::<48271, 0>::new(self.gen_b);
        Ok(gen_a
            .zip(gen_b)
            .take(40_000_000)
            .fold(0, |count, (a, b)| {
                if a & 0xFFFF == b & 0xFFFF {
                    count + 1
                } else {
                    count
                }
            })
            .into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        let gen_a = Generator::<16807, 3>::new(self.gen_a);
        let gen_b = Generator::<48271, 7>::new(self.gen_b);
        Ok(gen_a
            .zip(gen_b)
            .take(5_000_000)
            .fold(0, |count, (a, b)| {
                if a & 0xFFFF == b & 0xFFFF {
                    count + 1
                } else {
                    count
                }
            })
            .into())
    }
}
