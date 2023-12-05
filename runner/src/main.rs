// Change the year and day to the day you want to run
use lib::solutions::sol_2023_02::solver;

fn main() -> anyhow::Result<()> {
    let solver = solver()?;

    let start_time = std::time::Instant::now();
    let solution = solver.solve();
    let end_time = std::time::Instant::now();

    let elapsed = end_time - start_time;

    println!("\nSolution : {solution}");
    println!("Elapsed  : {}ms\n", elapsed.as_millis());

    Ok(())
}
