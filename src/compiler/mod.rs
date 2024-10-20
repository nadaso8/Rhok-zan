// This module contians the (hopefully short term) IR which will be used after source language AST generation,
// In the future I expect to move to MLIR, as it becomes a more mature framework for dataflow representations.

use petgraph::{visit::IntoNodeReferences, Graph};
use std::collections::HashSet;

struct RzIR {
    pub blocks: Vec<Block>,
}

impl RzIR {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    /// Get a mutable reference to a block for editing
    fn edit_block(&mut self, block: BlockHandle) -> Result<&mut Block, RzIRErr> {
        match self.blocks.get_mut(block.0) {
            Some(value) => Result::Ok(value),
            None => return Result::Err(RzIRErr::UninstantiatedBlock),
        }
    }

    /// Get an immutable reference to a block
    fn get_block(&self, block: BlockHandle) -> Result<&Block, RzIRErr> {
        match self.blocks.get(block.0) {
            Some(value) => Result::Ok(value),
            None => return Result::Err(RzIRErr::UninstantiatedBlock),
        }
    }

    /// Create a new block
    fn add_block(&mut self, name: String) -> BlockHandle {
        let handle = BlockHandle(self.blocks.len());
        self.blocks.push(Block {
            name,
            op_graph: Graph::new(),
        });
        return handle;
    }

    /// remove a block along with all of it's associated module nodes from the IR
    /// this function must search the entire graph of each module so it may be slow for larger designs.
    fn remove_block(&mut self, block: BlockHandle) {
        for value in &mut self.blocks {
            // allocate cache for which indexes need to be decremented
            let mut indecies_to_decrement = Vec::new();

            // search graph for now invalid modules and cache module type indecies who's handle must be updated
            value
                .op_graph
                .retain_nodes(|graph, index| match *graph.node_weight(index).unwrap() {
                    Op::Module(handle) => {
                        if handle == block {
                            false
                        } else if handle > block {
                            indecies_to_decrement.push(index);
                            true
                        } else {
                            true
                        }
                    }
                    _ => true,
                });

            // update handle for cached indecies
            for index in indecies_to_decrement {
                match value.op_graph.node_weight_mut(index).unwrap() {
                    Op::Module(handle) => handle.0 = handle.0 - 1,
                    _ => (),
                }
            }
        }

        // remove block from blocks stack
        self.blocks.remove(block.0);
    }

    fn compile(&self) -> Box<[crate::sim::circuit::operation::Operation]> {
        todo!()
    }
}

struct Block {
    name: String,
    op_graph: Graph<Op, Relation>,
}

#[derive(PartialEq, Eq)]
enum Op {
    Module(BlockHandle),
    Input(Port),
    Output(Port),
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Not,
}

struct Relation {
    source: Port,
    drain: Port,
}

#[derive(PartialEq, Eq)]
struct Port(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BlockHandle(usize);

enum RzIRErr {
    SomeErr,
    UninstantiatedBlock,
}
