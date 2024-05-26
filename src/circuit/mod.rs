mod operation;
mod signal;

use std::thread::Result;

use rayon::prelude::*;

use self::operation::*;
use self::signal::*;

#[derive(Clone, Debug)]
pub struct Circuit {
    description: Box<[Operation]>,
    signals: Box<[Signal]>,
    signals_swap: Box<[Signal]>,
}

impl Circuit {
fn step(&mut self) {
        // update self.signals_swap with pending signal values 
        self.signals_swap.par_iter_mut().enumerate().for_each(
            |(index, swap)|
            match self.description[index] {
                // I/O port handling 
                Operation::Input(get) => {
                    *swap = get(index)
                },
                Operation::Output(a, handle) => {

                    match handle(index, self.signals[a.0]) {
                        Ok => ,
                        Err(msg) => 
                    }

                },

                // standard boolean logic handling
                Operation::Not(a) => *swap = !self.signals[a.0],
                Operation::And(a, b) => *swap = self.signals[a.0] & self.signals[b.0],
                Operation::Nand(a, b) => *swap = !(self.signals[a.0] & self.signals[b.0]),
                Operation::Or(a, b) => *swap = self.signals[a.0] | self.signals[b.0],
                Operation::Nor(a, b) => *swap = !(self.signals[a.0] | self.signals[b.0]),
                Operation::Xor(a, b) => *swap = self.signals[a.0] ^ self.signals[b.0],
                Operation::Xnor(a, b) => *swap = !(self.signals[a.0] ^ self.signals[b.0])
            }
        );

        // exchange pointers for signals and signals_swap
        std::mem::swap(&mut self.signals, &mut self.signals_swap);
    }

    fn new(description: Box<[Operation]> ) -> Self {
        // allocate and fill vector for initial state
        let mut initial_state = Vec::with_capacity(description.len());
        initial_state.fill_with(||Signal::Undefined);

        // allocate and fill vector for swap
        let mut initial_swap = Vec::with_capacity(description.len());
        initial_swap.fill_with(||Signal::Undefined);

        return Circuit {
            description: description,
            signals: initial_state.into_boxed_slice(),
            signals_swap:initial_swap.into_boxed_slice(),
        };
    }

    fn log_output_err(&self, op_id: usize, user_handler_err: &str) {
        todo!("implement logging and graceful shutdown")
    }
}


#[cfg(test)]
mod tests { 
    use super::*;

    fn test_case_latch() {
        Circuit::new(
            Box::new([

            ])
        )
    }
}