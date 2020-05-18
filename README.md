# dijkstra.rs

A demo implementation of Dijkstra algorithm in Rust. The main assumption is that the graph is immutable once all the nodes and edges have been added. This allows an implementation that uses arrays rather than pointers to heap allocated values.

Run as

    unzip -p graph.zip | cargo run -- --source=111 --targets=899,989,998

which sould yield

    path: [178, 0, 20, 156, 321, 524, 725, 891, 1036, 1175, 1331, 1514, 1355, 1518, 1343, 1539, 1379, 1569, 1586, 1597, 1611, 1468, 1643, 1656, 1645, 1474, 1309, 1511, 1658]
    cost: 574