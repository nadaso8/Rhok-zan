// This module describes a set of builder class used to constuct a digital circuit graph
use crate::sim::circuit::{operation::*, signal::*};
use std::{u128, usize};

#[derive(Debug)]
pub struct Module {
    desc: Vec<Option<Operation>>,
}

impl Module {
    /// Make an empty Module object
    pub fn new() -> Self {
        Self { desc: Vec::new() }
    }

    /// Assigns a location with type Operation::Input<InputHandler<expr>> then returns it's SignalID
    pub fn mk_input(
        &mut self,
        loc: SignalID,
        expr: impl Fn(usize, u128) -> Signal + Sync + Send + 'static,
    ) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Input(InputHandler::new(Box::new(expr))));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_output(
        &mut self,
        loc: SignalID,
        input: SignalID,
        expr: impl Fn(usize, u128, Signal) + Sync + Send + 'static,
    ) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op =
                        Option::Some(Operation::Output(input, OutputHandler::new(Box::new(expr))));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_and(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::And(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_nand(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Nand(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_or(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Or(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_nor(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Nor(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_xor(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Xor(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_xnor(&mut self, loc: SignalID, input: (SignalID, SignalID)) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Xnor(input.0, input.1));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    pub fn mk_not(&mut self, loc: SignalID, input: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Not(input));
                    Result::Ok(())
                }
                Option::Some(_) => Result::Err(
                    "Cannot preform multiple assignments at the same location".to_string(),
                ),
            }
        } else {
            return Result::Err(
                "Cannot assign to a location which has not been allocated".to_string(),
            );
        }
    }

    /// Consumes self and converts it to type Box<[Operation]> replacing any Option::None
    /// allocated locations with Opperation::Input<InputHandler<||Signal::HighImpedance>>.
    pub fn into_desc(self) -> Box<[Operation]> {
        let mut tmp = Vec::new();
        for entry in self.desc {
            match entry {
                Some(op) => tmp.push(op),
                None => tmp.push(Operation::Input(InputHandler::new(Box::new(|_, _| {
                    Signal::HighImpedance
                })))),
            }
        }
        tmp.into_boxed_slice()
    }

    /// Allocates a location as none and returns allocated location's SignalID
    pub fn rz_alloc(&mut self) -> SignalID {
        self.desc.push(Option::None);
        SignalID(self.desc.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sim::circuit::Circuit;

    #[test]
    fn test_case_latch() {
        const TPI: usize = 8;
        const ALWAYS_PRINT: bool = true;
        const PRINT_Q: bool = true;
        const PRINT_Q_NOT: bool = true;

        let mut latch = Module::new();

        let (S, R) = (latch.rz_alloc(), latch.rz_alloc());
        latch
            .mk_input(S, |_index, tick| match (tick / (TPI as u128 * 2)) % (2) {
                0 => Signal::False,
                _ => Signal::True,
            })
            .unwrap();
        latch
            .mk_input(R, |_index, tick| match (tick / (TPI as u128 * 4)) % (2) {
                0 => Signal::False,
                _ => Signal::True,
            })
            .unwrap();

        let (Q, Q_not) = (latch.rz_alloc(), latch.rz_alloc());
        latch.mk_nor(Q, (R, Q_not)).unwrap();
        latch.mk_nor(Q_not, (S, Q)).unwrap();

        let outputs = (latch.rz_alloc(), latch.rz_alloc());
        latch
            .mk_output(outputs.0, Q, |index, tick, signal| {
                let should_print = tick % TPI as u128;
                if (should_print == 0 || ALWAYS_PRINT) && PRINT_Q {
                    println!("Index: {} is {} on Tick: {}", index, signal, tick)
                };
                return;
            })
            .unwrap();
        latch
            .mk_output(outputs.1, Q_not, |index, tick, signal| {
                let should_print = tick % TPI as u128;
                if (should_print == 0 || ALWAYS_PRINT) && PRINT_Q_NOT {
                    println!("Index: {} is {} on Tick: {}", index, signal, tick)
                };
                return;
            })
            .unwrap();

        let mut circuit = Circuit::new(latch.into_desc(), TPI);
        for _ in 0..=256 {
            circuit.tick();
        }
        println!("Q -> {}, !Q -> {}", outputs.0 .0, outputs.1 .0);
    }
}
