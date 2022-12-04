use std::fs;

fn part1() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .fold(0, |out, line| {
            let assignments = line
                .split(",")
                .map(|a| {
                    a.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();

            let start_1 = assignments[0][0];
            let end_1 = assignments[0][1];
            let start_2 = assignments[1][0];
            let end_2 = assignments[1][1];

            if (start_1 >= start_2 && end_1 <= end_2) || (start_2 >= start_1 && end_2 <= end_1) {
                return out + 1;
            }

            out
        })
}

fn part2() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .fold(0, |out, line| {
            let assignments = line
                .split(",")
                .map(|a| {
                    a.split("-")
                        .map(|n| n.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>();

            let start_1 = assignments[0][0];
            let end_1 = assignments[0][1];
            let start_2 = assignments[1][0];
            let end_2 = assignments[1][1];

            if start_1 <= end_2 && start_2 <= end_1 {
                return out + 1;
            }

            out
        })
}

fn main() {
    println!("Day 4, part 1: {}", part1());
    println!("Day 4, part 2: {}", part2());
}
