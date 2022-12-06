use std::{collections::HashSet, fs};

fn has_unique_elements(iter: &Vec<char>) -> bool {
    let mut uniq = HashSet::new();

    iter.into_iter().all(|x| uniq.insert(x))
}

fn get_start_of_message_index(num_unique_chars: usize) -> usize {
    let input = fs::read_to_string("./input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    let mut stack: Vec<char> = Vec::new();

    for i in 0..input.len() {
        stack.push(input[i]);

        if stack.len() == num_unique_chars {
            if has_unique_elements(&stack) {
                return i + 1;
            }

            stack.remove(0);
        }
    }

    0
}

fn main() {
    println!("Day 6, part 1: {:?}", get_start_of_message_index(4));
    println!("Day 6, part 2: {:?}", get_start_of_message_index(14));
}
