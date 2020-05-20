# dijkstra.rs

A demo implementation of Dijkstra algorithm in Rust. The main assumption is that the graph structure is immutable once all the nodes and edges have been added. Node state and edge properties can be changed, but no nodes or edges can be deleted once they are in the graph. This allows an implementation that uses arrays rather than pointers to heap allocated values.

Each node/edge is characterised by an id that serves as the index of the node/edge in the array of nodes/edges. In addition to that, each node carries a list of outgoing edge ids, and each edge carries the from and to node ids. This information cannot be modified, unlike node state and edge properties which are mutable.

The search from the source node id to the target node ids relies on the user defined functions: 1) `advance` that advances a given node state along a given edge, 2) `cost` that returns the cost of a given state, 3) `update` that updates a given node state, with the state returned by advance if the cost of the new state is lower than the old cost.

Run as

    unzip -p graph.zip | cargo run -- --source=111 --targets=899,989,998

which sould yield

    path: [178, 0, 20, 156, 321, 524, 725, 891, 1036, 1175, 1331, 1514, 1355, 1518, 1343, 1539, 1379, 1569, 1586, 1597, 1611, 1468, 1643, 1656, 1645, 1474, 1309, 1511, 1658]
    cost: 574.0

for a search on a 3d grid with 10 nodes along each dimension, where each node is connected to some of its neighbours on the grid via edges with some randomly assigned cost. The grid is taken from graph.zip, a zipped json file with the serialisation of the graph data structs.
The path is a sequence of edge ids, where the from node id of edge 178 is node id 111, and the to node id of edge 1658 is node id 998.

The search uses rayon library to parallelise computations along outgoing edges of a given node, improving performance by about 40% for the example graph used above, from 4.1 seconds to 2.6 seconds on i7-4785T CPU @ 2.20GHz × 4. An artificial delay of 10 milliseconds was added for each advance call to simulate the compute time required to advance state.