// This describes the parsing and output tokenstream of an rz_asm file.
// For rz_asm docs please see the rz_asm.md in the root directory of this repository.

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "interpreter/hdl_tokenizer/rz_asm/rz_asm.pest"]
pub struct RzAsmParser;