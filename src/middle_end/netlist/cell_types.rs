use circuit::signal;

use super::*;
use std::{
    borrow::BorrowMut,
    rc::{self, Rc},
    sync::mpsc::{channel, Receiver, SyncSender},
    u128, usize,
};

#[derive(Clone, Debug)]
pub struct ModuleInstance {
    instance_name: String,
    module_handle: ModuleHandle,
}
impl Cell for ModuleInstance {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(self.clone())
    }

    fn contents(&self) -> CellContents {
        CellContents::UserModule(self.module_handle)
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

    fn contents(&self) -> CellContents {
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
pub struct Print {}
impl Cell for Print {
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

#[derive(Debug, Clone)]
/// A cell which outputs the signal read on *watch* over an MPSC channel for processing/display on another thread.
///
/// Note that sender blocks on send which could cause the program to hang if the recieving thread ticks the simulation
/// and an output tries to send data to a full buffer.
///
/// I may add a corresponding InputChannel Cell. However, the architecture on that is more difficult as it rases questions
/// about when the channel is created and how it's passed to the thread generating values. At present just the OutputChannel
/// Cell is sufficient for MVP/Demo.
pub struct OutputChannel {
    tx: SyncSender<(usize, u128, signal::Signal)>,
}
impl Cell for OutputChannel {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(self.clone())
    }

    fn contents(&self) -> CellContents {
        let tx = self.tx.clone();
        let expr = Arc::new(move |index, tick, signal| {
            tx.send((index, tick, signal)).unwrap();
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

/// This cell is intended to be used for unit/integration tests
/// it will panic if the current value is different to what it
/// looks up from the expected waveform.
#[derive(Clone, Debug)]
pub struct Assert {
    waveform: Vec<signal::Signal>,
}

impl Cell for Assert {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        Box::new(self.clone())
    }

    fn contents(&self) -> CellContents {
        let waveform = self.waveform.clone().into_boxed_slice();
        let expr = Arc::new(move |index, tick, signal| {
            let waveform_idx = (tick % (waveform.len() as u128)) as usize;
            let expected = *waveform.get(waveform_idx).unwrap();
            assert_eq!(signal, expected);
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
