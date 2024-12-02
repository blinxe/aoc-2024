use aoc_2024::days::ALL_DAYS;

fn main() {
    let n = ALL_DAYS.len();
    let (pt1, pt2) = ALL_DAYS.last().unwrap();

    println!("\x1b[30m====\x1b[m");
    println!("\x1b[32m# DAY {:02}\x1b[m", n);
    println!("\x1b[30m----\x1b[m");
    println!("\x1b[34m## Part 1\x1b[m");
    pt1();
    println!("\x1b[30m----\x1b[m");
    println!("\x1b[34m## Part 2\x1b[m");
    pt2();
    println!();
}
