use crate::utils::input::read_input;

use itertools::Itertools;

type Equation = (usize, Vec<usize>);

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(": ");
            let result = s.next().unwrap().parse().unwrap();
            let operands = s
                .next()
                .unwrap()
                .split(' ')
                .map(|o| o.parse::<usize>().unwrap())
                .collect();
            (result, operands)
        })
        .collect()
}

fn is_solvable(eq: &Equation) -> bool {
    let nb_operators = eq.1.len() - 1;

    let possible_operators =
        itertools::repeat_n(['*', '+'], nb_operators).multi_cartesian_product();

    for v in possible_operators {
        let first_value = eq.1[0];
        let result =
            eq.1.iter()
                .skip(1)
                .enumerate()
                .fold(first_value, |acc, (i, operand)| match v[i] {
                    '*' => acc * operand,
                    '+' => acc + operand,
                    _ => panic!(),
                });
        if result == eq.0 {
            return true;
        }
    }
    false
}

fn is_solvable_v2(eq: &Equation) -> bool {
    let nb_operators = eq.1.len() - 1;

    let possible_operators =
        itertools::repeat_n(['+', '*', '|'], nb_operators).multi_cartesian_product();

    for v in possible_operators {
        let mut acc = eq.1[0];
        for (i, operand) in eq.1.iter().skip(1).enumerate() {
            match v[i] {
                '*' => acc *= operand,
                '+' => acc += operand,
                '|' => {
                    let op_str = operand.to_string();
                    acc = acc * (10_u32.pow(op_str.len() as u32) as usize) + operand;
                    // acc_str.push_str(operand.to_string().as_str());
                    // acc = acc_str.parse::<usize>().unwrap();
                }
                _ => panic!(),
            }
            if acc > eq.0 {
                break;
            }
        }
        if acc == eq.0 {
            return true;
        }
    }
    false
}

fn solve_part_1(input: &str) {
    let list = parse_input(input);
    let mut sum = 0;

    for e in list {
        if is_solvable(&e) {
            sum += e.0;
        }
    }
    println!("Sum {:?}", sum);
}

fn solve_part_2(input: &str) {
    let list = parse_input(input);
    let mut sum = 0;

    for e in list {
        if is_solvable_v2(&e) {
            sum += e.0;
        }
    }
    println!("Sum {:?}", sum);
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
        190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20
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
