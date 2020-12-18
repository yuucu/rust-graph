use graph::graph_module;
use graph::input_module;

fn main() {
    let input1 = input_module::get_input();
    let nums = input_module::convert_nums(input1);
    let node_num = nums[0];
    let edge_num = nums[1];
    let mut node_vec = vec![vec![0; 0]; node_num];

    for _i in 0..edge_num {
        let input_tmp = input_module::get_input();
        let nums_tmp = input_module::convert_nums(input_tmp);
        let node1 = nums_tmp[0];
        let node2 = nums_tmp[1];
        // 無向グラフなので互いにpush
        node_vec[node1].push(node2);
        node_vec[node2].push(node1);
    }
    let is_visited_from_0 = graph_module::bfs(0, &node_vec);

    let max_index_and_value_from_0 = is_visited_from_0
        .iter()
        .enumerate()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    let is_visited_from_max = graph_module::bfs(max_index_and_value_from_0.0, &node_vec);

    let max_index_and_value_from_max = is_visited_from_max
        .iter()
        .enumerate()
        .max_by(|(_, v1), (_, v2)| v1.cmp(v2))
        .unwrap();

    let answer = max_index_and_value_from_max.1;
    println!("{}", answer);
}
