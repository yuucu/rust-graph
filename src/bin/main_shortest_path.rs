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
    // println!("{:?}", node_vec);

    let input2 = input_module::get_input();
    let judge_num = input_module::convert_num(input2);

    for _i in 0..judge_num {
        let input_tmp = input_module::get_input();
        let nums_tmp = input_module::convert_nums(input_tmp);
        let from = nums_tmp[0];
        let to = nums_tmp[1];
        let visited = graph_module::bfs(from, &node_vec);
        let result = visited[to];
        // println!("{:?}", visited);
        println!("{}", result);
    }
}
