// mod AST;
// mod SSA;

use petgraph::graph::*;
struct Netlist {
    graph: Graph<dyn NetlistCell + Sized?, NetlistBus, >,
}

trait NetlistCell {
    fn lower()
}

/// maps a source returned 
struct NetlistBus {
    source: String,
    drain: String,
}
