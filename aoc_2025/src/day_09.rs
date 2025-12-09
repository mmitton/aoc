#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, Point2D, print, println};

#[derive(Default)]
pub struct Day09 {
    red_tiles: Vec<Point2D<isize>>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        let mut max_area = 0;
        for (i, a) in self.red_tiles.iter().enumerate() {
            for b in self.red_tiles.iter().skip(i + 1) {
                let ul: Point2D<isize> = Point2D::new(a.x.min(b.x), a.y.min(b.y));
                let lr: Point2D<isize> = Point2D::new(a.x.max(b.x), a.y.max(b.y));
                let area = (lr.x - ul.x + 1) * (lr.y - ul.y + 1);
                max_area = max_area.max(area);
            }
        }
        Ok(max_area.into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        self.red_tiles.push(self.red_tiles[0]);
        let mut max_area = 0;
        for (i, a) in self.red_tiles.iter().enumerate() {
            'scan: for b in self.red_tiles.iter().skip(i + 1) {
                let area = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);
                if area <= max_area {
                    continue;
                }
                let ul: Point2D<isize> = Point2D::new(a.x.min(b.x), a.y.min(b.y));
                let lr: Point2D<isize> = Point2D::new(a.x.max(b.x), a.y.max(b.y));
                for i in 0..self.red_tiles.len() - 1 {
                    let c: Point2D<isize> = Point2D::new(
                        self.red_tiles[i].x.min(self.red_tiles[i + 1].x),
                        self.red_tiles[i].y.min(self.red_tiles[i + 1].y),
                    );
                    let d: Point2D<isize> = Point2D::new(
                        self.red_tiles[i].x.max(self.red_tiles[i + 1].x),
                        self.red_tiles[i].y.max(self.red_tiles[i + 1].y),
                    );

                    // Check for intersection for 4 lines
                    fn crosses(
                        a1: isize,
                        a2: isize,
                        b1: isize,
                        b2: isize,
                        a3: isize,
                        b3: isize,
                        b4: isize,
                    ) -> bool {
                        (a1 + 1..=a2 - 1).contains(&a3)
                            && ((b3 <= b1 && b4 > b2)
                                || (b3 < b1 && b4 >= b2)
                                || (b1 + 1..=b2 - 1).contains(&b3)
                                || (b1 + 1..=b2 - 1).contains(&b4)
                                || (b3 == b1 && b4 == b2))
                    }
                    if c.x == d.x && crosses(ul.x, lr.x, ul.y, lr.y, c.x, c.y, d.y) {
                        continue 'scan;
                    }
                    if c.y == d.y && crosses(ul.y, lr.y, ul.x, lr.x, c.y, c.x, d.x) {
                        continue 'scan;
                    }
                }
                max_area = area;
            }
        }
        Ok(max_area.into())
    }
}

impl helper::Runner for Day09 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            match line.split_once(',') {
                Some((x, y)) => self.red_tiles.push(Point2D::new(x.parse()?, y.parse()?)),
                None => return Err(Error::InvalidInput(line.into())),
            }
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
