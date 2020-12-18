use std::io;

pub fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    return input;
}

pub fn convert_num(input: String) -> usize {
    let num: usize = input.trim().parse().ok().unwrap();
    return num;
}

pub fn convert_nums(input: String) -> Vec<usize> {
    let nums: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().ok().unwrap())
        .collect();
    return nums;
}
