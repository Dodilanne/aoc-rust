use std::io::Write;

use lib::solutions::sol_2022_01;

fn main() -> anyhow::Result<()> {
    let solver = sol_2022_01::solver()?;
    let solution = solver.solve();

    let mut stdout = std::io::stdout();
    stdout.write_all(solution.as_bytes())?;

    Ok(())
}
