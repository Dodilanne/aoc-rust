use super::fetch_input::fetch_input;

pub struct Solver {
    input: String,
    solve: fn(&str) -> String,
}

impl Solver {
    pub fn new(year: u16, day: u8, solve: fn(&str) -> String) -> anyhow::Result<Self> {
        let input = fetch_input(year, day)?;
        Ok(Self { input, solve })
    }

    pub fn solve(&self) -> String {
        (self.solve)(&self.input)
    }
}
