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

fn solve_part_1(input: &str) {
    let (mut vec1, mut vec2) = parse_input(input);
    vec1.sort();
    vec2.sort();
    let answer: u32 = vec1
        .into_iter()
        .zip(vec2)
        .map(|(e1, e2)| e2.abs_diff(e1))
        .sum();

    println!("{:?}", answer);
}

fn solve_part_2(input: &str) {
    let (vec1, vec2) = parse_input(input);

    let answer: u32 = vec1
        .into_iter()
        .map(|e1| e1 * vec2.iter().filter(|e2| **e2 == e1).count() as u32)
        .sum();

    println!("{:?}", answer);
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
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
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
