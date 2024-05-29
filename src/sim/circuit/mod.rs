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
    tick_counter: u128
}

impl Circuit {
    /// Steps the circuit simulation forward one tick.
    pub fn tick(&mut self) {
        // update self.signals_swap with pending signal values 
        self.signals_swap.par_iter_mut().enumerate().for_each(
            |(index, swap)|
            match &self.description[index] {
                // I/O port handling 
                Operation::Input(input) => {
                    if self.tick_counter % self.ticks_per_input as u128 == 0 {
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

    /// Produces a new circuit sim object from an opperation graph structure.
    /// 
    /// TPI indicates the ticks per input. AKA, how manny simulation ticks 
    /// will be run in between calling input node closures. It is HIGHLY
    /// recommended that this value is longer than the propagation delay
    /// of the circuit under testing.
    pub fn new(description: Box<[Operation]>, tpi: usize) -> Self {
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

    /// Gets the current state of the simulation and its description
    /// useful if you need to inspect a snapshot of the full simulation state.
    /// For regular outputs you should prefer the usage of output Opperation closures
    /// since they will be called in paralell during the simulation loop.
    pub fn inspect(&self) -> (&[Operation], &[Signal]) {
        (self.description.as_ref(), self.signals.as_ref())
    }

    /// Gets the current tick of the simulation.
    pub fn get_tick(&self) -> u128 {
        self.tick_counter
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
                    match (tick / (TPI as u128 * 2)) % (2) {
                        0 => Signal::False,
                        _ => Signal::True
                    }
                )}
            ),
            Operation::Input(
                InputHandler { handler: Box::new(
                    |index, tick|
                    match (tick / (TPI as u128 * 4)) % (2) {
                        0 => Signal::False,
                        _ => Signal::True
                    })
                }
            ),
            Operation::Nor(SignalID(0), SignalID(3)),
            Operation::Nor(SignalID(1), SignalID(2)),
            Operation::Output(SignalID(2), OutputHandler { handler: Box::new(
                |index, tick, signal| {
                    if tick % TPI as u128 == 0 {println!("Index: {} is {} on Tick: {}", index, signal, tick)};
                    return;
                }
            ) }),
            Operation::Output(SignalID(3), OutputHandler { handler: Box::new(
                |index, tick, signal| {
                    if tick % TPI as u128 == 0 {println!("Index: {} is {} on Tick: {}", index, signal, tick)};
                    return;
                }
            ) })
        ]);

        let mut circuit = Circuit::new(description, TPI);
        for _ in 0..=256 {
            circuit.tick();
        }

    }
}