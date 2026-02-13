use std::collections::{HashMap, HashSet};

use crate::utils::input::read_input;

fn parse_input(input: &str) -> (Vec<(i32, i32)>, (i32, i32)) {
    let mut guard = (0i32, 0i32);
    let obstacles = input
        .lines()
        .enumerate()
        .map(|(ln, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(cn, c)| match c {
                    '#' => Some((ln as i32, cn as i32)),
                    '^' => {
                        guard = (ln as i32, cn as i32);
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    (obstacles, guard)
}

fn solve_part_1(input: &str) {
    let (obstacles, mut guard) = parse_input(input);

    let lmax = input.lines().count() as i32;
    let cmax = input.lines().next().unwrap().len() as i32;

    let mut direction = '^';
    let mut visited = HashSet::new();
    while (0 <= guard.0) && (guard.0 < lmax) && (0 <= guard.1) && (guard.1 < cmax) {
        visited.insert(guard);
        let next = match direction {
            '^' => (guard.0 - 1, guard.1),
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => (guard.0, guard.1 - 1),
            _ => (0, 0),
        };
        if obstacles.contains(&next) {
            direction = match direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => direction,
            }
        } else {
            guard = next;
        }
    }

    println!("Visited positions: {}", visited.len());
}

fn solve_part_2(input: &str) {
    let (mut obstacles, guard_init) = parse_input(input);

    let lmax = input.lines().count() as i32;
    let cmax = input.lines().next().unwrap().len() as i32;

    let mut direction = '^';
    let mut guard = guard_init;
    let mut visited: HashMap<(i32, i32), HashSet<char>> = HashMap::new();
    while (0 <= guard.0) && (guard.0 < lmax) && (0 <= guard.1) && (guard.1 < cmax) {
        visited
            .entry(guard)
            .or_insert(HashSet::new())
            .insert(direction);
        let next = match direction {
            '^' => (guard.0 - 1, guard.1),
            '>' => (guard.0, guard.1 + 1),
            'v' => (guard.0 + 1, guard.1),
            '<' => (guard.0, guard.1 - 1),
            _ => (0, 0),
        };
        if obstacles.contains(&next) {
            direction = match direction {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => direction,
            }
        } else {
            guard = next;
        }
    }

    visited.remove(&guard_init);

    let mut count = 0;
    for (i, (gl, gc)) in visited.keys().enumerate() {
        println!("{}/{}", i, visited.len());
        obstacles.push((*gl, *gc));
        let mut direction = '^';
        let mut guard = guard_init;
        let mut visited: HashMap<(i32, i32), HashSet<char>> = HashMap::new();
        while (0 <= guard.0) && (guard.0 < lmax) && (0 <= guard.1) && (guard.1 < cmax) {
            if visited
                .entry(guard)
                .or_insert(HashSet::new())
                .insert(direction)
                == false
            {
                count += 1;
                break;
            }
            let next = match direction {
                '^' => (guard.0 - 1, guard.1),
                '>' => (guard.0, guard.1 + 1),
                'v' => (guard.0 + 1, guard.1),
                '<' => (guard.0, guard.1 - 1),
                _ => (0, 0),
            };
            if obstacles.contains(&next) {
                direction = match direction {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => direction,
                }
            } else {
                guard = next;
            }
        }
        obstacles.pop();
    }

    println!("Loops: {}", count);
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
        ....#.....
        .........#
        ..........
        ..#.......
        .......#..
        ..........
        .#..^.....
        ........#.
        #.........
        ......#...
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
