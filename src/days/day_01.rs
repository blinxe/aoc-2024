use crate::utils::input::read_input;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let e1 = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            let e2 = u32::from_str_radix(split.next().unwrap(), 10).unwrap();
            (e1, e2)
        })
        .unzip()
}

fn solve_part_1(input: &str) -> u32 {
    let (mut vec1, mut vec2) = parse_input(input);
    vec1.sort();
    vec2.sort();
    vec1.iter().zip(vec2).map(|(e1, e2)| e2.abs_diff(*e1)).sum()
}

fn solve_part_2(input: &str) -> u32 {
    let (vec1, vec2) = parse_input(input);

    vec1.into_iter()
        .map(|e1| e1 * vec2.iter().filter(|e2| **e2 == e1).count() as u32)
        .sum()
}

pub fn part_1() {
    let input = read_input(module_path!());
    let answer = solve_part_1(input.as_str());

    println!("{:?}", answer);
}

pub fn part_2() {
    let input = read_input(module_path!());
    let answer = solve_part_2(input.as_str());

    println!("{:?}", answer);
}

#[cfg(test)]
mod test {
    const EXAMPLE_1: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_part_1() {
        let answer = super::solve_part_1(EXAMPLE_1);
        println!("{:?}", answer);
    }

    const EXAMPLE_2: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_part_2() {
        let answer = super::solve_part_2(EXAMPLE_2);
        println!("{:?}", answer);
    }
}