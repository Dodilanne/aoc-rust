use crate::utils::solver::Solver;

fn solve(input: &str) -> String {
    println!("{}", input);
    String::from(input)
}

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 3, solve)
}
