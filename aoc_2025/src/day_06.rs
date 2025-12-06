#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};

#[derive(Default)]
pub struct Day06 {
    problems: Vec<(Vec<usize>, Op)>,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .problems
            .iter()
            .map(|(nums, op)| match op {
                Op::Add => nums.iter().sum::<usize>(),
                Op::Mul => nums.iter().product::<usize>(),
            })
            .sum::<usize>()
            .into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .problems
            .iter()
            .map(|(nums, op)| match op {
                Op::Add => nums.iter().sum::<usize>(),
                Op::Mul => nums.iter().product::<usize>(),
            })
            .sum::<usize>()
            .into())
    }
}

impl helper::Runner for Day06 {
    fn parse(&mut self, file: &[u8], part: u8) -> Result<(), Error> {
        let mut lines: Vec<String> = Lines::from_bufread(file, LinesOpt::RAW)?
            .iter()
            .map(|l| l.to_string())
            .collect();
        match part {
            1 => {
                let mut nums: Vec<Vec<usize>> = Vec::new();
                let mut ops = Vec::new();

                for line in lines.iter() {
                    let split: Vec<&str> = line.split_whitespace().collect();
                    if split[0] == "*" || split[0] == "+" {
                        ops.extend(
                            split
                                .iter()
                                .map(|c| if c == &"*" { Op::Mul } else { Op::Add }),
                        );
                    } else {
                        nums.push(split.iter().map(|n| n.parse::<usize>().unwrap()).collect());
                    }
                }

                for i in 0..nums[0].len() {
                    let mut n = Vec::new();
                    for j in nums.iter() {
                        n.push(j[i]);
                    }
                    self.problems.push((n, ops[i]));
                }
            }
            2 => {
                let ops: Vec<char> = lines.pop().unwrap().chars().collect();
                let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
                let mut ops_pos: Vec<usize> = ops
                    .iter()
                    .enumerate()
                    .filter_map(|(pos, c)| if *c == ' ' { None } else { Some(pos) })
                    .collect();
                ops_pos.push(ops.len());
                let mut ops_spans = Vec::new();
                for i in 0..ops_pos.len() - 1 {
                    ops_spans.push(ops_pos[i]..ops_pos[i + 1]);
                }

                for span in ops_spans.iter().rev() {
                    let mut nums = Vec::new();
                    for i in span.start..span.end {
                        let mut num = 0;
                        for line in lines.iter() {
                            match line[i] {
                                ' ' => {}
                                c @ '0'..='9' => num = (num * 10) + (c as u8 - b'0') as usize,
                                _ => unreachable!(),
                            }
                        }
                        if num > 0 {
                            nums.push(num);
                        }
                    }
                    let op = match ops[span.start] {
                        '*' => Op::Mul,
                        '+' => Op::Add,
                        _ => unreachable!(),
                    };
                    self.problems.push((nums, op));
                }
            }
            _ => unreachable!(),
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
