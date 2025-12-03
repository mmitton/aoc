#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day03 {
    banks: Vec<Vec<u8>>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn max_joltage(bank: &[u8], cur_joltage: usize, remaining: u32) -> Option<usize> {
        if remaining == 0 {
            return Some(cur_joltage);
        }
        if bank.len() < remaining as usize {
            return None;
        }
        let mut max_joltage: Option<usize> = None;
        let mut best = 0;
        for i in (0..bank.len()).rev() {
            if bank[i] < best {
                continue;
            }
            if let Some(joltage) = Self::max_joltage(
                &bank[i + 1..],
                bank[i] as usize + (cur_joltage * 10),
                remaining - 1,
            ) {
                let mj = max_joltage.unwrap_or_default();
                if joltage > mj {
                    best = bank[i];
                    max_joltage = Some(joltage);
                }
            }
        }

        max_joltage
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .banks
            .iter()
            .map(|bank| Self::max_joltage(bank, 0, 2).unwrap_or_default())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .banks
            .iter()
            .map(|bank| {
                let joltage = Self::max_joltage(bank, 0, 12).unwrap_or_default();
                println!("{joltage}");
                joltage
            })
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let bank = line.chars().map(|c| c as u8 - b'0').collect();
            self.banks.push(bank);
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
