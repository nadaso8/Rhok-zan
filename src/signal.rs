/// different signal states produced by a gate or circuit
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Signal {
    False,
    True,
    UncontrolledFalse,
    UncontrolledTrue,
    HighImpedance,
    Undefined
}

impl std::ops::Not for  Signal {
    type Output = Self;
    fn not(self) -> Signal {
        match self  {
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
            (Signal::False, _)                                      |
            (_, Signal::False)                                      => Signal::False,

            (Signal::True, Signal::True)                            => Signal::True,
            
            (Signal::True, Signal::UncontrolledFalse)               |
            (Signal::UncontrolledTrue, Signal::UncontrolledFalse)   |
            (Signal::HighImpedance, Signal::UncontrolledFalse)      |
            (Signal::Undefined, Signal::UncontrolledFalse)          |
            (Signal::UncontrolledFalse, Signal::True)               |
            (Signal::UncontrolledFalse, Signal::UncontrolledTrue)   |
            (Signal::UncontrolledFalse, Signal::HighImpedance)      |
            (Signal::UncontrolledFalse, Signal::Undefined)          |
            (Signal::UncontrolledFalse, Signal::UncontrolledFalse)  => Signal::UncontrolledFalse,

            (Signal::UncontrolledTrue, Signal::True)                |
            (Signal::True, Signal::UncontrolledTrue)                |
            (Signal::UncontrolledTrue, Signal::UncontrolledTrue )   => Signal::UncontrolledTrue,

            (Signal::HighImpedance, Signal::True)                   |
            (Signal::HighImpedance, Signal::UncontrolledTrue)       |
            (Signal::HighImpedance, Signal::Undefined)              |
            (Signal::True, Signal::HighImpedance)                   |
            (Signal::UncontrolledTrue, Signal::HighImpedance)       |
            (Signal::Undefined, Signal::HighImpedance)              |
            (Signal::HighImpedance, Signal::HighImpedance)          => Signal::HighImpedance,

            (Signal::Undefined, Signal::True)                       |
            (Signal::Undefined, Signal::UncontrolledTrue)           |
            (Signal::True, Signal::Undefined)                       |
            (Signal::UncontrolledTrue, Signal::Undefined)           |            
            (Signal::Undefined, Signal::Undefined)                  => Signal::Undefined
        }
    }
}

impl std::ops::BitOr for Signal {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitXor for Signal {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitand_operation() {
        // Iterate over all variants for the left operand
        for &left_operand in &[
            Signal::False,
            Signal::True,
            Signal::UncontrolledFalse,
            Signal::UncontrolledTrue,
            Signal::HighImpedance,
            Signal::Undefined,
        ] {
            // Iterate over all variants for the right operand
            for &right_operand in &[
                Signal::False,
                Signal::True,
                Signal::UncontrolledFalse,
                Signal::UncontrolledTrue,
                Signal::HighImpedance,
                Signal::Undefined,
            ] {
                // Perform the bitwise AND operation
                let result = left_operand & right_operand;

                // Print the output for each combination
                println!("{:?} & {:?} = {:?}", left_operand, right_operand, result);
            }
        }
    }
}
