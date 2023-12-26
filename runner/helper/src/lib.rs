mod algorithms;
mod error;
mod output;
mod parser;
mod run_output;

pub use algorithms::*;
pub use error::Error;
pub use output::Output;
pub use parser::{find_day_part_files, search_up, Lines, LinesOpt, SearchType};
pub use run_output::RunOutput;

pub trait Runner {
    fn parse(&mut self, path: &str, part1: bool) -> Result<(), Error>;
    fn part1(&mut self) -> Result<RunOutput, Error>;
    fn part2(&mut self) -> Result<RunOutput, Error>;
}

pub type NewRunner = fn() -> Box<dyn Runner>;

#[macro_export]
macro_rules! print {
    () => {};

    (FORCE $($args:tt)*) => {
        Output::print(format_args!($($args)*));
    };

    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            Output::print(format_args!($($args)*));
        }
    };
}

#[macro_export]
macro_rules! println {
    (FORCE) => {
        Output::println(format_args!(""));
    };

    (FORCE $($args:tt)*) => {
        Output::println(format_args!($($args)*));
    };

    () => {
        if cfg!(debug_assertions) {
            Output::println(format_args!(""));
        }
    };

    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            Output::println(format_args!($($args)*));
        }
    };
}
