use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interpreter/hdl_tokenizer/vhdl/vhdl.pest"]
pub struct RzVhdlParser;