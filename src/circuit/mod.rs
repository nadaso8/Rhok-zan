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
    ticks_per_input: usize,
    tick_counter: usize
}

impl Circuit {
fn tick(&mut self) {
        // update self.signals_swap with pending signal values 
        self.signals_swap.par_iter_mut().enumerate().for_each(
            |(index, swap)|
            match &self.description[index] {
                // I/O port handling 
                Operation::Input(input) => {
                    if self.tick_counter % self.ticks_per_input == 0 {
                        let pending_input = input.handler.as_ref()(index, self.tick_counter);

                        // This match statment exists to inject an uncontrolled vlaue on the leading edge
                        // of a gate transition. It's necesarry to do this as a test for uncontrolled loops
                        // in the circuit. If such a loop  exists then the uncontrolled value should
                        // dominate and thus propagate throughout the feedback path producing a stable
                        // uncontrolled output. Please run "test_case_latch" for a practical example.
                        *swap = match (pending_input, self.signals[index] ){
                            (Signal::True,Signal::False) => Signal::UncontrolledTrue,
                            (Signal::False,Signal::True) => Signal::UncontrolledFalse,
                            (_,_) => pending_input
                        };
                    } else {
                        *swap = self.signals[index];
                    }
                },
                Operation::Output(a, output) => {
                    *swap = self.signals[a.0];
                    output.handler.as_ref()(index, self.tick_counter, *swap); // user writes function to handle the resulting value
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

        // increment tick counter
        self.tick_counter += 1;
    }

    fn new(description: Box<[Operation]>, tpi: usize) -> Self {
        let description_length = description.as_ref().len();
        // allocate and fill vector for initial state
        let mut initial_state = Vec::with_capacity(description_length);
        for _ in 0..description_length {
            initial_state.push(Signal::Undefined);
        }
        assert_eq!(description_length, initial_state.len());

        // allocate and fill vector for swap
        let mut initial_swap = Vec::with_capacity(description_length);
        for _ in 0..description_length {
            initial_swap.push(Signal::Undefined);
        }
        assert_eq!(description_length, initial_swap.len());

        return Circuit {
            description: description,
            signals: initial_state.into_boxed_slice(),
            signals_swap:initial_swap.into_boxed_slice(),
            ticks_per_input: tpi,
            tick_counter: 0
        };
    }

    fn log_output_err(&self, op_id: usize, user_handler_err: &str) {
        todo!("implement logging and graceful shutdown")
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_case_latch() {
        use operation::*;
        use signal::*;

        const TPI:usize = 16;

        // build description 
        let description = Box::new([
            Operation::Input(
                InputHandler { handler: Box::new(
                    |index, tick|
                    match (tick / TPI) % (1 << 1) {
                        0 => Signal::True,
                        _ => Signal::False
                    }
                )}
            ),
            Operation::Input(
                InputHandler { handler: Box::new(
                    |index, tick|
                    match (tick / TPI) % (1 << 2) {
                        0 => Signal::True,
                        1 => Signal::True,
                        _ => Signal::False
                    })
                }
            ),
            Operation::Nor(SignalID(0), SignalID(3)),
            Operation::Nor(SignalID(1), SignalID(2)),
            Operation::Output(SignalID(2), OutputHandler { handler: Box::new(
                |index, tick, signal| println!("Index: {} is {} on Tick: {}", index, signal, tick)
            ) }),
            Operation::Output(SignalID(3), OutputHandler { handler: Box::new(
                |index, tick, signal| println!("Index: {} is {} on Tick: {}", index, signal, tick)
            ) })
        ]);

        let mut circuit = Circuit::new(description, TPI);
        for _ in 0..128 {
            circuit.tick();
        }

    }
}