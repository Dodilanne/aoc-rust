use clap::Parser;
use std::io::Write;

/// Generate a solution template for a given year and day
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The year to generate a solution for
    #[arg(short, long, default_value = "2023")]
    year: u16,

    /// The day to generate a solution for
    #[arg(short, long)]
    day: u8,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    generate_solution_file(&args)?;
    append_to_mod_file(&args)?;
    update_runner(&args)?;

    Ok(())
}

fn append_to_mod_file(args: &Args) -> anyhow::Result<()> {
    let mod_path = format!("lib/src/solutions/mod.rs");
    let contents = format!(
        r#"pub mod sol_{}_{:0>2};
"#,
        args.year, args.day
    );
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(mod_path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

fn generate_solution_file(args: &Args) -> anyhow::Result<()> {
    let solution_path = format!("lib/src/solutions/sol_{}_{:0>2}.rs", args.year, args.day);
    let contents = format!(
        r#"use crate::utils::solver::Solver;

fn solve(input: &str) -> String {{
    println!("{{}}", input);
    String::from(input)
}}

pub fn solver() -> anyhow::Result<Solver> {{
    Solver::new({}, {}, solve)
}}
"#,
        args.year, args.day
    );
    std::fs::write(solution_path, contents)?;

    Ok(())
}

fn update_runner(args: &Args) -> anyhow::Result<()> {
    let path = "runner/src/main.rs";
    let contents = format!(
        r#"use std::io::Write;

// Change the year and day to the day you want to run
use lib::solutions::sol_{}_{:0>2}::solver;

fn main() -> anyhow::Result<()> {{
    let solution = solver()?.solve();
    let mut stdout = std::io::stdout();
    stdout.write_all(solution.as_bytes())?;

    Ok(())
}}
"#,
        args.year, args.day
    );
    std::fs::write(path, contents)?;

    Ok(())
}
