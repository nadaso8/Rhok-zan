#[derive(Clone, Copy, Debug)]
pub struct SignalID (pub usize);

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    Input(fn(usize) -> Result<super::signal::Signal, &'static str>),
    Output(SignalID, fn(usize, super::signal::Signal) -> Result<(), &'static str>),
    Not(SignalID),
    And(SignalID, SignalID),
    Nand(SignalID, SignalID),
    Or(SignalID, SignalID),
    Nor(SignalID, SignalID),
    Xor(SignalID, SignalID),
    Xnor(SignalID, SignalID)
}

