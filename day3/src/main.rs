use std::fs;

fn part1() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .fold(0, |out, line| {
            let first_half = &line[0..line.len() / 2].chars().collect::<Vec<char>>();
            let second_half = &line[line.len() / 2..].chars().collect::<Vec<char>>();

            for c in first_half {
                if second_half.iter().any(|c2| c == c2) {
                    return match c.is_lowercase() {
                        true => out + (*c as i32) - 96,
                        false => out + (*c as i32) - 64 + 26,
                    };
                }
            }

            out
        })
}

fn part2() -> i32 {
    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .fold(0, |out, lines| {
            for c1 in lines[0].iter() {
                if lines[1].iter().any(|c2| c1 == c2) && lines[2].iter().any(|c3| c1 == c3) {
                    return match c1.is_lowercase() {
                        true => out + (*c1 as i32) - 96,
                        false => out + (*c1 as i32) - 64 + 26,
                    };
                }
            }

            out
        })
}

fn main() {
    println!("Day 3, part 1: {}", part1());
    println!("Day 3, part 2: {}", part2());
}
