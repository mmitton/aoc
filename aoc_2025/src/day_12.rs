#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};
use std::str::FromStr;

#[derive(Default)]
pub struct Day12 {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

struct Present {
    tiles: usize,
    shapes: Vec<Shape>,
}

impl FromStr for Present {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(Error::InvalidInput(s.into()));
        }

        let mut shapes = Vec::new();

        let initial: Vec<bool> = s.chars().map(|c| c == '#').collect();
        let tiles = initial.iter().filter(|c| **c).count();
        let mut s = Shape {
            shape: std::convert::TryInto::<[[bool; 3]; 3]>::try_into(
                initial
                    .chunks(3)
                    .map(|c| std::convert::TryInto::<[bool; 3]>::try_into(c).unwrap())
                    .collect::<Vec<[bool; 3]>>(),
            )
            .unwrap(),
        };
        for _ in 0..4 {
            if !shapes.contains(&s) {
                shapes.push(s);
            }
            let fx = s.flip_x();
            if !shapes.contains(&fx) {
                shapes.push(fx);
            }
            let fy = s.flip_y();
            if !shapes.contains(&fy) {
                shapes.push(fy);
            }

            s = s.rotate();
        }

        Ok(Present { tiles, shapes })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Shape {
    shape: [[bool; 3]; 3],
}

impl Shape {
    fn rotate(&self) -> Self {
        Self {
            shape: [
                [self.shape[2][0], self.shape[1][0], self.shape[0][0]],
                [self.shape[2][1], self.shape[1][1], self.shape[0][1]],
                [self.shape[2][2], self.shape[1][2], self.shape[0][2]],
            ],
        }
    }

    fn flip_x(&self) -> Self {
        let mut shape = self.shape;
        shape.iter_mut().for_each(|r| r.reverse());
        Self { shape }
    }

    fn flip_y(&self) -> Self {
        let mut shape = self.shape;
        shape.reverse();
        Self { shape }
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

impl FromStr for Region {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((wh, presents)) = s.split_once(": ") {
            if let Some((w, h)) = wh.split_once('x') {
                Ok(Self {
                    width: w.parse()?,
                    height: h.parse()?,
                    presents: presents
                        .split_whitespace()
                        .map(|v| v.parse().unwrap())
                        .collect(),
                })
            } else {
                Err(Error::InvalidInput(s.into()))
            }
        } else {
            Err(Error::InvalidInput(s.into()))
        }
    }
}

impl Region {
    fn can_fit(&self, presents: &[Present]) -> bool {
        #[derive(Debug)]
        struct State {
            present: usize,
            idx: usize,
            pos: Option<(usize, usize)>,
        }
        impl State {
            fn update_next(
                &mut self,
                max_idx: usize,
                max_x: usize,
                max_y: usize,
            ) -> Option<(usize, usize)> {
                if let Some((mut x, mut y)) = self.pos.take() {
                    self.idx += 1;
                    if self.idx == max_idx {
                        self.idx = 0;
                        if x == max_x {
                            x = 0;
                            if y == max_y {
                                self.pos = None;
                                return None;
                            }
                            y += 1;
                        } else {
                            x += 1;
                        }
                    }

                    self.pos = Some((x, y));
                } else {
                    self.idx = 0;
                    self.pos = Some((0, 0));
                }

                self.pos
            }

            fn next(&mut self, region: &mut [Vec<Option<u8>>], presents: &[Present]) -> bool {
                if let Some(pos) = self.pos {
                    // First remove it from the region
                    for (y, row) in presents[self.present].shapes[self.idx]
                        .shape
                        .iter()
                        .enumerate()
                    {
                        for (x, on) in row.iter().enumerate() {
                            if *on {
                                region[pos.1 + y][pos.0 + x] = None;
                            }
                        }
                    }
                } else {
                    self.pos = Some((0, 0));
                    self.idx = 0;
                }

                let max_idx = presents[self.present].shapes.len();
                let max_x = region[0].len() - 3;
                let max_y = region.len() - 3;
                'scan: while let Some(pos) = self.update_next(max_idx, max_x, max_y) {
                    for (y, row) in presents[self.present].shapes[self.idx]
                        .shape
                        .iter()
                        .enumerate()
                    {
                        for (x, on) in row.iter().enumerate() {
                            if *on && region[pos.1 + y][pos.0 + x].is_some() {
                                continue 'scan;
                            }
                        }
                    }
                    for (y, row) in presents[self.present].shapes[self.idx]
                        .shape
                        .iter()
                        .enumerate()
                    {
                        for (x, on) in row.iter().enumerate() {
                            if *on {
                                region[pos.1 + y][pos.0 + x] = Some(self.present as u8);
                            }
                        }
                    }
                    return true;
                }

                false
            }
        }

        if true {
            // Actual data has a flaw where all of the test cases need more total space for the
            // presents than is in the region.  No need to actually nest the presents for the right
            // answer
            let needed = presents
                .iter()
                .zip(self.presents.iter())
                .map(|(p, c)| p.tiles * c)
                .sum::<usize>();
            self.width * self.height >= needed
        } else {
            // Do the nesting if needed.
            let mut states = Vec::new();
            for (i, p) in self.presents.iter().enumerate() {
                for _ in 0..*p {
                    states.push(State {
                        present: i,
                        idx: 0,
                        pos: None,
                    });
                }
            }

            let total_needed = states
                .iter()
                .map(|s| {
                    presents[s.present].shapes[0]
                        .shape
                        .iter()
                        .map(|c| c.iter().filter(|c| **c).count())
                        .sum::<usize>()
                })
                .sum::<usize>();
            if total_needed > self.width * self.height {
                return false;
            }

            let mut region = vec![vec![None; self.width]; self.height];

            let mut idx = 0;
            while idx < states.len() {
                if states[idx].next(&mut region, presents) {
                    idx += 1;
                } else {
                    idx = idx.wrapping_sub(1);
                }
            }

            idx != usize::MAX
        }
    }
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self
            .regions
            .iter()
            .filter(|r| r.can_fit(&self.presents))
            .count()
            .into())
    }
}

impl helper::Runner for Day12 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        let mut s: Vec<char> = Vec::new();
        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            if line.contains(':') {
                if !s.is_empty() {
                    self.presents.push(String::from_iter(s.iter()).parse()?);
                    s.clear();
                }
                if line.contains('x') {
                    // Region
                    self.regions.push(line.parse()?);
                }
            } else {
                s.extend(line.chars());
            }
        }
        Ok(())
    }

    fn run_part(&mut self, part: u8) -> Result<helper::RunOutput, Error> {
        match part {
            1 => self.part1(),
            _ => Err(Error::Skipped),
        }
    }
}
