

# q1
s-tパス判定

```
cat inputs/graph_input_st.txt  | cargo run --bin main_st_path
```

# q2
最短経路問題(重みなし)

```
cat inputs/graph_input_shortest_path.txt  | cargo run --bin main_shortest_path
```

# q3
二部グラフ判定

```
cat inputs/graph_input.txt  | cargo run --bin main_bipartite_graph
# false
cat inputs/bigraph_input.txt  | cargo run --bin main_bipartite_graph
# true
```

# q4
グラフの直径

```
cat inputs/graph_input.txt  | cargo run --bin main_graph_diameter
```
