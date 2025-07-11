/// different signal states produced by a gate or circuit
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Signal {
    False,
    True,
    UncontrolledFalse,
    UncontrolledTrue,
    HighImpedance,
    Undefined,
}

impl std::ops::Not for Signal {
    type Output = Self;
    fn not(self) -> Signal {
        match self {
            Signal::False => Signal::True,
            Signal::True => Signal::False,
            Signal::UncontrolledFalse => Signal::UncontrolledTrue,
            Signal::UncontrolledTrue => Signal::UncontrolledFalse,
            Signal::HighImpedance => Signal::HighImpedance,
            Signal::Undefined => Signal::Undefined,
        }
    }
}

impl std::ops::BitAnd for Signal {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::False, Signal::False) => Signal::False,
            (Signal::False, Signal::True) => Signal::False,
            (Signal::False, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::False, Signal::UncontrolledTrue) => Signal::False,
            (Signal::False, Signal::HighImpedance) => Signal::False,
            (Signal::False, Signal::Undefined) => Signal::False,

            (Signal::True, Signal::False) => Signal::False,
            (Signal::True, Signal::True) => Signal::True,
            (Signal::True, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::True, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::True, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::True, Signal::Undefined) => Signal::Undefined,

            (Signal::UncontrolledFalse, Signal::False) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::True) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::UncontrolledTrue) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::HighImpedance) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::Undefined) => Signal::UncontrolledFalse,

            (Signal::UncontrolledTrue, Signal::False) => Signal::False,
            (Signal::UncontrolledTrue, Signal::True) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::UncontrolledTrue, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::UncontrolledTrue, Signal::Undefined) => Signal::Undefined,

            (Signal::HighImpedance, Signal::False) => Signal::False,
            (Signal::HighImpedance, Signal::True) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::HighImpedance, Signal::UncontrolledTrue) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::Undefined) => Signal::HighImpedance,

            (Signal::Undefined, Signal::False) => Signal::False,
            (Signal::Undefined, Signal::True) => Signal::Undefined,
            (Signal::Undefined, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::Undefined, Signal::UncontrolledTrue) => Signal::Undefined,
            (Signal::Undefined, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::Undefined, Signal::Undefined) => Signal::Undefined,
        }
    }
}

impl std::ops::BitOr for Signal {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::False, Signal::False) => Signal::False,
            (Signal::False, Signal::True) => Signal::True,
            (Signal::False, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::False, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::False, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::False, Signal::Undefined) => Signal::Undefined,

            (Signal::True, Signal::False) => Signal::True,
            (Signal::True, Signal::True) => Signal::True,
            (Signal::True, Signal::UncontrolledFalse) => Signal::True,
            (Signal::True, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::True, Signal::HighImpedance) => Signal::True,
            (Signal::True, Signal::Undefined) => Signal::True,

            (Signal::UncontrolledFalse, Signal::False) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::True) => Signal::True,
            (Signal::UncontrolledFalse, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::UncontrolledFalse, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::UncontrolledFalse, Signal::Undefined) => Signal::Undefined,

            (Signal::UncontrolledTrue, Signal::False) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::True) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::UncontrolledFalse) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::HighImpedance) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::Undefined) => Signal::UncontrolledTrue,

            (Signal::HighImpedance, Signal::False) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::True) => Signal::True,
            (Signal::HighImpedance, Signal::UncontrolledFalse) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::HighImpedance, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::Undefined) => Signal::HighImpedance,

            (Signal::Undefined, Signal::False) => Signal::Undefined,
            (Signal::Undefined, Signal::True) => Signal::True,
            (Signal::Undefined, Signal::UncontrolledFalse) => Signal::Undefined,
            (Signal::Undefined, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::Undefined, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::Undefined, Signal::Undefined) => Signal::Undefined,
        }
    }
}

impl std::ops::BitXor for Signal {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Signal::False, Signal::False) => Signal::False,
            (Signal::False, Signal::True) => Signal::True,
            (Signal::False, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::False, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::False, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::False, Signal::Undefined) => Signal::Undefined,

            (Signal::True, Signal::False) => Signal::True,
            (Signal::True, Signal::True) => Signal::False,
            (Signal::True, Signal::UncontrolledFalse) => Signal::UncontrolledTrue,
            (Signal::True, Signal::UncontrolledTrue) => Signal::UncontrolledFalse,
            (Signal::True, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::True, Signal::Undefined) => Signal::Undefined,

            (Signal::UncontrolledFalse, Signal::False) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::True) => Signal::UncontrolledTrue,
            (Signal::UncontrolledFalse, Signal::UncontrolledFalse) => Signal::UncontrolledFalse,
            (Signal::UncontrolledFalse, Signal::UncontrolledTrue) => Signal::UncontrolledTrue,
            (Signal::UncontrolledFalse, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::UncontrolledFalse, Signal::Undefined) => Signal::Undefined,

            (Signal::UncontrolledTrue, Signal::False) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::True) => Signal::UncontrolledFalse,
            (Signal::UncontrolledTrue, Signal::UncontrolledFalse) => Signal::UncontrolledTrue,
            (Signal::UncontrolledTrue, Signal::UncontrolledTrue) => Signal::UncontrolledFalse,
            (Signal::UncontrolledTrue, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::UncontrolledTrue, Signal::Undefined) => Signal::Undefined,

            (Signal::HighImpedance, Signal::False) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::True) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::UncontrolledFalse) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::UncontrolledTrue) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::HighImpedance, Signal::Undefined) => Signal::HighImpedance,

            (Signal::Undefined, Signal::False) => Signal::Undefined,
            (Signal::Undefined, Signal::True) => Signal::Undefined,
            (Signal::Undefined, Signal::UncontrolledFalse) => Signal::Undefined,
            (Signal::Undefined, Signal::UncontrolledTrue) => Signal::Undefined,
            (Signal::Undefined, Signal::HighImpedance) => Signal::HighImpedance,
            (Signal::Undefined, Signal::Undefined) => Signal::Undefined,
        }
    }
}

use std::fmt::Display;

impl Display for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::False => "False",
                Self::True => "True",
                Self::UncontrolledFalse => "Uncontrolled False",
                Self::UncontrolledTrue => "Uncontrolled True",
                Self::Undefined => "Undefined",
                Self::HighImpedance => "High Impedance",
            }
        )
    }
}
