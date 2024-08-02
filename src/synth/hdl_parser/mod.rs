// This module pretains to the production and unification
// of tokenstreams produced by the various accepted Rhok'zan
// source languages.

use pest::iterators::Pairs;

mod verilog;
mod vhdl;
mod rz_asm;

enum SourceTokenStream <'a>{
    Verilog(Pairs<'a, verilog::Rule>),
}