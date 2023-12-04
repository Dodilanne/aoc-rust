use crate::utils::solver::Solver;

fn solve(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("no first number");
            let last = digits.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("invalid number")
        })
        .sum::<u32>();

    sum.to_string()
}

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 1, solve)
}
