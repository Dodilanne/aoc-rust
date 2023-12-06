use crate::utils::solver::Solver;

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 3, solve_p1)
}

fn solve_p1(input: &str) -> String {
    let max_line_len = input
        .lines()
        .map(|l| l.len())
        .max()
        .expect("no max line len");
    let res = std::iter::repeat('.')
        .take(max_line_len + 2)
        .chain(std::iter::once('\n'))
        .chain(input.lines().flat_map(|l| {
            std::iter::once('.')
                .chain(l.chars())
                .chain(std::iter::once('.'))
                .chain(std::iter::once('\n'))
        }))
        .chain(std::iter::repeat('.').take(max_line_len + 2))
        .collect::<String>();


        println!("input: {}", input);
        println!("res: {}", res);

    String::from(res)
}

#[test]
fn test_solve_p1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(solve_p1(input), "4361");
}
