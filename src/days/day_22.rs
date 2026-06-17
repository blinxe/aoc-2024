use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use crate::utils::input::read_input;

type Secret = u64;
type Sequence = [i8; 4];

fn parse_input(input: &str) -> Vec<Secret> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn gen_next_secret(secret: Secret) -> Secret {
    let mut out = ((secret * 64) ^ secret) % 16777216;
    out = ((out / 32) ^ out) % 16777216;
    out = ((out * 2048) ^ out) % 16777216;

    out
}

fn solve_part_1(input: &str) {
    let ids = parse_input(input);

    let mut sum = 0u64;
    for id in ids {
        let mut secret = id;
        for _ in 0..2000 {
            secret = gen_next_secret(secret);
        }

        sum += secret as u64;
    }

    println!("Sum of 2000th secrets: {}", sum);
}

fn solve_part_2(input: &str) {
    let ids = parse_input(input);

    let mut sequence_scores: HashMap<Sequence, usize> = HashMap::new();

    for id in ids {
        let mut found_sequences: HashSet<Sequence> = HashSet::new();
        let mut sequence: Vec<i8> = Vec::new();
        let mut secret = id;
        for _ in 0..2000 {
            let prev_bananas = (secret % 10) as i8;
            secret = gen_next_secret(secret);
            let new_bananas = (secret % 10) as i8;
            sequence.push(new_bananas - prev_bananas);
            if sequence.len() > 4 {
                sequence.remove(0);

                let seqarr = sequence.as_array().unwrap();
                if found_sequences.contains(seqarr) {
                    continue;
                }
                found_sequences.insert(seqarr.to_owned());

                sequence_scores
                    .entry(seqarr.to_owned())
                    .or_insert(0)
                    .add_assign(new_bananas as usize);
            }
        }
    }

    let max = sequence_scores.iter().max_by_key(|(_, v)| **v).unwrap();
    println!("Most bananas: {}", max.1);
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
        1
        10
        100
        2024
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = indoc! {"
        1
        2
        3
        2024
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
