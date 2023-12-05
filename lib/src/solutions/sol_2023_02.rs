use crate::utils::solver::Solver;

pub fn solver() -> anyhow::Result<Solver> {
    Solver::new(2023, 2, solve_pt_1)
}

fn solve_pt_1(input: &str) -> String {
    let res = input
        .lines()
        .filter_map(|l| -> Option<u32> {
            let truncated_line = l.replace("Game ", "");
            let split: Vec<&str> = truncated_line.split(':').collect();
            let game_id = split[0].parse::<u32>().expect("game id should be a number");

            let (red, green, blue) = split[1].split(";").fold((0, 0, 0), |mut acc, draw| {
                draw.trim().split(",").for_each(|data| {
                    let split = data.trim().split(" ").collect::<Vec<_>>();
                    let qty = split[0]
                        .parse::<u32>()
                        .expect("quantity should be a number");
                    match split[1] {
                        "red" => acc.0 = std::cmp::max(acc.0, qty),
                        "green" => acc.1 = std::cmp::max(acc.1, qty),
                        "blue" => acc.2 = std::cmp::max(acc.2, qty),
                        _ => {}
                    };
                });
                acc
            });

            if red <= 12 && green <= 13 && blue <= 14 {
                Some(game_id)
            } else {
                None
            }
        })
        .sum::<u32>();

    res.to_string()
}

#[test]
fn test_pt_1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(solve_pt_1(input), "8");
}
