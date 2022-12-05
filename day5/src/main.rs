use std::fs;

use regex::Regex;

fn get_stacks(rows: usize, columns: usize) -> Vec<Vec<char>> {
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    lines[0..rows]
        .into_iter()
        .rev()
        .fold(vec![Vec::new(); columns], |mut out, line| {
            let chars = line.chars().collect::<Vec<char>>();

            for i in 0..columns {
                let id = chars[i + 1 + (i * 3)];

                if id != ' ' {
                    out[i].push(id);
                }
            }

            out
        })
}

fn get_instructions(rows: usize) -> Vec<(usize, usize, usize)> {
    let alpha_regex = Regex::new(r"[a-z]").unwrap();
    let input = fs::read_to_string("./input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    lines[rows + 2..]
        .into_iter()
        .fold(Vec::new(), |mut out, line| {
            let result = alpha_regex.replace_all(line, "");
            let counts = result
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            out.push((counts[0], counts[1], counts[2]));

            out
        })
}

fn part1(rows: usize, columns: usize) -> Vec<char> {
    let mut stacks = get_stacks(rows, columns);
    let instructions = get_instructions(rows);

    for (count, from, to) in instructions {
        let from_parsed = from - 1;
        let to_parsed = to - 1;

        for _ in 1..=count {
            let id = stacks[from_parsed].pop().unwrap();

            stacks[to_parsed].push(id);
        }
    }

    stacks.iter().fold(Vec::new(), |mut out, stack| {
        out.push(*stack.last().unwrap());

        out
    })
}

fn part2(rows: usize, columns: usize) -> Vec<char> {
    let mut stacks = get_stacks(rows, columns);
    let instructions = get_instructions(rows);

    for (count, from, to) in instructions {
        let from_parsed = from - 1;
        let to_parsed = to - 1;
        let mut to_move: Vec<char> = Vec::new();

        for _ in 1..=count {
            let id = stacks[from_parsed].pop().unwrap();

            to_move.push(id);
        }

        to_move
            .iter()
            .rev()
            .for_each(|id| stacks[to_parsed].push(*id));
    }

    stacks.iter().fold(Vec::new(), |mut out, stack| {
        out.push(*stack.last().unwrap());

        out
    })
}

fn main() {
    println!("Day 5, part 1: {:?}", part1(8, 9));
    println!("Day 5, part 2: {:?}", part2(8, 9));
}
