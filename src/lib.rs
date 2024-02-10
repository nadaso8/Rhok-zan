use std::{ops::{BitAnd, BitOr, BitXor, Not}, sync::Arc, collections::HashMap};

/// different signal states produced by a gate or input and the time delay for their arrival in gate transitions
#[derive(Debug, Clone, Copy)]
enum Signal {
    True,
    False,
    UncontrolledTrue,
    UncontrolledFalse,
    HighImpedance,
    Undefined
}

impl Not for  Signal {
    type Output = Self;
    fn not(self) -> Signal {
        match self  {
            Signal::True => Signal::False,
            Signal::False => Signal::True,
            Signal::UncontrolledTrue => Signal::UncontrolledFalse,
            Signal::UncontrolledFalse => Signal::UncontrolledTrue,
            Signal::Undefined => Signal::Undefined,
            Signal::HighImpedance => Signal::HighImpedance,
        }
    }
}

impl BitAnd for Signal {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

impl BitOr for Signal {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl BitXor for Signal {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
enum NodeID{
    Gate(usize),
    Port(usize),
}

#[derive(Debug, Clone, Copy)]
enum Gate {
    Not{state: Signal, inputs: NodeID},
    Nand{state: Signal, inputs: [NodeID; 2]},
    Nor{state: Signal, inputs: [NodeID; 2]},
    And{state: Signal, inputs: [NodeID; 2]},
    Or{state: Signal, inputs: [NodeID; 2]},
    Xor{state: Signal, inputs: [NodeID; 2]},
}

/// A collection of gates which may be inspected or consumed in order to step it forward  
#[derive(Debug, Clone)]
struct Circuit(Vec<Gate>);

impl Circuit {
    /// move state of gates forward one gate transition
    fn eval(&self) -> Self{
        todo!()
    }
}

pub fn sim() -> Result<Vec<Signal>,&'static str> {
    todo!("sim loop")
}