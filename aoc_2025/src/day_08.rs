#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, Point3D, print, println};

#[derive(Default)]
pub struct Day08 {
    junctions: Vec<JunctionBox>,
}

struct JunctionBox {
    circuit: usize,
    coord: Point3D<isize>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut distances: Vec<(u64, u32, u32)> = Vec::new();
        for i in 0..self.junctions.len() {
            for j in i + 1..self.junctions.len() {
                let dist = self.junctions[i]
                    .coord
                    .euclidean_dist_squared(&self.junctions[j].coord);
                distances.push((dist as u64, i as u32, j as u32));
            }
        }
        distances.sort();
        let iters = if self.junctions.len() == 20 { 10 } else { 1000 };
        for (_, i, j) in distances.iter().take(iters).copied() {
            let from = self.junctions[j as usize].circuit;
            let to = self.junctions[i as usize].circuit;
            if from == to {
                continue;
            }
            self.junctions.iter_mut().for_each(|j| {
                if j.circuit == from {
                    j.circuit = to;
                }
            });
        }
        let mut sizes = vec![0; self.junctions.len()];
        for (id, c) in sizes.iter_mut().enumerate() {
            *c = self.junctions.iter().filter(|j| j.circuit == id).count();
        }

        sizes.sort();
        sizes.reverse();

        Ok(sizes.iter().take(3).product::<usize>().into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let mut distances: Vec<(u64, u32, u32)> = Vec::new();
        for i in 0..self.junctions.len() {
            for j in i + 1..self.junctions.len() {
                let dist = self.junctions[i]
                    .coord
                    .euclidean_dist_squared(&self.junctions[j].coord);
                distances.push((dist as u64, i as u32, j as u32));
            }
        }
        distances.sort();
        let mut x1 = 0;
        let mut x2 = 0;
        for (_, i, j) in distances.iter().copied() {
            let i = i as usize;
            let j = j as usize;
            let from = self.junctions[j].circuit;
            let to = self.junctions[i].circuit;
            if from == to {
                continue;
            }
            x1 = self.junctions[i].coord.x;
            x2 = self.junctions[j].coord.x;
            self.junctions.iter_mut().for_each(|j| {
                if j.circuit == from {
                    j.circuit = to;
                }
            });
        }
        Ok((x1 * x2).into())
    }
}

impl helper::Runner for Day08 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            let p: Vec<&str> = line.split(',').collect();
            if p.len() != 3 {
                return Err(Error::InvalidInput(line.into()));
            }
            let circuit = self.junctions.len();
            self.junctions.push(JunctionBox {
                circuit,
                coord: Point3D::new(p[0].parse()?, p[1].parse()?, p[2].parse()?),
            });
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
