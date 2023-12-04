// Change the year and day to the day you want to run
use lib::solutions::sol_2023_01::solver;

fn main() -> anyhow::Result<()> {
    let start_time = std::time::Instant::now();
    let solution = solver()?.solve();
    let end_time = std::time::Instant::now();

    println!("Output: {}", solution);
    println!("Took: {} ms", (end_time - start_time).as_millis());

    Ok(())
}
