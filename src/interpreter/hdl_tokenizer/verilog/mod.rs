use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interpreter/hdl_tokenizer/verilog/verilog.pest"]
pub struct RzVerilogParser;