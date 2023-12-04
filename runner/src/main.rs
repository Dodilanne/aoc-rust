use std::io::Write;

// Change the year and day to the day you want to run
use lib::solutions::sol_2023_02::solver;

fn main() -> anyhow::Result<()> {
    let solution = solver()?.solve();
    let mut stdout = std::io::stdout();
    stdout.write_all(solution.as_bytes())?;

    Ok(())
}
