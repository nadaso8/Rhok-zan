
#[derive(Clone, Copy, Debug)]
pub struct SignalID (pub usize);

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Not(SignalID),
    And(SignalID, SignalID),
    Nand(SignalID, SignalID),
    Or(SignalID, SignalID),
    Nor(SignalID, SignalID),
    Xor(SignalID, SignalID),
    Xnor(SignalID, SignalID)
}

