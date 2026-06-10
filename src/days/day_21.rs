use itertools::Itertools;
use memoize::memoize;
use std::ops::Index;

use crate::utils::input::read_input;

type Code = String;

fn parse_input(input: &str) -> Vec<Code> {
    input.lines().map(|l| l.to_string()).collect()
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}
impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(PartialEq, Clone)]
enum KeyboardType {
    Digicode,
    Arrows,
}
#[derive(Clone, Hash, PartialEq, Eq)]
struct Keyboard {
    map: Vec<(char, Pos)>,
    cursor: char,
}

impl Index<char> for Keyboard {
    type Output = Pos;

    fn index(&self, index: char) -> &Self::Output {
        &self
            .map
            .iter()
            .filter(|entry| entry.0 == index)
            .next()
            .unwrap()
            .1
    }
}

impl Keyboard {
    fn new(layout: KeyboardType) -> Self {
        Self {
            map: match layout {
                KeyboardType::Digicode => Vec::from([
                    ('7', Pos::new(0, 0)),
                    ('8', Pos::new(1, 0)),
                    ('9', Pos::new(2, 0)),
                    ('4', Pos::new(0, 1)),
                    ('5', Pos::new(1, 1)),
                    ('6', Pos::new(2, 1)),
                    ('1', Pos::new(0, 2)),
                    ('2', Pos::new(1, 2)),
                    ('3', Pos::new(2, 2)),
                    ('0', Pos::new(1, 3)),
                    ('A', Pos::new(2, 3)),
                ]),
                _ => Vec::from([
                    ('^', Pos::new(1, 0)),
                    ('A', Pos::new(2, 0)),
                    ('<', Pos::new(0, 1)),
                    ('v', Pos::new(1, 1)),
                    ('>', Pos::new(2, 1)),
                ]),
            },
            cursor: 'A',
        }
    }
    fn contains(&self, p: &Pos) -> bool {
        self.map
            .iter()
            .filter(|entry| entry.1 == *p)
            .next()
            .is_some()
    }
    fn is_move_allowed(&mut self, moves: &[char]) -> bool {
        let mut coord = self[self.cursor];
        for &m in moves {
            match m {
                '<' => coord.x -= 1,
                '>' => coord.x += 1,
                '^' => coord.y -= 1,
                'v' => coord.y += 1,
                _ => panic!(),
            }
            if !self.contains(&coord) {
                return false;
            }
        }
        true
    }
    fn move_cursor(&mut self, dest: char) -> Vec<Vec<char>> {
        let src = self[self.cursor];
        let dst = self[dest];
        let dx = dst.x as isize - src.x as isize;
        let dy = dst.y as isize - src.y as isize;
        let mut moves = Vec::new();

        if dx > 0 {
            moves.append(&mut vec!['>'; dx as usize]);
            if dy > 0 {
                moves.append(&mut vec!['v'; dy as usize]);
            } else {
                moves.append(&mut vec!['^'; dy.abs() as usize]);
            }
        } else {
            moves.append(&mut vec!['<'; dx.abs() as usize]);
            if dy > 0 {
                moves.append(&mut vec!['v'; dy as usize]);
            } else {
                moves.append(&mut vec!['^'; dy.abs() as usize]);
            }
        }
        let len = moves.len();
        let possibilities: Vec<Vec<char>> = moves
            .into_iter()
            .permutations(len)
            .filter_map(|mut moves| {
                if self.is_move_allowed(&moves) {
                    moves.push('A');
                    Some(moves)
                } else {
                    None
                }
            })
            .collect();

        self.cursor = dest;
        let min_len = possibilities.iter().map(|m| m.len()).min().unwrap();
        possibilities
            .into_iter()
            .unique()
            .filter(|m| m.len() == min_len)
            .collect()
    }
}

#[memoize]
fn blx_recursive_func(instructions: Vec<char>, mut keyboards: Vec<Keyboard>) -> usize {
    let mut best_sequence = 0;
    for i in instructions {
        let sequence_list = keyboards[0].move_cursor(i);
        if keyboards.len() == 1 {
            best_sequence += sequence_list[0].len();
            continue;
        }
        let mut best_sequences = Vec::new();
        for s in sequence_list {
            best_sequences.push(blx_recursive_func(s, keyboards[1..].to_vec()));
        }
        best_sequence += best_sequences.iter().min().unwrap();
    }
    best_sequence
}

fn solve_part_1(input: &str) {
    let codes = parse_input(input);
    let mut results = Vec::new();
    let keyboards = [
        Keyboard::new(KeyboardType::Digicode),
        Keyboard::new(KeyboardType::Arrows),
        Keyboard::new(KeyboardType::Arrows),
    ];

    for c in codes {
        println!("{}", c);
        let length = blx_recursive_func(c.chars().collect(), keyboards.to_vec());
        let num_code = c[..3].parse::<usize>().unwrap();
        let res = length * num_code;
        println!("{} * {} = {}", num_code, length, res);
        results.push(res);
    }
    println!("Sum of complexity: {}", results.iter().sum::<usize>());
}

fn solve_part_2(input: &str) {
    let codes = parse_input(input);
    let mut results = Vec::new();
    let mut keyboards = Vec::new();

    keyboards.push(Keyboard::new(KeyboardType::Digicode));
    keyboards.append(&mut vec![Keyboard::new(KeyboardType::Arrows); 25]);

    for c in codes {
        println!("{}", c);
        let length = blx_recursive_func(c.chars().collect(), keyboards.clone());
        let num_code = c[..3].parse::<usize>().unwrap();
        let res = length * num_code;
        println!("{} * {} = {}", num_code, length, res);
        results.push(res);
    }
    println!("Sum of complexity: {}", results.iter().sum::<usize>());
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
        029A
        980A
        179A
        456A
        379A
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
