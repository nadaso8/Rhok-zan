use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::signal::{Signal, SignalWidth};

#[derive(Debug, Clone, Copy)]
enum Gate {
    Not{state: Signal, inputs: NodeID},
    Nand{state: Signal, inputs: [NodeID; 2]},
    Nor{state: Signal, inputs: [NodeID; 2]},
    And{state: Signal, inputs: [NodeID; 2]},
    Or{state: Signal, inputs: [NodeID; 2]},
    Xor{state: Signal, inputs: [NodeID; 2]},
}

/// adress formatting for nodes in a circuit graph structure
#[derive(Debug, Clone, Copy)]
struct NodeID(usize);

#[derive(Debug, Clone, Copy)]
struct InputID(usize);

#[derive(Debug, Clone, Copy)]
struct OutputID(usize);


// data elements of nodes in a circuit graph structure
enum Node {
    Gate(Gate), // gate to retrieve signal from 
    Input(InputID), // input addr to retrieve signal from
    Output(OutputID, NodeID) // output addr to write signal to as well as a gate adress to read from
}

#[derive(Debug, Clone)]
enum Circuit {

}

impl Circuit {

}