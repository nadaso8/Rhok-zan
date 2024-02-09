use std::ops::{BitAnd, BitOr, BitXor, Not};

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
struct GateID(usize);


#[derive(Debug, Clone, Copy)]
enum Gate {
    Not{state: Signal, inputs: GateID},
    Nand{state: Signal, inputs: [GateID; 2]},
    Nor{state: Signal, inputs: [GateID; 2]},
    And{state: Signal, inputs: [GateID; 2]},
    Or{state: Signal, inputs: [GateID; 2]},
    Xor{state: Signal, inputs: [GateID; 2]},
}

impl Gate {
    /// mutates state based on the downstream states of the gate's inputs
    fn eval(&self,  ) -> Self {
        match self {
            Self::Not{state, inputs} => {

            },
            Self::Nand{state, inputs} => {

            },
            Self::Nor{state, inputs} => {

            },
            Self::And{state, inputs} => {

            },
            Self::Or{state, inputs} => {

            },
            Self::Xor{state, inputs} => {

            },
        }
    }
}

pub fn sim() -> Result<Vec<Signal>,&'static str> {
    todo!("sim loop")
}