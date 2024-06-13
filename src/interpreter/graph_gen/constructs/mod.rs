// This module stores various constructor functions for implementations of common circuit elements such as latches, flipflops 
// or other verilog primitives which are in actuality multi node graph constructs.

use super::rust::*;
use crate::sim::circuit::operation::SignalID;

/// Constructs a latch in module taking nodes set and reset as it's inputsl.
/// The returned tuple is formatted as: (Q, Qbar)
fn latch(module: &mut RzModule, set: SignalID, rst: SignalID) -> Result<(SignalID, SignalID), String> {
    let (q, qbar) = (module.rz_alloc(), module.rz_alloc());
    if let Err(msg) = module.mk_nor(q, (rst,qbar)) {
        return Result::Err(msg);
    }
    if let Err(msg) = module.mk_nor(qbar, (set,q)) {
        return Result::Err(msg);
    }

    return Result::Ok((q, qbar));
}

/// Constructs a D Latch with enable pin.
/// The returned tuple is formatted as: (Q, Qbar)
fn d_latch(module: &mut RzModule, dat: SignalID, en: SignalID) -> Result<(SignalID, SignalID), String> {
    let dat_bar = module.rz_alloc();
    let en_gate = (module.rz_alloc(), module.rz_alloc());
    
    if let Err(msg) = module.mk_not(dat_bar, dat) {
        return Result::Err(msg);
    }

    if let Err(msg) = module.mk_and(en_gate.0, (en, dat)) {
        return Result::Err(msg);
    }

    if let Err(msg) = module.mk_and(en_gate.1, (en, dat_bar)) {
        return Result::Err(msg);
    }

    return match latch(module, en_gate.0, en_gate.1) {
        Err(msg) => Result::Err(msg),
        Ok(output) => Result::Ok(output)
    }
}

/// Constructs a negative edge triggered flipflop with asyncronous clear
/// The returned tuple is formatted as: (Q, Qbar)
fn ne_d_flipflop(module: &mut RzModule, clk: SignalID, dat: SignalID) -> Result<(SignalID, SignalID), String> {
    unimplemented!("I make no guarantees as to if I will support clocked components")
}

fn register(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: &[SignalID], ce: Option<SignalID>) -> Box<[SignalID]> {
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