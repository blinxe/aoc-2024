use std::usize;

use crate::utils::input::read_input;
use regex::Regex;

fn solve_part_1(input: &str) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum: usize = re
        .captures_iter(input)
        .map(|c| {
            let a = c.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let b = c.get(2).unwrap().as_str().parse::<usize>().unwrap();
            a * b
        })
        .sum();

    println!("Sum: {}", sum);
}

fn solve_part_2(input: &str) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();
    let matches = re.captures_iter(input);
    let mut sum = 0;
    let mut enabled = true;
    for m in matches {
        match m.get_match().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let a = m.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let b = m.get(2).unwrap().as_str().parse::<usize>().unwrap();
                    sum += a * b;
                }
            }
        }
    }

    println!("Sum: {}", sum);
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
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1(EXAMPLE_1);
    }

    const EXAMPLE_2: &str = indoc! {"
        xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    "};

    #[test]
    fn test_part_2() {
        super::solve_part_2(EXAMPLE_2);
    }
}
