use std::rc::Rc;

use super::{Cell, Module, ModuleHandle};

struct ModuleInstance {
    instance_name: Box<str>,
    module_handle: ModuleHandle,
}

impl Cell for ModuleInstance {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(ModuleInstance {
            instance_name: self.instance_name.clone(),
            module_handle: self.module_handle.clone(),
        })
    }

    fn contents(&self) -> super::CellContents {
        todo!()
    }

    fn interface(&self) -> &[super::Port] {
        todo!()
    }
}

struct AndGate {}

struct NandGate {}

struct OrGate {}

struct NorGate {}

struct XorGate {}

struct XnorGate {}

struct Inverter {}
