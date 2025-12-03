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

    fn max_joltage(mut bank: &[u8], remaining: usize) -> usize {
        let mut joltage = 0;

        for remaining in (1..=remaining).rev() {
            assert!(remaining <= bank.len());
            let mut best_pos = bank.len() - remaining;
            for i in (0..bank.len() - remaining).rev() {
                if bank[best_pos] <= bank[i] {
                    best_pos = i;
                }
            }

            joltage = (joltage * 10) + bank[best_pos] as usize;
            bank = &bank[best_pos + 1..];
        }
        joltage
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .banks
            .iter()
            .map(|bank| Self::max_joltage(bank, 2))
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .banks
            .iter()
            .map(|bank| Self::max_joltage(bank, 12))
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
