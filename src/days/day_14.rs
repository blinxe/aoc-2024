use regex::Regex;

use crate::utils::input::read_input;

type Pos = (isize, isize);
type Velo = (isize, isize);

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

fn parse_input(input: &str) -> Vec<(Pos, Velo)> {
    let re = Regex::new(r"p=(.*),(.*) v=(.*),(.*)").unwrap();
    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).unwrap();
            let px = cap.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let py = cap.get(2).unwrap().as_str().parse::<isize>().unwrap();
            let vx = cap.get(3).unwrap().as_str().parse::<isize>().unwrap();
            let vy = cap.get(4).unwrap().as_str().parse::<isize>().unwrap();
            ((px, py), (vx, vy))
        })
        .collect()
}

fn move_bot(start: Pos, v: Velo, s: isize) -> Pos {
    (
        (start.0 + s * v.0).rem_euclid(WIDTH),
        (start.1 + s * v.1).rem_euclid(HEIGHT),
    )
}

fn solve_part_1(input: &str) {
    let bots = parse_input(input);
    let bots: Vec<_> = bots.iter().map(|b| move_bot(b.0, b.1, 100)).collect();

    let tl = bots
        .iter()
        .filter(|b| b.0 < WIDTH / 2 && b.1 < HEIGHT / 2)
        .count();
    let tr = bots
        .iter()
        .filter(|b| b.0 > WIDTH / 2 && b.1 < HEIGHT / 2)
        .count();
    let bl = bots
        .iter()
        .filter(|b| b.0 < WIDTH / 2 && b.1 > HEIGHT / 2)
        .count();
    let br = bots
        .iter()
        .filter(|b| b.0 > WIDTH / 2 && b.1 > HEIGHT / 2)
        .count();

    println!("Safety factor: {}", tl * tr * bl * br);
}

fn mean(bots: &Vec<(Pos, Velo)>) -> (f64, f64) {
    let mx = bots.iter().map(|b| b.0 .0).sum::<isize>() as f64 / bots.len() as f64;
    let my = bots.iter().map(|b| b.0 .1).sum::<isize>() as f64 / bots.len() as f64;
    (mx, my)
}

fn variance(bots: &Vec<(Pos, Velo)>) -> (f64, f64) {
    let (mx, my) = mean(bots);
    let vx = bots
        .iter()
        .map(|b| (b.0 .0 as f64 - mx).powi(2))
        .sum::<f64>()
        .sqrt();
    let vy = bots
        .iter()
        .map(|b| (b.0 .1 as f64 - my).powi(2))
        .sum::<f64>()
        .sqrt();
    (vx, vy)
}

fn get_display_string(bots: &Vec<(Pos, Velo)>) -> String {
    let mut screen = [[' '; WIDTH as usize]; HEIGHT as usize];
    for b in bots.iter() {
        screen[b.0 .1 as usize][b.0 .0 as usize] = '#';
    }
    let mut string = String::new();
    for l in 0..HEIGHT {
        for c in 0..WIDTH {
            string.push(screen[l as usize][c as usize]);
        }
        string.push('\n');
    }

    string
}

fn solve_part_2(input: &str) {
    let bots_init = parse_input(input);

    // find best x frame in cycle
    let mut bots = bots_init.clone();
    let mut min_var_x = (WIDTH * WIDTH) as f64;
    let mut min_var_y = (HEIGHT * HEIGHT) as f64;
    let mut best_x_s = WIDTH;
    let mut best_y_s = HEIGHT;
    for s in 1..(WIDTH.max(HEIGHT)) {
        for b in bots.iter_mut() {
            b.0 = move_bot(b.0, b.1, 1);
        }
        let (vx, vy) = variance(&bots);
        if vx < min_var_x {
            min_var_x = vx;
            best_x_s = s;
        }
        if vy < min_var_y {
            min_var_y = vy;
            best_y_s = s;
        }
    }
    println!("tighest x grouping at s={}", best_x_s);
    println!("tighest y grouping at s={}", best_y_s);

    let mut best_s = -1;
    for xcycles in 0..HEIGHT {
        if (xcycles * WIDTH + best_x_s) % HEIGHT == best_y_s {
            best_s = xcycles * WIDTH + best_x_s;
            break;
        }
    }

    let mut bots = bots_init.clone();
    for b in bots.iter_mut() {
        b.0 = move_bot(b.0, b.1, best_s);
    }
    let string = get_display_string(&bots);
    println!("{}", string);

    println!("Best grouping at t={}s", best_s);
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
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
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
