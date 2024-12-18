#[allow(unused_imports)]
use helper::{print, println, Error, HashMap, HashSet, Lines, LinesOpt, Output, RunOutput, Runner};

#[derive(Default)]
pub struct Day03 {
    target: usize,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }

    fn map_address(address: usize) -> (isize, isize, isize) {
        if address == 1 {
            return (0, 0, 0);
        }

        let mut ring = 1;
        let mut ring_size = 9;
        let mut ring_start = 2;
        while ring_start + ring_size < address {
            ring += 1;
            ring_start += ring_size;
            ring_size += 8;
        }

        let mut x: isize = ring as isize;
        let mut y: isize = 0;
        let mut left_over = address - ring_start;

        if left_over > 0 {
            // Move Up!
            if left_over <= ring {
                y -= left_over as isize;
                left_over = 0;
            } else {
                y -= ring as isize;
                left_over -= ring;
            }
        }
        if left_over > 0 {
            // Move left!
            if left_over <= ring * 2 {
                x -= left_over as isize;
                left_over = 0;
            } else {
                x -= ring as isize * 2;
                left_over -= ring * 2;
            }
        }
        if left_over > 0 {
            // Move down!
            if left_over <= ring * 2 {
                y += left_over as isize;
                left_over = 0;
            } else {
                y += ring as isize * 2;
                left_over -= ring * 2;
            }
        }
        if left_over > 0 {
            // Move right!
            if left_over <= ring * 2 + 1 {
                x += left_over as isize;
                left_over = 0;
            } else {
                x += ring as isize * 2 + 1;
                left_over -= ring * 2 + 1;
            }
        }
        if left_over > 0 {
            // Move up!
            y -= left_over as isize;
        }

        let dx = if x < 0 { -x } else { x };
        let dy = if y < 0 { -y } else { y };

        (x, y, dx + dy)
    }

    fn stress(max: usize) -> usize {
        let mut map: HashMap<(isize, isize), usize> = HashMap::default();
        let mut mem: Vec<usize> = Vec::new();

        map.insert((0, 0), 0);
        mem.push(1);

        let mut addr = 2;
        loop {
            let (x, y, _) = Self::map_address(addr);
            let mut sum = 0;
            for x1 in -1..=1 {
                for y1 in -1..=1 {
                    if let Some(idx) = map.get(&(x + x1, y + y1)) {
                        sum += mem[*idx];
                    }
                }
            }
            map.insert((x, y), mem.len());
            mem.push(sum);

            if sum > max {
                return sum;
            }

            addr += 1;
        }
    }
}

impl Runner for Day03 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        assert_eq!(lines.len(), 1);
        self.target = lines[0].parse()?;
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

impl Day03 {
    fn part1(&mut self) -> Result<RunOutput, Error> {
        let (_, _, dist) = Self::map_address(self.target);
        Ok(dist.into())
    }

    fn part2(&mut self) -> Result<RunOutput, Error> {
        Ok(Self::stress(self.target).into())
    }
}
