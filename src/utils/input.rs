use std::fs;

pub fn read_input(modpath: &str) -> String {
    let i = modpath.len() - 2;
    let fname = format!("inputs/input_{}.txt", &modpath[i..]);
    fs::read_to_string(fname).unwrap().replace("\r\n", "\n")
}
