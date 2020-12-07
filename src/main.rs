use std::io;

fn main() {
    let input1 = get_input();
    let nums = convert_nums(input1);
    let node_num = nums[0];
    let edge_num = nums[1];
    let node_vec = vec![vec![0u32; edge_num as usize]; node_num as usize];

    for i in 0..edge_num {
        let input_tmp = get_input();
        let nums_tmp = convert_nums(input_tmp);
        let node1 = nums_tmp[0];
        let node2 = nums_tmp[1];
        println!("node1: {}, node2: {}", node1, node2);
    }
}

fn get_input() -> String {
    let mut input=String::new();
    io::stdin().read_line(&mut input).ok();
    return input;
}

fn convert_nums(input: String) -> Vec<u32>{
    let nums: Vec<u32> = input.trim().split_whitespace()
        .map(|n| n.parse().ok().unwrap()).collect();
    nums
}

