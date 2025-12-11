#[allow(unused_imports)]
use helper::{Error, HashMap, HashSet, Lines, LinesOpt, print, println};
use std::collections::VecDeque;

#[derive(Default)]
pub struct Day11 {
    names: HashMap<String, usize>,
    devices: HashMap<usize, Device>,
    you: usize,
    svr: usize,
    out: usize,
    fft: usize,
    dac: usize,
}

#[derive(Debug)]
struct Device {
    connections: Vec<usize>,
    paths_from: HashMap<usize, usize>,
}

impl Device {
    fn paths_here(&self) -> usize {
        self.paths_from.values().sum()
    }
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }

    fn path_count(&mut self, from: usize, to: usize) -> usize {
        self.devices.values_mut().for_each(|d| d.paths_from.clear());

        let mut work = VecDeque::new();
        work.push_front(from);

        while let Some(at) = work.pop_front() {
            let device = self.devices.get(&at).unwrap();
            let connections = device.connections.clone();
            let paths_here = if at == from { 1 } else { device.paths_here() };

            for connection in connections.iter() {
                *self
                    .devices
                    .get_mut(connection)
                    .unwrap()
                    .paths_from
                    .entry(at)
                    .or_default() = paths_here;
                if *connection != to && !work.contains(connection) {
                    work.push_back(*connection);
                }
            }
        }

        self.devices.get(&to).unwrap().paths_here()
    }

    fn part1(&mut self) -> Result<helper::RunOutput, Error> {
        Ok(self.path_count(self.you, self.out).into())
    }

    fn part2(&mut self) -> Result<helper::RunOutput, Error> {
        let fft_to_dac = self.path_count(self.fft, self.dac);
        let dac_to_fft = self.path_count(self.dac, self.fft);

        let paths = match (fft_to_dac, dac_to_fft) {
            (0, dac_to_fft) => {
                let svr_to_dac = self.path_count(self.svr, self.dac);
                let fft_to_out = self.path_count(self.fft, self.out);
                svr_to_dac * dac_to_fft * fft_to_out
            }
            (fft_to_dac, 0) => {
                let svr_to_fft = self.path_count(self.svr, self.fft);
                let dac_to_out = self.path_count(self.dac, self.out);
                svr_to_fft * fft_to_dac * dac_to_out
            }
            _ => unreachable!(),
        };

        Ok(paths.into())
    }

    fn get_name_id(&mut self, name: &str) -> usize {
        let name_string = name.to_string();
        let next_id = self.names.len();
        let id = *self.names.entry(name_string).or_insert(next_id);
        match name {
            "you" => self.you = id,
            "svr" => self.svr = id,
            "out" => self.out = id,
            "fft" => self.fft = id,
            "dac" => self.dac = id,
            _ => {}
        }
        id
    }
}

impl helper::Runner for Day11 {
    fn parse(&mut self, file: &[u8], _part: u8) -> Result<(), Error> {
        let lines = Lines::from_bufread(file, LinesOpt::RAW)?;
        for line in lines.iter() {
            if let Some((device_name, connections)) = line.split_once(": ") {
                let device_id = self.get_name_id(device_name);
                let mut connections_vec = Vec::new();
                for connection in connections.split_whitespace() {
                    connections_vec.push(self.get_name_id(connection));
                }
                self.devices.insert(
                    device_id,
                    Device {
                        connections: connections_vec,
                        paths_from: HashMap::default(),
                    },
                );
            } else {
                return Err(Error::InvalidInput(line.into()));
            }
        }
        self.devices.insert(
            self.out,
            Device {
                connections: Vec::new(),
                paths_from: HashMap::default(),
            },
        );
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
