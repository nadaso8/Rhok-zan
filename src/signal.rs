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
            Signal::False => Signal::True,
            Signal::True => Signal::False,
            Signal::UncontrolledFalse => Signal::UncontrolledTrue,
            Signal::UncontrolledTrue => Signal::UncontrolledFalse,
            Signal::Undefined => Signal::Undefined,
            Signal::HighImpedance => Signal::HighImpedance,
        }
    }
}

impl BitAnd for Signal {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::True, Signal::True) => Signal::True,
            (Signal::False, _) |
            (_, Signal::False) => Signal::False,
            (Signal::UncontrolledTrue, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal)
    
        }
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
