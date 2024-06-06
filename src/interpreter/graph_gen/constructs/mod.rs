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

fn flipflop(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: SignalID, edge_trigger: Option<bool>) -> SignalID {
    todo!("reset functionality not implemented ")
    match edge_trigger {
        Some(true) => {
            // construct inverse signals for clk and dat
            let inv_clk = module.rz_alloc();
            module.mk_not(inv_clk, clk).unwrap();
            let inv_dat = module.rz_alloc();
            module.mk_not(inv_dat, dat).unwrap();

            // construct sub as a latch which is enabled on clk low
            let (sub_set, sub_rst) = (module.rz_alloc(), module.rz_alloc());
            module.mk_and(sub_set, (inv_clk, dat)).unwrap();
            module.mk_and(sub_rst, (inv_clk, inv_dat)).unwrap();
            let (sub_q, sub_q_not) = latch(module, sub_set, sub_rst);

            // construct main as a latch which is enabled on clk high
            let (main_set, main_rst) = (module.rz_alloc(), module.rz_alloc());
            module.mk_and(main_set, (clk, sub_q)).unwrap();
            module.mk_and(main_rst, (clk, sub_q_not)).unwrap();
            let (main_q, _) = latch(module, main_set, main_rst);

            return main_q;
        },
        Some(false) => {
            // construct inverse singals for clk and dat 
            let inv_clk = module.rz_alloc();
            module.mk_not(inv_clk, clk).unwrap();
            let inv_dat = module.rz_alloc();
            module.mk_not(inv_dat, dat).unwrap();

            // construct sub as a latch which is enabled on clk high
            let (sub_set, sub_rst) = (module.rz_alloc(), module.rz_alloc());
            module.mk_and(sub_set, (clk, dat)).unwrap();
            module.mk_and(sub_rst, (clk, inv_dat)).unwrap();
            let (sub_q, sub_q_not) = latch(module, sub_set, sub_rst);
            
            // construct main as a latch which is enabled on clk low
            let (main_set, main_rst) = (module.rz_alloc(), module.rz_alloc());
            module.mk_and(main_set, (inv_clk, sub_q)).unwrap();
            module.mk_and(main_rst, (inv_clk, sub_q_not)).unwrap();
            let (main_q,_) = latch(module, main_set, main_rst);

            return main_q;
        }, 
        None => {
            let inv_dat = module.rz_alloc();
            module.mk_not(inv_dat, dat).unwrap();

            let (set, rst) = (module.rz_alloc(),module.rz_alloc());
            module.mk_and(set, (clk, dat)).unwrap();
            module.mk_and(rst, (clk, inv_dat)).unwrap();
            let (main_q,_) = latch(module, set, rst);

            return main_q;
        }
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