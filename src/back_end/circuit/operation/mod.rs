use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct SignalID(pub usize);

#[derive(Debug, Clone)]
pub enum Operation<I, O>
where
    I: CircuitInput,
    O: CircuitOutput,
{
    Input(I),
    Output(SignalID, O),
    Not(SignalID),
    And(SignalID, SignalID),
    Nand(SignalID, SignalID),
    Or(SignalID, SignalID),
    Nor(SignalID, SignalID),
    Xor(SignalID, SignalID),
    Xnor(SignalID, SignalID),
}

pub trait CircuitInput: Debug + Clone + Sync + Send {
    fn recieve(&self, gate_id: usize, tick: u128) -> super::Signal;
}

pub trait CircuitOutput: Debug + Clone + Sync + Send {
    fn send(&self, gate_id: usize, tick: u128, state: super::Signal);
}
