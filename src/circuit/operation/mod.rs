#[derive(Clone, Copy, Debug)]
pub struct SignalID (pub usize);

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Input(FnMut),
    Output(SignalID, FnMut),
    Not(SignalID),
    And(SignalID, SignalID),
    Nand(SignalID, SignalID),
    Or(SignalID, SignalID),
    Nor(SignalID, SignalID),
    Xor(SignalID, SignalID),
    Xnor(SignalID, SignalID)
}

