// This module describes a set of builder functions which produce a circuit graph from native rust code

use egui::debug_text;

use crate::sim::circuit;
use super::super::super::sim::circuit::{
    operation::*,
    signal::*
};
use std::{collections::HashMap, u128, usize};

#[derive(Debug, PartialEq, Eq)]
pub struct VarName(String);

#[derive(Debug)]
pub struct RzModule {
    desc: Vec<Operation>,
    bind: HashMap<VarName, SignalID>
}

impl RzModule {

    /// Make an empty RzModule object
    pub fn new() -> Self {
        Self{
            desc: Vec::new(),
            bind: HashMap::new()
        }
    }
    /// Binds a variable name to the first unused signal ID 
    pub fn bind(&mut self, name: &str) {
        todo!()
    }

    /// Assigns the SignalID looked up from name with a computation produced by op
    pub fn assign(&mut self, name: &str, expr: impl Fn(&mut Self, SignalID)) -> Result<(), String>{
        todo!()
    }

    pub fn mk_input(&mut self, loc: SignalID, handler: impl Fn(usize, u128) -> Signal + Sync + Send) {
        todo!()
    }

    pub fn mk_output(&mut self, loc: SignalID, handler: impl Fn(usize, u128, Signal) + Sync + Send) {
        todo!()
    }

    pub fn mk_and(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_nand(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_or(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_nor(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_xor(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_xnor(&mut self, loc: SignalID, a: String, b: String) {
        todo!()
    }

    pub fn mk_not(&mut self, loc: SignalID, a: String) {
        todo!()
    }

    pub fn into_desc(self) -> Box<[Operation]> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_latch() {
        let mut latch = RzModule::new();
        latch.bind(&"Q");
        latch.bind(&"Q'");
        latch.bind(&"S");
        latch.bind(&"R");
        latch.assign(&"Q",
            |latch, var_id| {
                
            }
        ).unwrap()

    }
}