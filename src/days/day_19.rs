use crate::utils::input::read_input;
use memoize::memoize;
use regex::Regex;

type Pattern = String;
type Design = String;

fn parse_input(input: &str) -> (Vec<Pattern>, Vec<Design>) {
    let mut split = input.split("\n\n");
    let patterns = split
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    let designs = split
        .next()
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    (patterns, designs)
}

fn solve_part_1(input: &str) {
    let (patterns, designs) = parse_input(input);
    let r = format!("^({})*$", patterns.join("|"));
    let re = Regex::new(&r).unwrap();
    let mut sum = 0;
    for d in designs {
        if re.is_match(&d) {
            sum += 1;
        }
    }
    println!("Number of possible designs: {}", sum);
}

#[memoize(Ignore: patterns)]
fn count_matches(patterns: &Vec<Pattern>, input: String) -> usize {
    if input.is_empty() {
        return 1;
    }
    let mut sum = 0;
    for p in patterns {
        if input.starts_with(p) {
            sum += count_matches(patterns, input[p.len()..].to_string());
        }
    }
    sum
}

fn solve_part_2(input: &str) {
    let (patterns, designs) = parse_input(input);
    let mut sum = 0;
    for d in designs {
        let nb = count_matches(&patterns, d.to_string());
        sum += nb;
    }
    println!("Number of different ways: {}", sum);
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
        r, wr, b, g, bwu, rb, gb, br

        brwrr
        bggr
        gbbr
        rrbgbr
        ubwu
        bwurrg
        brgr
        bbrgwb
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
