use rayon::iter::{IntoParallelIterator, ParallelIterator};
use crate::signal::Signal;

#[derive(Debug, Clone, Copy)]
enum Gate {
    Not{state: Signal, inputs: NodeID},
    Nand{state: Signal, inputs: [NodeID; 2]},
    Nor{state: Signal, inputs: [NodeID; 2]},
    And{state: Signal, inputs: [NodeID; 2]},
    Or{state: Signal, inputs: [NodeID; 2]},
    Xor{state: Signal, inputs: [NodeID; 2]},
}

#[derive(Debug, Clone, Copy)]
struct NodeID(usize);

#[derive(Debug, Clone, Copy)]
struct InputID(usize);

#[derive(Debug, Clone, Copy)]
struct OutputID(usize);


#[derive(Debug, Clone, Copy)]
enum Node {
    Gate(Gate), // gate to retrieve signal from 
    Input(InputID), // input addr to retrieve signal from
    Output(OutputID, NodeID) // output addr to write signal to as well as a gate adress to read from
}

#[derive(Debug, Clone)]
struct Circuit {

}

impl Circuit {

}