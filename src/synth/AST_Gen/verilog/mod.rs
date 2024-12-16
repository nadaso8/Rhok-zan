/// depricated implementation of verilog parser in pest I am at the moment planning on using the verilog crate
/// and have as such removed pest as a project dependency
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interpreter/hdl_tokenizer/verilog/verilog.pest"]
pub struct RzVerilogParser;