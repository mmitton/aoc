use helper::new_year::{Config, Day};
use helper::Error;

fn main() -> Result<(), Error> {
    let config = Config::new(
        "aoc",
        (1..=12)
            .map(|day| Day::new(day, if day == 12 { 1 } else { 2 }))
            .collect(),
    );

    helper::new_year::main(config)
}
