use graph::input_module;
use graph::graph_module;

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
    let is_bipartite = graph_module::bipartite_graph_bfs(0, &node_vec);
    println!("{}", is_bipartite);

}

