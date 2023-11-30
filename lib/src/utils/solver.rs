use super::fetch_input::fetch_input;

pub struct Solver {
    input: String,
    year: u16,
    day: u8,
    solve: fn(&str) -> String,
}

impl Solver {
    pub fn new(year: u16, day: u8, solve: fn(&str) -> String) -> anyhow::Result<Self> {
        let input = fetch_input(year, day)?;
        Ok(Self {
            input,
            year,
            day,
            solve,
        })
    }

    pub fn solve(&self) -> String {
        (self.solve)(&self.input)
    }
}
