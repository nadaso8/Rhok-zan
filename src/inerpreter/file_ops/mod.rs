use crate::sim::circuit::{
    operation::*,
    signal::*
};

/// Write a graph save file
pub fn write_rkz_save(graph: Box<[Operation]>) -> Result<(),String> {
    todo!()
}

/// Validate type and try to open possible save file
fn open_rkz_save(file_name: String) -> Result<std::fs::File, String> {
    todo!()
}

/// check hash and write rkz save file 
pub fn read_rkz_save(file: std::fs::File) -> Result<Box<[Operation]>, String> {
    todo!()
}