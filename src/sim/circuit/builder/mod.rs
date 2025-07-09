// This module describes the builder class used to constuct a digital circuit graph
// it was adapted to use petraph and uses it kinda weirdly as a result
// eventually this needs a full re-implementation.

use crate::sim::circuit::{operation::*, signal::*};
use std::{sync::Arc, u128, usize};

#[derive(Debug)]
pub struct GateLevelDescription {
    pub desc: Vec<Option<Operation>>,
}

impl GateLevelDescription {
    /// Make an empty Module object
    pub fn new() -> Self {
        Self { desc: Vec::new() }
    }

    /// Assigns a location with type Operation::Input<InputHandler<expr>> then returns it's SignalID
    pub fn mk_input(
        &mut self,
        loc: SignalID,
        expr: Arc<dyn Fn(usize, u128) -> Signal + Sync + Send>,
    ) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Input(InputHandler::new(expr)));
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
        var: SignalID,
        expr: Arc<dyn Fn(usize, u128, Signal) + Sync + Send>,
    ) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Output(var, OutputHandler::new(expr)));
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

    // the interface to all of these is bad input tuple wtf... it should be an lhs and rhs arg which get used directly
    pub fn mk_and(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::And(lhs, rhs));
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

    pub fn mk_nand(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Nand(lhs, rhs));
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

    pub fn mk_or(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Or(lhs, rhs));
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

    pub fn mk_nor(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Nor(lhs, rhs));
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

    pub fn mk_xor(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Xor(lhs, rhs));
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

    pub fn mk_xnor(&mut self, loc: SignalID, lhs: SignalID, rhs: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Xnor(lhs, rhs));
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

    pub fn mk_not(&mut self, loc: SignalID, var: SignalID) -> Result<(), String> {
        if let Some(op) = self.desc.get_mut(loc.0) {
            match op {
                Option::None => {
                    *op = Option::Some(Operation::Not(var));
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

    /// references self and converts it to type Box<[Operation]>' replacing any Option::None
    /// allocated locations with Opperation::Input<InputHandler<||Signal::HighImpedance>>.
    pub fn into_desc(&self) -> Box<[Operation]> {
        let mut tmp = Vec::new();
        for entry in &self.desc {
            match entry {
                Some(op) => tmp.push(op.clone()),
                None => tmp.push(Operation::Input(InputHandler::new(Arc::new(|_, _| {
                    Signal::HighImpedance
                })))),
            }
        }
        tmp.into_boxed_slice()
    }

    /// Allocates a location as none and returns allocated location's SignalID
    pub fn rz_alloc(&mut self) -> SignalID {
        let id = SignalID(self.desc.len());
        self.desc.push(Option::None);
        id
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
        const PRINT_Q_NOT: bool = false;

        let mut latch = GateLevelDescription::new();

        let (s, r) = (latch.rz_alloc(), latch.rz_alloc());
        latch
            .mk_input(
                s,
                Arc::new(|_index, tick| match (tick / (TPI as u128 * 2)) % (2) {
                    0 => Signal::False,
                    _ => Signal::True,
                }),
            )
            .unwrap();
        latch
            .mk_input(
                r,
                Arc::new(|_index, tick| match (tick / (TPI as u128 * 4)) % (2) {
                    0 => Signal::False,
                    _ => Signal::True,
                }),
            )
            .unwrap();

        let (q, q_not) = (latch.rz_alloc(), latch.rz_alloc());
        latch.mk_nor(q, r, q_not).unwrap();
        latch.mk_nor(q_not, s, q).unwrap();

        let outputs = (latch.rz_alloc(), latch.rz_alloc());
        latch
            .mk_output(
                outputs.0,
                q,
                Arc::new(|index, tick, signal| {
                    let should_print = tick % TPI as u128;
                    if (should_print == 0 || ALWAYS_PRINT) && PRINT_Q {
                        println!("Index: {} is {} on Tick: {}", index, signal, tick)
                    };
                    return;
                }),
            )
            .unwrap();
        latch
            .mk_output(
                outputs.1,
                q_not,
                Arc::new(|index, tick, signal| {
                    let should_print = tick % TPI as u128;
                    if (should_print == 0 || ALWAYS_PRINT) && PRINT_Q_NOT {
                        println!("Index: {} is {} on Tick: {}", index, signal, tick)
                    };
                    return;
                }),
            )
            .unwrap();

        let mut circuit = Circuit::new(latch.into_desc(), TPI);
        for _ in 0..=256 {
            circuit.tick();
        }
        println!("Q -> {}, !Q -> {}", outputs.0 .0, outputs.1 .0);
    }
}
