use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::utils::input::read_input;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

type Cost = usize;

#[derive(Debug, Clone)]
struct Elt {
    cost: Cost,
    obstacle: bool,
}
struct Grid {
    data: Vec<Vec<Elt>>,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            data: vec![
                vec![
                    Elt {
                        cost: 0,
                        obstacle: false
                    };
                    size
                ];
                size
            ],
        }
    }
}

impl Index<&Pos> for Grid {
    type Output = Elt;

    fn index(&self, index: &Pos) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}
impl IndexMut<&Pos> for Grid {
    fn index_mut(&mut self, index: &Pos) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.data.iter() {
            for elt in line {
                if elt.obstacle {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            let mut split = l.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            Pos::new(x, y)
        })
        .collect()
}

fn get_neighbors(grid: &mut Grid, pos: Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let x = (pos.x as isize + dx) as usize;
        let y = (pos.y as isize + dy) as usize;
        if x < grid.data[0].len() && y < grid.data.len() {
            let np = Pos::new(x, y);
            if grid[&np].cost == 0 && !grid[&np].obstacle {
                neighbors.push(np);
            }
        }
    }
    neighbors
}

fn dijkstra(grid: &mut Grid) {
    let mut set = vec![Pos::new(0, 0)];
    let mut cost = 1;
    while !set.is_empty() {
        let mut next_set = Vec::new();
        for pos in set {
            for n in get_neighbors(grid, pos) {
                grid[&n].cost = cost;
                next_set.push(n);
            }
        }
        set = next_set;
        cost += 1;
    }
}

fn solve_part_1_with_params(input: &str, grid_size: usize, bytes_nb: usize) {
    let bytes = &parse_input(input);

    let mut grid = Grid::new(grid_size);
    for byte_pos in bytes.iter().take(bytes_nb) {
        grid[byte_pos].obstacle = true;
    }

    dijkstra(&mut grid);
    let steps = grid.data.last().unwrap().last().unwrap().cost;

    println!("Steps: {}", steps);
}

fn solve_part_1(input: &str) {
    solve_part_1_with_params(input, 71, 1024);
}

fn solve_part_2_with_params(input: &str, grid_size: usize) {
    let bytes = &parse_input(input);

    let mut low = 0;
    let mut high = bytes.len();
    while high != low + 1 {
        let curs = (high + low) / 2;
        let mut grid = Grid::new(grid_size);
        for byte_pos in bytes.iter().take(curs + 1) {
            grid[byte_pos].obstacle = true;
        }
        dijkstra(&mut grid);
        if grid.data.last().unwrap().last().unwrap().cost != 0 {
            // println!("{}/{}/{}: {:?} ok", low, curs, high, bytes[curs]);
            low = curs;
        } else {
            // println!("{}/{}/{}: {:?} not ok", low, curs, high, bytes[curs]);
            high = curs;
        }
    }

    println!("Blocking byte: {:?}", bytes[high]);
}

fn solve_part_2(input: &str) {
    solve_part_2_with_params(input, 71);
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
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn test_part_1() {
        super::solve_part_1_with_params(EXAMPLE_1, 7, 12);
    }

    const EXAMPLE_2: &str = EXAMPLE_1;

    #[test]
    fn test_part_2() {
        super::solve_part_2_with_params(EXAMPLE_2, 7);
    }
}
