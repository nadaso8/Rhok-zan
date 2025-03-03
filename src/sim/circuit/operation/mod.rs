use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct SignalID(pub usize);

pub enum Operation {
    Input(InputHandler<dyn Fn(usize, u128) -> super::Signal + Sync + Send>),
    Output(
        SignalID,
        OutputHandler<dyn Fn(usize, u128, super::Signal) + Sync + Send>,
    ),
    Not(SignalID),
    And(SignalID, SignalID),
    Nand(SignalID, SignalID),
    Or(SignalID, SignalID),
    Nor(SignalID, SignalID),
    Xor(SignalID, SignalID),
    Xnor(SignalID, SignalID),
}

impl Clone for Operation {
    fn clone(&self) -> Self {
        match self {
            Self::Input(InputHandler { handler }) => {
                Self::Input(InputHandler::new(handler.clone()))
            }
            Self::Output(var, OutputHandler { handler }) => {
                Self::Output(var.clone(), OutputHandler::new(handler.clone()))
            }
            Self::Not(var) => Self::Not(var.clone()),
            Self::And(lhs, rhs) => Self::And(lhs.clone(), rhs.clone()),
            Self::Nand(lhs, rhs) => Self::Nand(lhs.clone(), rhs.clone()),
            Self::Or(lhs, rhs) => Self::Or(lhs.clone(), rhs.clone()),
            Self::Nor(lhs, rhs) => Self::Nor(lhs.clone(), rhs.clone()),
            Self::Xor(lhs, rhs) => Self::Xor(lhs.clone(), rhs.clone()),
            Self::Xnor(lhs, rhs) => Self::Xnor(lhs.clone(), rhs.clone()),
        }
    }
}

impl Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type: {}",
            match self {
                Self::Input(_) => format!("input",),
                Self::Output(I, _) => format!("output Of: {}", I.0),
                Self::Not(I) => format!("not Of: {}", I.0),
                Self::And(I, J) => format!("and Of: {}, {}", I.0, J.0),
                Self::Nand(I, J) => format!("nand Of: {}, {}", I.0, J.0),
                Self::Or(I, J) => format!("or Of: {}, {}", I.0, J.0),
                Self::Nor(I, J) => format!("nor Of: {}, {}", I.0, J.0),
                Self::Xor(I, J) => format!("xor Of: {}, {}", I.0, J.0),
                Self::Xnor(I, J) => format!("xnor Of: {}, {}", I.0, J.0),
            }
        )
    }
}

#[derive(Debug)]
pub struct InputHandler<F>
where
    F: Fn(usize, u128) -> super::Signal + Sync + Send + ?Sized,
{
    pub handler: Arc<F>,
}

impl<F> InputHandler<F>
where
    F: Fn(usize, u128) -> super::Signal + Sync + Send + ?Sized,
{
    pub fn new(func: Arc<F>) -> Self {
        Self { handler: func }
    }
}

#[derive(Debug)]
pub struct OutputHandler<F>
where
    F: Fn(usize, u128, super::Signal) + Sync + Send + ?Sized,
{
    pub handler: Arc<F>,
}

impl<F> OutputHandler<F>
where
    F: Fn(usize, u128, super::Signal) + Sync + Send + ?Sized,
{
    pub fn new(func: Arc<F>) -> Self {
        Self { handler: func }
    }
}
