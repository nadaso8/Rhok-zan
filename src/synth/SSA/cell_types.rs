use std::rc::Rc;

use super::{CellType, Module};

struct ModuleInstance {
    module: Rc<Module>,
}

struct AndGate {}
impl CellType for AndGate {
    fn ports(&self) -> Vec<super::Port> {
        todo!()
    }

    fn lower(&self) -> Result<Vec<super::Cell>, super::LowerError> {
        todo!()
    }

    fn is_boundary(&self) -> Option<super::PortHandle> {
        todo!()
    }

    fn is_primitive(&self) -> Option<crate::sim::circuit::operation::Operation> {
        todo!()
    }
}

struct NandGate {}

struct OrGate {}

struct NorGate {}

struct XorGate {}

struct XnorGate {}

struct Inverter {}
