use std::collections::HashMap;

use crate::utils::input::read_input;

fn parse_input(input: &str) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let mut split = input.split("\n\n");
    let rules = split.next().unwrap();
    let productions = split.next().unwrap();

    let mut rules_map = HashMap::new();
    rules.lines().for_each(|l| {
        rules_map
            .entry(l[0..2].parse::<u8>().unwrap())
            .or_insert(Vec::new())
            .push(l[3..].parse::<u8>().unwrap())
    });

    let productions = productions
        .lines()
        .map(|l| l.split(',').map(|p| p.parse::<u8>().unwrap()).collect())
        .collect();

    (rules_map, productions)
}

fn is_order_valid(page_1: u8, page_2: u8, rules: &HashMap<u8, Vec<u8>>) -> bool {
    match rules.get(&page_2) {
        Some(v) => !v.contains(&page_1),
        None => true,
    }
}

fn solve_part_1(input: &str) {
    let (rules, productions) = parse_input(input);
    let mut sum: usize = 0;
    for prods in productions {
        let mut valid = true;

        for (p_index, &p1) in prods[0..prods.len()].iter().enumerate() {
            for &p2 in &prods[p_index + 1..] {
                if !is_order_valid(p1, p2, &rules) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            sum += prods[prods.len() / 2] as usize;
        }
    }
    println!("Sum: {}", sum);
}

fn reorder_prod(prod: Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut res = prod.clone();

    for i in 0..res.len() - 1 {
        for j in i + 1..res.len() {
            if !is_order_valid(res[i], res[j], &rules) {
                res.insert(i, res[j]);
                res.remove(j + 1);
            }
        }
    }
    res
}

fn solve_part_2(input: &str) {
    let (rules, productions) = parse_input(input);
    let mut sum: usize = 0;
    for prod in productions {
        let mut valid = true;

        for (p_index, &p1) in prod[0..prod.len() - 1].iter().enumerate() {
            for &p2 in &prod[p_index + 1..] {
                if !is_order_valid(p1, p2, &rules) {
                    valid = false;
                    break;
                }
            }
        }
        if !valid {
            let reordered = reorder_prod(prod, &rules);
            sum += reordered[reordered.len() / 2] as usize;
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
        47|53
        97|13
        97|61
        97|47
        75|29
        61|13
        75|53
        29|13
        97|29
        53|29
        61|53
        97|53
        61|29
        47|13
        75|47
        97|75
        47|61
        75|61
        47|29
        75|13
        53|13

        75,47,61,53,29
        97,61,53,29,13
        75,29,13
        75,97,47,61,53
        61,13,29
        97,13,75,29,47
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
