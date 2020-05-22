# dijkstra.rs

A demo implementation of Dijkstra algorithm in Rust. The main assumption is that the graph structure is immutable once all the nodes and edges have been added. Node state and edge properties can be changed, but no nodes or edges can be deleted once they are in the graph. This allows an implementation that uses arrays rather than pointers to heap allocated values.

Each node/edge is characterised by an id that serves as the index of the node/edge in the array of nodes/edges. In addition to that, each node carries a list of outgoing edge ids, and each edge carries the from and to node ids. This information cannot be modified, unlike node state and edge properties which are mutable.

The search from the source node id to the target node ids relies on the user defined functions: 1) `advance` that advances a given node state along a given edge, 2) `cost` that returns the cost of a given state, 3) `update` that updates a given node state, with the state returned by advance if the cost of the new state is lower than the old cost.

Run as

    cargo build --release
    time cargo run --release -- --source=0 --targets=999 < <( unzip -p graph.zip )

which sould yield

    path: [3, 61, 69, 131, 729, 791, 1391, 1987, 1993, 2003, 2599, 2605, 2615, 3213, 3270, 3268, 2667, 2727, 2785, 2795, 3391, 3399, 3461, 4059, 4121, 4717, 4727, 5327, 5925, 5983, 5989]
    cost: 1212.0

for a search on a 3d grid with 10 nodes along each dimension, where each node is connected to all neighbours on the grid via edges with some randomly assigned cost. The grid is taken from graph.zip, a zipped json file with the serialisation of the graph data struct.
The path is a sequence of edge ids, where the from of edge 3 is node id 0, and the to of edge 5989 is node id 999.

The search uses rayon library to parallelise computations along outgoing edges of a given node, improving performance by about 60% for the example graph used above, from 32 seconds to 12 seconds on i7-4785T CPU @ 2.20GHz × 4. An artificial delay of 10 milliseconds was added for each advance call to simulate the compute time required to advance state. 

Each node has 6 outgoing edges, but on average only 3 are advanced in the search, since the others are terminating at the nodes that have been closed. There are 1000 nodes in the grid, which results in the search taking about 30 seconds, 10 milliseonds per edge. With parallelization, the number of outgoing edges is irrelevant as long as it is less than the number of available cores, which results in about 10 seconds.
