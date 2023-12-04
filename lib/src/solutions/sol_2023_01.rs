use crate::utils::solver::Solver;

fn solve(input: &str) -> String {
    let sum: usize = input
        .lines()
        .map(|line| -> usize {
            let mut f: Option<char> = None;
            let mut l: Option<char> = None;

            for c in line.chars() {
                if c.is_numeric() {
                    if f.is_some() {
                        l = Some(c);
                    } else {
                        f = Some(c);
                    }
                }
            }

            if l.is_none() {
                l = f;
            }

            if let Some(first) = f {
                if let Some(last) = l {
                    let res = format!("{}{}", first, last).parse::<usize>();
                    return match res {
                        Ok(nbr) => nbr,
                        _ => 0,
                    };
                }
            }

            return 0;
        })
        .sum();

    return sum.to_string();
}

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 1, solve)
}
