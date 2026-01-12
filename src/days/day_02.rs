use itertools::Itertools;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|list| {
            list.split(" ")
                .map(|str| str.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn is_valid(vals: &[i32]) -> bool {
    vals.iter()
        .tuple_windows()
        .map(|(n1, n2)| n2 - n1)
        .tuple_windows()
        .all(|(n1, n2)| {
            n1.signum() == n2.signum()
                && 1 <= n1.abs()
                && n1.abs() <= 3
                && 1 <= n2.abs()
                && n2.abs() <= 3
        })
}

fn is_valid_p2(vals: &[i32]) -> bool {
    if is_valid(vals) {
        return true;
    }

    for i in 0..vals.len() {
        let mut v = Vec::new();
        v.extend_from_slice(&vals[..i]);
        v.extend_from_slice(&vals[i + 1..]);

        if is_valid(&v) {
            return true;
        }
    }

    false
}

fn solve_part_1(input: &str) {
    let reports = parse_input(input);
    let answer = reports
        .iter()
        .filter_map(|vals| if is_valid(&vals) { Some(()) } else { None })
        .count();

    println!("{:?}", answer);
}

fn solve_part_2(input: &str) {
    let reports = parse_input(input);
    let answer = reports
        .iter()
        .filter_map(|vals| if is_valid_p2(&vals) { Some(()) } else { None })
        .count();

    println!("{}", answer);
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
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
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
