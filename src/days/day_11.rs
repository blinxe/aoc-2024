use std::{collections::HashMap, usize, vec};

use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

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

fn get_cache(cache: &mut HashMap<usize, HashMap<u8, usize>>, stone: usize, round: u8) -> usize {
    if !cache.contains_key(&stone) {
        cache.insert(stone, HashMap::new());
    }
    if cache.get(&stone).unwrap().contains_key(&round) {
        return *cache.get(&stone).unwrap().get(&round).unwrap();
    }
    if round == 1 {
        return apply_rules(stone).len();
    }
    let mut sum = 0;
    for s in apply_rules(stone) {
        sum += get_cache(cache, s, round - 1);
    }
    cache.get_mut(&stone).unwrap().insert(round, sum);
    sum
}

fn blink_v2(stones: Vec<usize>, number: u8) -> usize {
    let mut cache = HashMap::new();
    let mut sum = 0;
    for s in stones {
        sum += get_cache(&mut cache, s, number);
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
    println!("{:?}", blink_v2(stones, 75));
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
