/*
Beware all ye mortals who enter here I this file is born from the ravings of a madwoman
long since lost to the abominiation of the elder gods which is recursively flattening
a the farce of a netlist which was the graph structure designed by her own huberis.
There is no god and no master in this domain. Simply the horrors and turmoil of handle
translation.
*/

pub mod cell_types;

use crate::back_end::circuit::{self, operation::SignalID};
use crate::uuid::Uuid;
use std::{collections::HashMap, fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct Netlist {
    modules: HashMap<ModuleHandle, Module>,
}

impl Netlist {
    pub fn as_circuit(
        &self,
        module_handle: ModuleHandle,
        input_tick_ratio: usize,
    ) -> Result<circuit::Circuit, NetlistLowerError> {
        let mut gld = circuit::builder::GateLevelDescription::new();
        let top = self.modules.get(module_handle.0).unwrap();

        let mut port_allocations = Vec::new();
        for _idx in 0..top.portlist.len() {
            port_allocations.push(gld.rz_alloc())
        }

        self.lower(&mut gld, top, port_allocations)?;

        Result::Ok(circuit::Circuit::new(gld.into_desc(), input_tick_ratio))
    }

    /// A recursive function which builds an instance of the provided module in
    /// gate level description (gld) returning by reference. note that *all*
    /// ports must be pre-allocated by the caller of the function.
    fn lower(
        &self,
        gld: &mut circuit::builder::GateLevelDescription,
        module: &Module,
        port_allocations: Vec<SignalID>,
    ) -> Result<(), NetlistLowerError> {
        // we cannot instantiate a module which is empty.
        if module.cells.is_empty() {
            return Err(NetlistLowerError::EmptyModule);
        };

        // Setup namespace to keep track of where has been allocated.
        let mut name_space: HashMap<Address, SignalID> = HashMap::new();

        // intitialize namespace by ensuring all allocations from
        // parent module are added.
        for (port_handle, port_desc) in module.portlist.iter() {
            let alloc = match port_allocations.get(port_idx) {
                Some(t) => t,
                None => {
                    return Result::Err(NetlistLowerError::PortNotAllocated);
                }
            };
            name_space.insert(port_desc.local_location, *alloc);
        }

        for (cell_idx, cell) in module.cells.iter().enumerate() {
            let cell_handle = CellHandle(cell_idx);

            let mut child_port_mapping = Vec::new();
            let cell_interface = match cell.interface() {
                CellInterface::Builtin(t) => t,
                CellInterface::UserModule(module) => match self.modules.get(module.0) {
                    Some(t) => Box::from(t.portlist.as_slice()),
                    None => {
                        return Result::Err(NetlistLowerError::ModuleHandleDNE);
                    }
                },
            };
            for (port_idx, port_desc) in cell_interface.iter().enumerate() {
                let port_handle = PortHandle(port_idx);
                let current_address = Address(cell_handle, port_handle);

                let signal_id = match port_desc.port_type {
                    PortType::Input => {
                        // lookup source address or allocate and add it to namespace if needed
                        match module.wires.get(&Drain(current_address)) {
                            Option::Some(source_address) => match name_space.get(&source_address.0)
                            {
                                Some(sig) => *sig,
                                None => {
                                    let sig = gld.rz_alloc();
                                    name_space.insert(source_address.0, sig);
                                    sig
                                }
                            },
                            None => {
                                // **** you allocates your unlinked address.
                                // In all seriousness this should log a warning for the user
                                // since if they want a high impedance link it should be
                                // added to their design explicitly not via fallback allocation

                                gld.rz_alloc()
                            }
                        }
                    }
                    PortType::Output => {
                        // allocate and add current address to namespace if needed
                        match name_space.get(&current_address) {
                            Some(id) => *id,
                            None => {
                                let id = gld.rz_alloc();
                                name_space.insert(current_address, id);
                                id
                            }
                        }
                    }
                };

                child_port_mapping.push(signal_id);
            }

            // handle construction of cell contents
            match cell.contents() {
                CellContents::BuiltinModule(module) => {
                    self.lower(gld, module.as_ref(), child_port_mapping)?;
                }
                CellContents::UserModule(module_handle) => {
                    // get user specified module from list of modules in self
                    let module = match self.modules.get(module_handle.0) {
                        Some(t) => t,
                        None => {
                            return Result::Err(NetlistLowerError::ModuleHandleDNE);
                        }
                    };

                    // recurse into new module passing in the child_port_mapping generated
                    // at current scope of
                    self.lower(gld, module, child_port_mapping)?;
                }
                CellContents::Primitive(gate_type) => {
                    // Get all the port mappings primitives might use.
                    // The port indexes for this are hardcoded, but since
                    // I can't come up with a more sensible mappping i'm not
                    // going to make this more generalized.
                    let loc = child_port_mapping.get(0);
                    let lhs = child_port_mapping.get(1);
                    let rhs = child_port_mapping.get(2);

                    // This match statement just matches to the gate_type and calls
                    // the appropriate function on gld to instantiate it. It's very
                    // long and gross but not much happens.
                    match gate_type {
                        // unary
                        PrimitiveType::Not => {
                            gld.mk_not(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }

                        // binary
                        PrimitiveType::And => {
                            gld.mk_and(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        PrimitiveType::Nand => {
                            gld.mk_nand(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        PrimitiveType::Or => {
                            gld.mk_or(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        PrimitiveType::Nor => {
                            gld.mk_nor(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        PrimitiveType::Xor => {
                            gld.mk_xor(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        PrimitiveType::Xnor => {
                            gld.mk_xnor(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match rhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                            )
                            .unwrap();
                        }
                        // special
                        PrimitiveType::Input(expr) => {
                            gld.mk_input(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                expr,
                            )
                            .unwrap();
                        }
                        PrimitiveType::Output(expr) => {
                            gld.mk_output(
                                match loc {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                match lhs {
                                    Some(sig) => *sig,
                                    None => {
                                        return Result::Err(
                                            NetlistLowerError::ChildPortNotAllocated,
                                        );
                                    }
                                },
                                expr,
                            )
                            .unwrap();
                        }
                    }
                }
                CellContents::InputPlaceholder => {
                    /*
                    literally do nothing for this branch do not allocate don't pass go don't collect
                    200$.

                    This is a special case of cell (a hack lol) so that cells local to
                    the current module have something to link to. It should be given an allocation
                    passed in from the parent module.

                    There is no analogue for output ports because circuit representation used for simulation
                    is singally linked meaning each node is only aware of nodes which it depends on. So the
                    local adress which said output links to will simply be given an allocation from the parent
                    up front.
                    */
                }
            };
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    portlist: HashMap<PortHandle, Port>,
    wires: HashMap<Drain, Source>,
    cells: HashMap<CellHandle, Box<dyn Cell>>,
}

impl Module {
    fn new(name: String) -> Self {
        Self {
            name: name,
            portlist: HashMap::new(),
            wires: HashMap::new(),
            cells: HashMap::new(),
        }
    }

    fn mk_cell() {
        todo!()
    }

    fn mk_wire() {
        todo!()
    }
}

pub trait Cell: Debug {
    /// returns a box containing a deep copy of self
    fn clone_as_box(&self) -> Box<dyn Cell>;

    /// returns a lowered description of the Cell
    fn contents(&self) -> CellContents;

    /// returns a description of the cell interface
    fn interface(&self) -> CellInterface;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct ModuleHandle(Uuid);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CellHandle(Uuid);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PortHandle(Uuid);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Address(CellHandle, PortHandle);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Source(Address);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Drain(Address);

#[derive(Clone, Debug)]
pub struct Port {
    name: String,
    port_type: PortType,
    local_location: Address,
}

#[derive(Clone, Copy, Debug)]
pub enum PortType {
    Input,
    Output,
}

pub enum CellContents {
    Primitive(PrimitiveType),
    UserModule(ModuleHandle),
    BuiltinModule(Box<Module>),
    InputPlaceholder,
}
#[derive(Debug)]
pub enum CellInterface {
    Builtin(Box<[Port]>),
    UserModule(ModuleHandle),
}

pub enum PrimitiveType {
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    // it likely is possible to do these without refrence counting but
    // this should work and it's not worth engineering that right now
    Input(Arc<dyn Fn(usize, u128) -> circuit::signal::Signal + Sync + Send>),
    Output(Arc<dyn Fn(usize, u128, circuit::signal::Signal) + Sync + Send>),
}

#[derive(Clone, Copy, Debug)]
pub enum NetlistLowerError {
    EmptyModule,
    ModuleHandleDNE,
    PortNotAllocated,
    ChildPortNotAllocated,
}

pub struct NetlistLowerContext {}

#[cfg(test)]
mod tests {
    use std::collections::btree_map;

    use circuit::signal::{self, Signal};

    use super::*;

    #[test]
    /// instantiates a 2 gate nor latch and simulates it to test that sequential behavior including race conditions is preserved
    // TODO: remove make Print nodes be replaced with a NOP node to reduce test runtime if the testing isn't being run manually.
    fn latch_lower_and_sim() {
        let mut cells: Vec<Box<dyn Cell>> = Vec::new();
        use cell_types::*;
        const PRINT_Q: bool = false;
        const PRINT_Q_NOT: bool = false;

        // cell 00 S
        cells.push(Box::new(Waveform {
            setup_time: 0,
            waveform: vec![
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::True, // (S:1, R:0)
                Signal::True,
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::False, // (S:0, R:1)
                Signal::False,
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::True, // (S:1, R:1)
                Signal::True,
            ],
        }));
        // cell 01 R
        cells.push(Box::new(Waveform {
            setup_time: 0,
            waveform: vec![
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::False, // (S:1, R:0)
                Signal::False,
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::True, // (S:0, R:1)
                Signal::True,
                Signal::False, // (S:0, R:0)
                Signal::False,
                Signal::True, // (S:1, R:1)
                Signal::True,
            ],
        }));
        // cell 02
        cells.push(Box::new(NorGate {}));
        // cell 03
        cells.push(Box::new(NorGate {}));

        // Describe waveform to assert on Q
        let mut test_waveform = btree_map::BTreeMap::new();
        test_waveform.insert(0, Signal::UncontrolledTrue);
        test_waveform.insert(1, Signal::UncontrolledFalse);
        test_waveform.insert(2, Signal::UncontrolledTrue);
        test_waveform.insert(3, Signal::UncontrolledFalse);
        test_waveform.insert(4, Signal::UncontrolledFalse);
        test_waveform.insert(13, Signal::UncontrolledTrue);
        test_waveform.insert(20, Signal::UncontrolledFalse);
        test_waveform.insert(22, Signal::False);

        // cell 04 Q Assert
        cells.push(Box::new(DeltaAssert {
            waveform: test_waveform,
            period: 24,
            setup_time: 6,
            phase_offset: 2,
        }));

        // Optional printing (don't enable if you aren't actively debugging since print is slow to run)
        if PRINT_Q {
            cells.push(Box::new(Print {})); // cell 05 Q
        }
        if PRINT_Q_NOT {
            cells.push(Box::new(Print {})); // cell 06 Q!
        }

        let mut wires = HashMap::new();
        // connect nor gates to clock inputs
        wires.insert(
            Drain(Address(CellHandle(2), PortHandle(1))),
            Source(Address(CellHandle(0), PortHandle(0))),
        );
        wires.insert(
            Drain(Address(CellHandle(3), PortHandle(1))),
            Source(Address(CellHandle(1), PortHandle(0))),
        );
        // connect nor gates to eachother creating feedback path
        wires.insert(
            Drain(Address(CellHandle(2), PortHandle(2))),
            Source(Address(CellHandle(3), PortHandle(0))),
        );
        wires.insert(
            Drain(Address(CellHandle(3), PortHandle(2))),
            Source(Address(CellHandle(2), PortHandle(0))),
        );
        // connect Print to Q
        wires.insert(
            Drain(Address(CellHandle(5), PortHandle(1))),
            Source(Address(CellHandle(2), PortHandle(0))),
        );
        // connect Print to Q!
        wires.insert(
            Drain(Address(CellHandle(6), PortHandle(1))),
            Source(Address(CellHandle(3), PortHandle(0))),
        );
        // connect Testing Assert to Q
        wires.insert(
            Drain(Address(CellHandle(4), PortHandle(1))),
            Source(Address(CellHandle(2), PortHandle(0))),
        );

        let netlist = Netlist {
            modules: vec![Module {
                name: "Latch".to_string(),
                // No ports needed since this is top level module
                portlist: Vec::new(),
                wires,
                cells,
            }],
        };

        let mut circuit = netlist.as_circuit(ModuleHandle(0), 2).unwrap();

        for idx in 0..240 {
            circuit.tick()
        }
    }

    #[test]
    /// instantiate a full adder and test that it instantiates correctly.
    fn test_case_full_adder() {
        const PRINT_Z: bool = false;
        const PRINT_C: bool = false;

        let mut cells: Vec<Box<dyn Cell>> = Vec::new();
        let mut wires: HashMap<Drain, Source> = HashMap::new();

        todo!("unfinished")
    }

    #[test]
    /// instantiate a 2 bit ripple adder and test that it instantiates correctly.
    fn test_case_ripple_adder() {
        todo!()
    }
}
