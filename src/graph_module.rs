use std::collections::VecDeque;

/// bfs(幅優先探索)
/// nodes内のfromからの各nodeの最短距離を返します。
/// 到達できない場合は-1が入っています。
pub fn bfs(from: usize, nodes: &Vec<Vec<usize>>) -> Vec<isize> {
    let node_num = nodes.len();
    let mut todo: VecDeque<usize> = VecDeque::new();
    let mut visited = vec![-1; node_num];

    todo.push_back(from);
    visited[from] = 0;

    while !todo.is_empty() {
        let visit_node = todo.pop_front().unwrap();
        let push_nodes = &nodes[visit_node];
        for n in push_nodes {
            // 訪れていないnodeだけをtodoに追加
            if visited[*n] != -1 {
                continue;
            }
            visited[*n] = visited[visit_node] + 1;
            todo.push_back(*n);
        }
    }
    return visited;
}

