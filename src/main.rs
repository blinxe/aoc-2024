use aoc_2024::days::ALL_DAYS;

fn main() {
    for (i, (pt1, pt2)) in ALL_DAYS.iter().enumerate() {
        println!("\x1b[30m====\x1b[m");
        println!("\x1b[32m# DAY {:02}\x1b[m", i + 1);
        println!("\x1b[30m----\x1b[m");
        println!("\x1b[34m## Part 1\x1b[m");
        pt1();
        println!("\x1b[30m----\x1b[m");
        println!("\x1b[34m## Part 2\x1b[m");
        pt2();
        println!();
    }
}
