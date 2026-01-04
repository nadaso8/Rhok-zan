pub mod builder;
pub mod operation;
pub mod signal;

use rayon::prelude::*;

use self::operation::*;
use self::signal::*;

#[derive(Debug)]
pub struct Circuit<I, O>
where
    I: CircuitInput,
    O: CircuitOutput,
{
    description: Box<[Operation<I, O>]>,
    signals: Box<[Signal]>,
    signals_swap: Box<[Signal]>,
    ticks_per_input: usize,
    tick_counter: u128,
}

impl<I, O> Circuit<I, O>
where
    I: CircuitInput,
    O: CircuitOutput,
{
    /// Steps the circuit simulation forward one tick.
    pub fn tick(&mut self) {
        // update self.signals_swap with pending signal values
        self.signals_swap
            .par_iter_mut()
            .enumerate()
            .for_each(|(index, swap)| match &self.description[index] {
                // I/O port handling
                Operation::Input(input) => {
                    if self.tick_counter % self.ticks_per_input as u128 == 0 {
                        let pending_input = input.recieve(
                            index,
                            /*
                            The division here is needed in order to split the Input tick space and Simulation tick space.
                            If this was not present input closures would have to be aware of the depth of the circuit
                            calling them or risk samping issues. The circuit may only call the input closure every TPI
                            ticks so that uncontrolled singals are allowed time to propagate through the circuit.

                            For example if an input changed between true and false with a period of TPI the value would
                            appear constant true to the circuit as the closure woulldn't be called when the value was
                            false.
                            */
                            self.tick_counter / self.ticks_per_input as u128,
                        );

                        // This match statment exists to inject an uncontrolled vlaue on the leading edge
                        // of a gate transition. It's necesarry to do this as a test for uncontrolled loops
                        // in the circuit. If such a loop  exists then the uncontrolled value should
                        // dominate and thus propagate throughout the feedback path producing a stable
                        // uncontrolled output. Please run "test_case_latch" for a practical example.
                        *swap = match (pending_input, self.signals[index]) {
                            (Signal::True, Signal::False) => Signal::UncontrolledTrue,
                            (Signal::False, Signal::True) => Signal::UncontrolledFalse,
                            (_, _) => pending_input,
                        };
                    } else {
                        *swap = self.signals[index];
                    }
                }
                Operation::Output(a, output) => {
                    *swap = self.signals[a.0];
                    output.send(index, self.tick_counter, *swap);
                }

                // standard boolean logic handling
                Operation::Not(a) => *swap = !self.signals[a.0],
                Operation::And(a, b) => *swap = self.signals[a.0] & self.signals[b.0],
                Operation::Nand(a, b) => *swap = !(self.signals[a.0] & self.signals[b.0]),
                Operation::Or(a, b) => *swap = self.signals[a.0] | self.signals[b.0],
                Operation::Nor(a, b) => *swap = !(self.signals[a.0] | self.signals[b.0]),
                Operation::Xor(a, b) => *swap = self.signals[a.0] ^ self.signals[b.0],
                Operation::Xnor(a, b) => *swap = !(self.signals[a.0] ^ self.signals[b.0]),
            });

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
    pub fn new(description: Box<[Operation<I, O>]>, tpi: usize) -> Self {
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
            description,
            signals: initial_state.into_boxed_slice(),
            signals_swap: initial_swap.into_boxed_slice(),
            ticks_per_input: tpi,
            tick_counter: 0,
        };
    }

    /// Gets the current state of the simulation and its description
    /// useful if you need to inspect a snapshot of the full simulation state.
    /// For regular outputs you should prefer the usage of output Opperation closures
    /// since they will be called in paralell during the simulation loop.
    pub fn inspect(&self) -> (&[Operation<I, O>], &[Signal]) {
        (self.description.as_ref(), self.signals.as_ref())
    }

    /// Gets the current tick of the simulation.
    pub fn get_tick(&self) -> u128 {
        self.tick_counter
    }
}

#[cfg(test)]
mod tests {}
