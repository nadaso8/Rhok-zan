use super::*;
use std::{u128, usize};

#[derive(Clone, Debug)]
pub struct ModuleInstance {
    instance_name: String,
    module_handle: ModuleHandle,
}
impl Cell for ModuleInstance {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(self.clone())
    }

    fn contents(&self) -> super::CellContents {
        super::CellContents::UserModule(self.module_handle)
    }

    fn interface(&self) -> CellInterface {
        CellInterface::UserModule(self.module_handle)
    }
}

// primitive gates
#[derive(Clone, Copy, Debug)]
pub struct AndGate {}
impl Cell for AndGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(self.clone())
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::And)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A&B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NandGate {}
impl Cell for NandGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::Nand)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A~&B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OrGate {}
impl Cell for OrGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::Or)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A|B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NorGate {}
impl Cell for NorGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::Nor)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A~|B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct XorGate {}
impl Cell for XorGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::Xor)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A^B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct XnorGate {}
impl Cell for XnorGate {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> super::CellContents {
        CellContents::Primitive(PrimitiveType::Xnor)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "A~^B".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
            Port {
                name: "B".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(2)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Inverter {}
impl Cell for Inverter {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> CellContents {
        CellContents::Primitive(PrimitiveType::Not)
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "~A".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "A".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
        ];
        // type should be primitive so local location should be disregarded
        CellInterface::Builtin(Box::new(interface))
    }
}

// primitive non gate components
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    pub period: usize,
    pub pulse_width: usize,
}
impl Cell for Clock {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn interface(&self) -> CellInterface {
        let interface = [Port {
            name: "clk".to_string(),
            port_type: PortType::Output,
            local_location: Address(CellHandle(0), PortHandle(0)),
        }];
        CellInterface::Builtin(Box::new(interface))
    }

    fn contents(&self) -> CellContents {
        let period = self.period;
        let pulse_width = self.pulse_width;
        let expr = Arc::new(move |_index, tick| -> circuit::signal::Signal {
            match ((tick % period as u128) as usize) < pulse_width {
                true => circuit::signal::Signal::True,
                false => circuit::signal::Signal::False,
            }
        });
        CellContents::Primitive(PrimitiveType::Input(expr))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PrintOutput {}
impl Cell for PrintOutput {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(*self)
    }

    fn contents(&self) -> CellContents {
        let expr = Arc::new(|index, tick, signal| {
            print!("index: {index} = {signal} at {tick} \n");
        });
        CellContents::Primitive(PrimitiveType::Output(expr))
    }

    fn interface(&self) -> CellInterface {
        let interface = [
            Port {
                name: "state".to_string(),
                port_type: PortType::Output,
                local_location: Address(CellHandle(0), PortHandle(0)),
            },
            Port {
                name: "watch".to_string(),
                port_type: PortType::Input,
                local_location: Address(CellHandle(0), PortHandle(1)),
            },
        ];
        CellInterface::Builtin(Box::new(interface))
    }
}
