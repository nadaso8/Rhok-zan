// This module stores various constructor functions for implementations of common circuit elements such as latches, flipflops 
// or other verilog primitives which are in actuality multi node graph constructs.

mod memory_elements {
    use crate::interpreter::gld_compiler::builder::*;
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

    /// Constructs a JK Latch with enable pin. 
    /// The returned tuple is formatted as: (Q, Qbar)
    fn jk_latch(module: &mut RzModule, j: SignalID, k: SignalID, en: SignalID) -> Result<(SignalID, SignalID), String> {
        unimplemented!()
    }

    /// Constructs a negative edge triggered flipflop with asyncronous clear and preset
    /// The returned tuple is formatted as: (Q, Qbar)
    fn ne_d_flipflop(module: &mut RzModule, clk: SignalID, dat: SignalID) -> Result<(SignalID, SignalID), String> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }

    /// Constructs a positive edge triggered flipflop with asyncronous clear and preset 
    /// The returned tuple is formatted as: (Q, Qbar)
    fn pe_d_flipflop(module: &mut RzModule, clk: SignalID, dat: SignalID) -> Result<(SignalID, SignalID), String> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }

    /// Constructs a negative edge triggered jk flipflop with asyncronous clear and preset
    /// The returned tuple is formatted as: (Q, Qbar)
    fn ne_jk_flipflop(module: &mut RzModule, clk: SignalID, dat: SignalID) -> Result<(SignalID, SignalID), String> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }

    /// Constructs a positive edge triggered jk flipflop with asyncronous clear
    /// The returned tuple is formatted as: (Q, Qbar)
    fn pe_jk_flipflop(module: &mut RzModule, clk: SignalID, dat: SignalID) -> Result<(SignalID, SignalID), String> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }

    fn pe_reg(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: &[SignalID], ce: Option<SignalID>) -> Box<[SignalID]> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }

    fn ne_reg(module: &mut RzModule, clk: SignalID, rst: SignalID, dat: &[SignalID], ce: Option<SignalID>) -> Box<[SignalID]> {
        unimplemented!("I make no guarantees as to if I will support clocked components")
    }
}

mod logic {
    use crate::interpreter::gld_compiler::builder::*;
    use crate::sim::circuit::operation::SignalID;

    fn comparitor() {
        unimplemented!()    
    }

    fn gt() {
        unimplemented!()
    }

    fn lt() {
        unimplemented!()
    }

    fn ge() {
        unimplemented!()
    }

    fn le() {
        unimplemented!()
    }

    fn eq() {
        unimplemented!()
    }

    fn sorter() {
        unimplemented!()
    }

    fn multiplexor() {
        unimplemented!()
    }
}

mod operators {
    use crate::interpreter::gld_compiler::builder::*;
    use crate::sim::circuit::operation::SignalID;

    fn full_adder() {
        unimplemented!()
    }

    fn add() {
        unimplemented!()
    }

    fn compliment() {
        unimplemented!()
    }

    fn subtract() {
        unimplemented!()
    }

    fn shift() {
        unimplemented!()
    }

    fn divide() {
        unimplemented!()
    }

    fn multiply() {
        unimplemented!()
    }

    fn modulus() {
        unimplemented!()
    }
}