/// a collection of signals packaged together for easy group routing
#[derive(Debug, Clone, Copy)]
pub struct SignalWidth(Vec<Signal>);

/// different signal states produced by a gate or circuit
#[derive(Debug, Clone, Copy)]
pub enum Signal {
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
