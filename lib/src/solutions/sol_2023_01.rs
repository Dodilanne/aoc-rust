use crate::utils::solver::Solver;

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 1, solve_pt_2)
}

#[allow(dead_code)]
fn solve_pt_1(input: &str) -> String {
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

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn solve_pt_2(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let mut digits = vec![];
            let mut index = 0;
            while index < line.len() {
                let sub_line = &line[index..];
                let nbr = NUMBERS.iter().enumerate().find_map(|(i, &nbr)| {
                    if sub_line.starts_with(nbr) {
                        Some(i + 1)
                    } else {
                        None
                    }
                });
                if let Some(nbr) = nbr {
                    index += NUMBERS[nbr - 1].len() - 1;
                    digits.push(nbr);
                } else {
                    if let Some(digit) = sub_line.chars().next().and_then(|c| c.to_digit(10)) {
                        digits.push(digit as usize);
                    }
                    index += 1;
                }
            }

            let mut it = digits.iter();
            let first = it.next().expect("no first number");
            let last = it.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("invalid number")
        })
        .sum::<u32>();

    sum.to_string()
}
