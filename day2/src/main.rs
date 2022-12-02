use std::fs;

fn part1() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .fold(0, |out, pairs| {
            let player1 = pairs[0].as_str();
            let player2 = pairs[1].as_str();

            let choice_score = match player2 {
                "X" => 1, // Rock
                "Y" => 2, // Paper
                "Z" => 3, // Scissors
                _ => panic!("Unknown choice for player 2"),
            };

            let outcome_score = match (player1, player2) {
                // Paper beats rock | Scissors beats paper | Rock beats scissors
                ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
                // Draw when same choices
                ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
                // No points if you lose
                _ => 0,
            };

            out + choice_score + outcome_score
        })
}

fn part2() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
        .fold(0, |out, pairs| {
            let player1 = pairs[0].as_str();
            let desired_outcome = pairs[1].as_str();

            let player2 = match desired_outcome {
                // End in loss
                "X" => match player1 {
                    "A" => "C",
                    "B" => "A",
                    "C" => "B",
                    _ => panic!("Unknown choice for player 1"),
                },
                // End in draw - choose same as player 1
                "Y" => player1,
                // End in win
                "Z" => match player1 {
                    "A" => "B",
                    "B" => "C",
                    "C" => "A",
                    _ => panic!("Unknown choice for player 1"),
                },
                _ => panic!("Unknown desired outcome"),
            };

            let choice_score = match player2 {
                "A" => 1, // Rock
                "B" => 2, // Paper
                "C" => 3, // Scissors
                _ => panic!("Unknown choice for player 2"),
            };

            let outcome_score = match (player1, player2) {
                // Paper beats rock | Scissors beats paper | Rock beats scissors
                ("A", "B") | ("B", "C") | ("C", "A") => 6,
                // Draw when same choices
                (c1, c2) if c1.eq(c2) => 3,
                // No points if you lose
                _ => 0,
            };

            out + choice_score + outcome_score
        })
}

fn main() {
    println!("Day 1, part 1: {}", part1());
    println!("Day 1, part 2: {}", part2());
}
