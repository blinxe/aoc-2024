use memoize::memoize;
use std::vec;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[memoize]
fn apply_rules(stone: usize) -> Vec<usize> {
    if stone == 0 {
        vec![1]
    } else if stone.to_string().len().is_multiple_of(2) {
        let stone_str = stone.to_string();
        vec![
            stone_str[0..stone_str.len() / 2].parse().unwrap(),
            stone_str[stone_str.len() / 2..].parse().unwrap(),
        ]
    } else {
        vec![stone * 2024]
    }
}

fn blink(stones: Vec<usize>, number: u8) -> Vec<usize> {
    let new_stones = stones.iter().flat_map(|s| apply_rules(*s)).collect();
    if number > 1 {
        return blink(new_stones, number - 1);
    }
    return new_stones;
}

#[memoize]
fn get(stone: usize, round: u8) -> usize {
    if round == 1 {
        return apply_rules(stone).len();
    }
    let mut sum = 0;
    for s in apply_rules(stone) {
        sum += get(s, round - 1);
    }
    sum
}

fn solve_part_1(input: &str) {
    let stones = parse_input(input);
    let stones = blink(stones, 25);
    println!("{:?}", stones.len());
}

fn solve_part_2(input: &str) {
    let stones = parse_input(input);
    let sum: usize = stones.into_iter().map(|s| get(s, 75)).sum();
    println!("{:?}", sum);
}

pub fn part_1() {
    let input = read_input(module_path!());
    solve_part_1(input.as_str());
}

pub fn part_2() {
    let input = read_input(module_path!());
    solve_part_2(input.as_str());
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const EXAMPLE_1: &str = indoc! {"
        125 17
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
