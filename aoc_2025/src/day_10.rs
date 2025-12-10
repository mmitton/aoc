#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};
use std::{collections::VecDeque, str::FromStr};

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

impl Machine {
    fn indicator_presses(&self) -> usize {
        let mut work: VecDeque<(usize, Vec<bool>, usize)> = VecDeque::new();
        work.push_front((0, vec![false; self.target.len()], usize::MAX));

        while let Some((presses, lights, last_pressed)) = work.pop_front() {
            for (button_idx, button) in self.buttons.iter().enumerate() {
                if button_idx == last_pressed {
                    continue;
                }
                let mut new_lights = lights.clone();
                for i in button.iter().copied() {
                    new_lights[i] = !new_lights[i];
                }
                if new_lights == self.target {
                    return presses + 1;
                }
                work.push_back((presses + 1, new_lights, button_idx));
            }
        }

        unreachable!()
    }

    fn joltage_presses(&self) -> usize {
        fn inc(
            presses_total: &mut u16,
            target: u16,
            presses: &mut [u16; 16],
            avail_buttons: &[usize],
            buttons: &[Vec<usize>],
            next_joltage: &mut [u16; 16],
        ) -> bool {
            'inc_loop: loop {
                for i in avail_buttons.iter().copied() {
                    if i == avail_buttons[0] && presses[i] != 0 {
                        *presses_total -= presses[i];
                        for b in buttons[i].iter().copied() {
                            next_joltage[b] += presses[i];
                        }
                        presses[i] = 0;
                        continue;
                    }

                    let max_presses = buttons[i]
                        .iter()
                        .copied()
                        .map(|i| next_joltage[i])
                        .min()
                        .unwrap()
                        + presses[i];
                    // println!(
                    //     "   {i} {} {max_presses}  {target_joltage:?} {next_joltage:?}",
                    //     presses[i]
                    // );
                    if presses[i] == max_presses {
                        *presses_total -= presses[i];
                        for b in buttons[i].iter().copied() {
                            next_joltage[b] += presses[i];
                        }
                        presses[i] = 0;
                        continue;
                    }
                    if i == avail_buttons[0] {
                        presses[i] += max_presses;
                        for b in buttons[i].iter().copied() {
                            next_joltage[b] -= max_presses;
                        }
                        *presses_total += max_presses;
                    } else {
                        presses[i] += 1;
                        for b in buttons[i].iter().copied() {
                            next_joltage[b] -= 1;
                        }
                        *presses_total += 1;
                    }
                    if *presses_total == target {
                        return true;
                    }
                    continue 'inc_loop;
                }
                return false;
            }
        }

        let mut joltage = [0; 16];
        for (i, v) in self.joltage.iter().copied().enumerate() {
            joltage[i] = v;
        }

        // Solve for each joltage one by one
        // let mut cur = vec![(0, joltage)];
        // let mut next = Vec::new();
        let mut cur = Vec::with_capacity(10000000);
        let mut next = Vec::with_capacity(10000000);
        cur.push((0, joltage));
        let mut handled = vec![false; self.joltage.len()];
        let mut buttons_master: Vec<(usize, usize, Vec<usize>)> = Vec::new();
        for _ in 0..self.joltage.len() {
            // find next position to handle
            buttons_master.clear();
            for i in 0..self.joltage.len() {
                if handled[i] {
                    continue;
                }

                let mut buttons = Vec::new();
                for (button_idx, button) in self.buttons.iter().enumerate() {
                    let mut found = false;
                    for b in button.iter().copied() {
                        if handled[b] {
                            found = false;
                            break;
                        }
                        if b == i {
                            found = true;
                        }
                    }
                    if found {
                        buttons.push(button_idx);
                    }
                }
                if buttons.is_empty() {
                    continue;
                }
                buttons_master.push((buttons.len(), i, buttons));
            }
            if buttons_master.is_empty() {
                break;
            }
            buttons_master.sort();
            let (_, pos, avail_buttons) = buttons_master.remove(0);

            handled[pos] = true;

            for (cur_presses, cur_joltage) in cur.drain(..) {
                if cur_joltage[pos] == 0 {
                    next.push((cur_presses, cur_joltage));
                    continue;
                }

                // println!("{cur_presses} {cur_joltage:?}");

                let mut presses = [0; 16];
                let next_pressed = cur_presses + cur_joltage[pos];
                let mut next_joltage = cur_joltage;
                let mut presses_total = 0;
                while inc(
                    &mut presses_total,
                    cur_joltage[pos],
                    &mut presses,
                    &avail_buttons,
                    &self.buttons,
                    &mut next_joltage,
                ) {
                    next.push((next_pressed, next_joltage));
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        cur.iter()
            .filter_map(|(presses, joltage)| {
                if joltage.iter().any(|c| *c != 0) {
                    None
                } else {
                    Some(*presses)
                }
            })
            .min()
            .unwrap() as usize
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
