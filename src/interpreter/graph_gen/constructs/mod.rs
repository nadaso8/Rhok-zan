// This module stores various constructor functions for implementations of common circuit elements such as latches, flipflops 
// or other verilog primitives which are in actuality multi node graph constructs.

use eframe::glow::FALSE;

use super::rust::*;
use crate::sim::circuit::operation::SignalID;

/// Constructs a latch in module taking nodes set and reset as it's inputsl.
/// The returned tuple is formatted as: (Q, !Q)
fn latch(module: &mut RzModule, set: SignalID, rst: SignalID) -> Result<(SignalID, SignalID), String> {
    let (q, q_not) = (module.rz_alloc(), module.rz_alloc());
    if let Err(msg) = module.mk_nor(q, (rst,q_not)) {
        return Result::Err(msg);
    }
    if let Err(msg) = module.mk_nor(q_not, (set,q)) {
        return Result::Err(msg);
    }

    return Result::Ok((q, q_not));
}

fn flipflop(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: SignalID, clk_edge: Option<bool>, rst_edge: Option<bool>) -> Result<SignalID, String> {
    match (clk_edge, rst_edge) {
        // non edge triggered clock sync reset
        (None, None) => {

        },
        
        // non edge triggered clock posedge reset
        (None, Some(true)) => {

        },

        // non edge triggered clock posedge reset
        (None, Some(false)) => {

        },

        // posedge clk sync reset
        (Some(true),None) => {

        },

        // negedte clk sync reset
        (Some(false),None) => {

        },

        // posedge clk posedge reset
        (Some(true), Some(true)) => {

        },

        // invalid verilog
        (Some(false), Some(true)) => {
            Result::Err("Differing edge triggers in a single always block are unsupported".to_string())
        },

        // invalid verilog 
        (Some(true), Some(false)) => {
            Result::Err("Differing edge triggers in a single always block are unsupported".to_string())
        },

        // negedge clk negedge reset
        (Some(false), Some(false)) => {
            
        },
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