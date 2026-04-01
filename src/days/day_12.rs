use std::{cell::RefCell, collections::HashSet};

use itertools::Itertools;

use crate::utils::input::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Plot {
    x: isize,
    y: isize,
}

type Grid = Vec<Vec<char>>;
type Region = HashSet<Plot>;

fn parse_input(input: &str) -> (Grid, Vec<Region>) {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();

    let mut regions: Vec<RefCell<Region>> = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let mut appended = false;

            let c = grid[y][x];
            let mut idx_left_region = None;
            if x > 0 {
                let nx = x - 1;
                let ny = y;
                if grid[ny][nx] == c {
                    idx_left_region = regions.iter().position(|rc| {
                        rc.borrow().contains(&Plot {
                            x: nx as isize,
                            y: ny as isize,
                        })
                    });
                    let idx = idx_left_region.unwrap();
                    regions[idx].borrow_mut().insert(Plot {
                        x: x as isize,
                        y: y as isize,
                    });
                    appended = true;
                }
            }

            if y > 0 {
                let nx = x;
                let ny = y - 1;
                if grid[ny][nx] == c {
                    let rc_up_region = regions
                        .iter()
                        .find(|rc| {
                            rc.borrow().contains(&Plot {
                                x: nx as isize,
                                y: ny as isize,
                            })
                        })
                        .unwrap();
                    rc_up_region.borrow_mut().insert(Plot {
                        x: x as isize,
                        y: y as isize,
                    });
                    appended = true;
                    if let Some(idx_rclr) = idx_left_region {
                        if regions[idx_rclr] != *rc_up_region {
                            let merged: Region = rc_up_region
                                .take()
                                .union(&regions[idx_rclr].take())
                                .cloned()
                                .collect();
                            rc_up_region.replace(merged);
                            regions.remove(idx_rclr);
                        }
                    }
                }
            }

            if !appended {
                regions.push(RefCell::new(Region::from([Plot {
                    x: x as isize,
                    y: y as isize,
                }])));
            }
        }
    }

    let regions = regions.iter().map(|rc| rc.take()).collect();
    (grid, regions)
}

fn plot_perimeter(grid: &Grid, plot: Plot) -> usize {
    let mut p = 0;
    for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let x = plot.x + dx;
        let y = plot.y + dy;
        if y >= 0 && y < grid.len() as isize && x >= 0 && x < grid[0].len() as isize {
            if grid[y as usize][x as usize] != grid[plot.y as usize][plot.x as usize] {
                p += 1;
            }
        } else {
            p += 1; // edge / no neighbor
        }
    }
    p
}

fn solve_part_1(input: &str) {
    let (grid, regions) = parse_input(input);
    let price: usize = regions
        .iter()
        .map(|r| r.len() * r.iter().map(|p| plot_perimeter(&grid, *p)).sum::<usize>())
        .sum();

    println!("Price: {}", price);
}

fn get_neighborhood_4(grid: &Grid, plot: Plot) -> Vec<Plot> {
    let mut neighbors = Vec::new();
    for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let nx = plot.x + dx;
        let ny = plot.y + dy;
        if ny >= 0 && ny < grid.len() as isize && nx >= 0 && nx < grid[0].len() as isize {
            if grid[ny as usize][nx as usize] == grid[plot.y as usize][plot.x as usize] {
                neighbors.push(Plot {
                    x: nx as isize,
                    y: ny as isize,
                })
            }
        }
    }

    neighbors
}

fn get_orthogonal_neighbors(neighbors: &Vec<Plot>) -> Vec<(Plot, Plot)> {
    neighbors
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let p1 = pair[0];
            let p2 = pair[1];
            if p1.x != p2.x && p1.y != p2.y {
                Some((*p1, *p2))
            } else {
                None
            }
        })
        .collect()
}

fn plot_count_corners(grid: &Grid, p: Plot) -> usize {
    let neighbors = get_neighborhood_4(grid, p);

    match neighbors.len() {
        0 => 4,
        1 => 2,
        2..=4 => {
            let ortho_pairs = get_orthogonal_neighbors(&neighbors);
            if ortho_pairs.len() == 0 {
                0
            } else {
                let mut corners = if ortho_pairs.len() == 1 { 1 } else { 0 }; // here is an explanatory comment
                for (p1, p2) in ortho_pairs {
                    let diag_x;
                    let diag_y;
                    if p1.x == p.x {
                        diag_x = p2.x;
                        diag_y = p1.y;
                    } else {
                        diag_x = p1.x;
                        diag_y = p2.y;
                    }
                    if grid[diag_y as usize][diag_x as usize] != grid[p1.y as usize][p1.x as usize]
                    {
                        corners += 1;
                    }
                }
                corners
            }
        }
        _ => panic!(),
    }
}

fn solve_part_2(input: &str) {
    let (grid, regions) = parse_input(input);
    let price: usize = regions
        .iter()
        .map(|r| {
            r.len()
                * r.iter()
                    .map(|p| plot_count_corners(&grid, *p))
                    .sum::<usize>()
        })
        .sum();

    println!("Price: {}", price);
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
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
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
