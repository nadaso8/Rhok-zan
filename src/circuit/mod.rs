mod operation;
mod signal;

use rayon::prelude::*;

use self::operation::*;
use self::signal::*;

#[derive(Debug)]
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
                Operation::Input(input) => {
                    let pending_input = input.handler.as_ref()(index);

                    // This match statment exists to inject an uncontrolled vlaue on the leading edge
                    // of a gate transition. It's necesarry to do this as a test for uncontrolled loops
                    // in the circuit. If such a loop  exists then the uncontrolled value should
                    // dominate and thus propagate throughout the feedback path producing a stable
                    // uncontrolled output. Please run "test_case_latch" for a practical example.
                    match (pending_input, self.signals[index] ){
                        (Signal::True,Signal::False) => Signal::UncontrolledTrue,
                        (Signal::False,Signal::True) => Signal::UncontrolledFalse,
                        (_,_) => pending_input
                    };
                },
                Operation::Output(a, output) => {
                    *swap = self.signals[a.0];
                    output.handler.as_ref()(index, *swap); // user writes function to handle the resulting value
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
    use std::ops::Index;

    use super::*;

    fn test_case_latch() {
        use operation::*;
        use signal::*;
        
        let input_fn = 
        |frequency: usize, call_counter: Box<usize>, state: Box<Signal>| -> Signal {
            if *call_counter % frequency == 0 {*state = !*state};           
            *call_counter += 1;
            return *state.clone();
        };

        // allocate input variables such that they persist through callings by circuit
        let mut input_states = (Signal::False, Signal::True);
        let mut input_counters = (0,0);

        let description = [
            Operation::Input(
                |_| {input_fn(1, Box::new(0), Box::new(Signal::False))}
            ),
            Operation::Input(|_|
                
            )
        ];
    }
}