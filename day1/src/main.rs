use std::fs;

fn part1() -> i32 {
    let mut max = 0;
    let mut curr = 0;

    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .for_each(|line| match line {
            "" => {
                if curr > max {
                    max = curr;
                }

                curr = 0;
            }
            l => curr += l.parse::<i32>().unwrap(),
        });

    max
}

fn part2() -> i32 {
    let mut totals = vec![];
    let mut curr = 0;

    fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .for_each(|line| match line {
            "" => {
                totals.push(curr);
                curr = 0;
            }
            l => curr += l.parse::<i32>().unwrap(),
        });

    totals.sort_by(|a, b| b.cmp(a));
    totals[..3].into_iter().fold(0, |out, curr| out + curr)
}

fn main() {
    println!("Day 1, part 1: {}", part1());
    println!("Day 1, part 2: {}", part2());
}
