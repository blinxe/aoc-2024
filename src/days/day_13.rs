use crate::utils::input::read_input;
use regex::Regex;

#[derive(Debug)]
struct Equation {
    a: f64,
    b: f64,
    total: f64,
}

fn parse_input(input: &str) -> Vec<(Equation, Equation)> {
    let entries = input.split("\n\n");
    entries
        .map(|s| {
            let lines: Vec<_> = s.lines().collect();
            let re = Regex::new(r"X\+(.*), Y\+(.*)").unwrap();
            let caps = re.captures(lines[0]).unwrap();
            let mut eq1 = Equation {
                a: 0.,
                b: 0.,
                total: 0.,
            };
            let mut eq2 = Equation {
                a: 0.,
                b: 0.,
                total: 0.,
            };
            eq1.a = caps.get(1).unwrap().as_str().parse::<f64>().unwrap();
            eq2.a = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();

            let caps = re.captures(lines[1]).unwrap();
            eq1.b = caps.get(1).unwrap().as_str().parse::<f64>().unwrap();
            eq2.b = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();

            let re = Regex::new(r"X=(.*), Y=(.*)").unwrap();
            let caps = re.captures(lines[2]).unwrap();
            eq1.total = caps.get(1).unwrap().as_str().parse::<f64>().unwrap();
            eq2.total = caps.get(2).unwrap().as_str().parse::<f64>().unwrap();
            (eq1, eq2)
        })
        .collect()
}

fn solve_part_1(input: &str) {
    let equations = parse_input(input);
    let mut tokens = 0;
    for e in equations {
        let determinant = e.0.a * e.1.b - e.0.b * e.1.a;
        if determinant != 0.0 {
            let a = (e.0.total * e.1.b - e.1.total * e.0.b) / determinant;
            let b = (e.1.total * e.0.a - e.0.total * e.1.a) / determinant;

            if a == a.ceil() && b == b.ceil() && a < 100.0 && b < 100.0 {
                tokens += 3 * a as usize + b as usize;
            }
        }
    }
    println!("Tokens to win: {}", tokens);
}

fn solve_part_2(input: &str) {
    let mut equations = parse_input(input);
    let mut tokens = 0;
    for e in equations.iter_mut() {
        e.0.total += 10000000000000.0;
        e.1.total += 10000000000000.0;
        let determinant = e.0.a * e.1.b - e.0.b * e.1.a;
        if determinant != 0.0 {
            let a = (e.0.total * e.1.b - e.1.total * e.0.b) / determinant;
            let b = (e.1.total * e.0.a - e.0.total * e.1.a) / determinant;

            if a == a.ceil() && b == b.ceil() {
                tokens += 3 * a as usize + b as usize;
            }
        }
    }
    println!("Tokens to win: {}", tokens);
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
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
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
