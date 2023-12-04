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

fn solve_pt_2(input: &str) -> String {
    let sum = input
        .lines()
        .map(|line| {
            let mut index = 0;
            let parsed_line = std::iter::from_fn(move || {
                let reduced_line = &line[index..];
                let result = if reduced_line.starts_with("one") {
                    Some('1')
                } else if reduced_line.starts_with("two") {
                    Some('2')
                } else if reduced_line.starts_with("three") {
                    Some('3')
                } else if reduced_line.starts_with("four") {
                    Some('4')
                } else if reduced_line.starts_with("five") {
                    Some('5')
                } else if reduced_line.starts_with("six") {
                    Some('6')
                } else if reduced_line.starts_with("seven") {
                    Some('7')
                } else if reduced_line.starts_with("eight") {
                    Some('8')
                } else if reduced_line.starts_with("nine") {
                    Some('9')
                } else {
                    let result = reduced_line.chars().next();
                    result
                };

                index += 1;

                result
            });

            let mut digits = parsed_line.filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("no first number");
            let last = digits.last().unwrap_or(first);

            format!("{first}{last}")
                .parse::<u32>()
                .expect("invalid number")
        })
        .sum::<u32>();

    sum.to_string()
}

#[test]
fn test_pt_2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    let output = "281";
    assert_eq!(solve_pt_2(input), output);
}
