#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};
use std::{collections::BTreeMap, str::FromStr};

#[derive(Default)]
pub struct Day10 {
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

fn indicator_presses(
    target: &[bool],
    joltage: &[u16; 16],
    buttons: &[Vec<usize>],
) -> Vec<(u32, [u16; 16])> {
    let max = 2usize.pow(buttons.len() as u32);
    let mut lights = vec![false; target.len()];
    let mut results = Vec::new();

    'presses: for presses in 0..max {
        lights.iter_mut().for_each(|l| *l = false);
        let mut remaining_joltage = *joltage;
        for (i, toggle) in buttons.iter().enumerate() {
            if (presses >> i) & 1 == 1 {
                for j in toggle.iter().copied() {
                    lights[j] = !lights[j];
                    if remaining_joltage[j] == 0 {
                        continue 'presses;
                    } else {
                        remaining_joltage[j] -= 1;
                    }
                }
            }
        }
        if lights == target {
            results.push((presses.count_ones(), remaining_joltage));
        }
    }

    results
}

fn joltage_presses(
    cache: &mut BTreeMap<[u16; 16], Option<usize>>,
    joltage: [u16; 16],
    buttons: &[Vec<usize>],
) -> Option<usize> {
    if !joltage.iter().any(|j| *j != 0) {
        return Some(0);
    }

    if let Some(presses) = cache.get(&joltage) {
        return *presses;
    }

    let target: Vec<bool> = joltage.iter().map(|j| j % 2 == 1).collect();
    let presses = if target.contains(&true) {
        let mut possible = indicator_presses(&target, &joltage, buttons);

        let mut min_presses = None;
        for (presses, remaining_joltage) in possible.drain(..) {
            if let Some(remaining_presses) = joltage_presses(cache, remaining_joltage, buttons) {
                min_presses = Some(
                    min_presses
                        .unwrap_or(usize::MAX)
                        .min(presses as usize + remaining_presses),
                );
            }
        }

        min_presses
    } else {
        let mut half_joltage = joltage;
        half_joltage.iter_mut().for_each(|j| *j /= 2);
        joltage_presses(cache, half_joltage, buttons).map(|half_presses| half_presses * 2)
    };

    cache.insert(joltage, presses);
    presses
}

impl Machine {
    fn indicator_presses(&self) -> usize {
        let max_joltage = [u16::MAX; 16];
        let all = indicator_presses(&self.target, &max_joltage, &self.buttons);
        all.iter()
            .map(|(presses, _)| *presses as usize)
            .min()
            .unwrap()
    }

    fn joltage_presses(&self) -> usize {
        let mut joltage = [0; 16];
        joltage
            .iter_mut()
            .zip(self.joltage.iter())
            .for_each(|(a, b)| *a = *b);
        joltage_presses(&mut BTreeMap::default(), joltage, &self.buttons).unwrap()
    }
}

impl FromStr for Machine {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split_whitespace().collect();

        let target_str = parts.remove(0);
        let target = target_str
            .chars()
            .take(target_str.len() - 1)
            .skip(1)
            .map(|c| c == '#')
            .collect();

        let joltage_str = parts
            .pop()
            .unwrap()
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap();
        let joltage = joltage_str.split(',').map(|v| v.parse().unwrap()).collect();

        let mut buttons = Vec::new();
        for button in parts.iter() {
            buttons.push(
                button
                    .strip_prefix('(')
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap()
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect(),
            );
        }

        Ok(Machine {
            target,
            buttons,
            joltage,
        })
    }
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .machines
            .iter()
            .map(|m| m.indicator_presses())
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .machines
            .iter()
            .map(|m| m.joltage_presses())
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day10 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            self.machines.push(line.parse()?);
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
