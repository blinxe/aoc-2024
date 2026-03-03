use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use crate::utils::input::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i16,
    y: i16,
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, HashMap<Pos, HashSet<Pos>>) {
    let mut starts = HashMap::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.char_indices()
                .map(|(x, c)| {
                    let h = c.to_digit(10).unwrap() as u8;
                    if h == 0 {
                        let p = Pos {
                            x: x as i16,
                            y: y as i16,
                        };
                        starts.insert(p, HashSet::from([p]));
                    }
                    h
                })
                .collect()
        })
        .collect();

    (grid, starts)
}

fn neighbors(grid: &Vec<Vec<u8>>, p: &Pos) -> Vec<Pos> {
    let mut out = Vec::new();
    for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        if p.x + dx >= 0
            && p.x + dx < grid[0].len() as i16
            && p.y + dy >= 0
            && p.y + dy < grid.len() as i16
        {
            out.push(Pos {
                x: p.x + dx,
                y: p.y + dy,
            });
        }
    }
    out
}

fn solve_part_1(input: &str) {
    let (grid, mut steps) = parse_input(input);

    for h in 1..=9 {
        let mut next_steps = HashMap::new();
        for (p, origins) in steps {
            for n in neighbors(&grid, &p) {
                if grid[n.y as usize][n.x as usize] == h {
                    next_steps
                        .entry(n)
                        .or_insert(HashSet::new())
                        .extend(&origins);
                }
            }
        }
        steps = next_steps;
    }

    let score = steps
        .into_iter()
        .map(|(_, origins)| origins.len())
        .sum::<usize>();
    println!("Score: {}", score);
}

fn parse_input_v2(input: &str) -> (Vec<Vec<u8>>, HashMap<Pos, usize>) {
    let mut starts = HashMap::new();
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.char_indices()
                .map(|(x, c)| {
                    let h = c.to_digit(10).unwrap() as u8;
                    if h == 0 {
                        let p = Pos {
                            x: x as i16,
                            y: y as i16,
                        };
                        starts.insert(p, 1);
                    }
                    h
                })
                .collect()
        })
        .collect();

    (grid, starts)
}

fn solve_part_2(input: &str) {
    let (grid, mut steps) = parse_input_v2(input);

    for h in 1..=9 {
        let mut next_steps = HashMap::new();
        for (p, nb_paths) in steps {
            for n in neighbors(&grid, &p) {
                if grid[n.y as usize][n.x as usize] == h {
                    next_steps.entry(n).or_insert(0).add_assign(nb_paths);
                }
            }
        }
        steps = next_steps;
    }

    let score: usize = steps.values().sum();
    println!("Score: {}", score);
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
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
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
