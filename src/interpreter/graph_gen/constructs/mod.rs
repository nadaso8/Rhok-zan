// This module stores various constructor functions for implementations of common circuit elements such as latches, flipflops 
// or other verilog primitives which are in actuality multi node graph constructs.

use super::rust::*;
use crate::sim::circuit::operation::SignalID;

/// Constructs a latch in module taking nodes set and reset as it's inputsl.
/// The returned tuple is formatted as: (Q, !Q)
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

fn register(module: &mut RzModule, clk: Option<SignalID>, rst: SignalID, dat: &[SignalID], ce: Option<SignalID>) -> Box<[SignalID]> {
    unimplemented!("I make no guarantees as to if I will support clocked components")
}

fn compliment(module: &mut RzModule, dat: &[SignalID]) -> Box<[SignalID]> {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

/// Constructs a fulladder implementing x + y = z
/// where the output tuple is formatted as (z, carry_out)
fn full_adder(module: &mut RzModule, x:SignalID, y:SignalID, carry_in: SignalID) -> (SignalID, SignalID) {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn ripple_add(module: &mut RzModule) {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn multiplex(module: &mut RzModule) {
    unimplemented!("I make no guarantees as to if I will support interpreting of non boolean logical operations")
}

fn variable_shift(module: &mut RzModule) {
    unimplemented!("I make no guarantees as to if I will support interpreting of non boolean logical operations")
}

fn shift_multiply(module: &mut RzModule) {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}

fn shift_divide(module: &mut RzModule) {
    unimplemented!("I make no guarantees as to if I will support interpreting of arithmatic operations")
}