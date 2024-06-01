// This module stores various constructor functions for implementations of common circuit elements such as latches, flipflops 
// or other verilog primitives which are in actuality multi node graph constructs.

use super::rust::*;
use crate::sim::circuit::operation::SignalID;

/// Constructs a latch in module taking nodes set and reset as it's inputsl. The returned tuple is formatted as: (Q, !Q)
fn latch(module: &mut RzModule, set: SignalID, rst: SignalID) -> (SignalID, SignalID) {
    let (q, q_not) = (module.rz_alloc(), module.rz_alloc());
    module.mk_nor(q, (rst,q_not)).unwrap();
    module.mk_nor(q_not, (set,q)).unwrap();
    return (q, q_not);
}

fn flipflop(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: SignalID, ce: Option<SignalID>) -> SignalID {
    if let Some(id) = ce {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    } else {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }
}

fn register() {
    unimplemented!("I make no guarantees as to if I will support clocked components")
}

fn complimenter() {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn full_adder() {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn ripple_adder() {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn multiplexer() {
    unimplemented!("I make no guarantees as to if I will support interpreting of non boolean logical operations")
}

fn variable_shift() {
    unimplemented!("I make no guarantees as to if I will support interpreting of non boolean logical operations")
}

fn shift_multiplier() {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn shift_divider() {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}