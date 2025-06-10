use crate::sim::circuit::{self, operation::SignalID};
use std::collections::HashMap;

mod cell_types;

struct Netlist {
    modules: Vec<Module>,
}

impl Netlist {
    pub fn as_circuit(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<circuit::Circuit, NetlistLowerError> {
        let mut gld = circuit::builder::Module::new();
        let top = self.modules.get(module_handle.0).unwrap();

        /*
        At present tpi is static but would ideally be dynamically inferred from the depth of the gld
        This is mostly because it's not pressing to worry about that at present and assuming a depth of 1k
        is plenty sufficient for basic testing.
        */
        let tpi = 1000;

        Result::Ok(circuit::Circuit::new(gld.into_desc(), tpi))
    }

    /// A recursive function which builds an instance of the provided module in
    /// gate level description (gld) returning by reference. note that *all*
    /// ports must be pre-allocated by the caller of the function.
    fn lower(
        &self,
        gld: &mut circuit::builder::Module,
        module: &Module,
        ports: &HashMap<PortHandle, SignalID>,
    ) -> Result<(), NetlistLowerError> {
        // we cannot instantiate a module which is empty
        if module.cells.is_empty() {
            return Err(NetlistLowerError::EmptyModule);
        };

        let mut name_space: HashMap<Address, SignalID> = HashMap::new();
        for (cell_idx, cell) in module.cells.as_slice().into_iter().enumerate() {
            let cell_handle = CellHandle(cell_idx);

            match cell.contents() {
                CellContents::BuiltinModule(module) => {
                    // recurse into module
                    todo!()
                }
                CellContents::UserModule(module_handle) => {
                    // get definition of use
                    let user_defined_module = match self.modules.get(module_handle.0) {
                        Some(T) => T,
                        None => {
                            return Err(NetlistLowerError::ModuleHandleDNE);
                        }
                    };

                    // recurse into user defined module
                    todo!()
                }
                CellContents::Primitive(contents) => {
                    let interface = cell.interface();
                    let mut port_signal_ids = Vec::with_capacity(interface.len());

                    for (port_idx, port) in interface.into_iter().enumerate() {
                        let port_handle = PortHandle(port_idx);
                        let current_adress = Address(cell_handle, port_handle);
                        match port.port_type {
                            PortType::Input => {
                                // fetch source address
                                // may be none representing a high impredance input
                                match module.wires.get(&Drain(current_adress)) {
                                    Some(source_address) => {
                                        // check namespace for prior allocation of source address and allocate it if one does not exist
                                        let signal_id = match name_space.get(&source_address.0) {
                                            Some(T) => T.to_owned(),
                                            None => {
                                                let T = gld.rz_alloc();
                                                name_space.insert(source_address.0, T);
                                                T
                                            }
                                        };
                                        port_signal_ids.push(signal_id);
                                    }
                                    None => {
                                        // allocate a new signal and leave it unassigned/named to represent the high
                                        // impedance input. push that sighalID to current port without adding to namespace
                                        port_signal_ids.push(gld.rz_alloc());
                                    }
                                }
                            }
                            PortType::Output => {
                                let signal_id = match name_space.get(&current_adress) {
                                    Some(signal_id) => {}
                                    None => {}
                                };
                            }
                        }

                        // fetch signal id of address or allocate a new one
                        let signal_id = match name_space.get(&Address(cell_handle, port_handle)) {
                            Some(T) => T,
                            None => &gld.rz_alloc(),
                        };
                    }
                }
            }
        }

        Ok(())
    }
}

struct Module {
    name: Box<str>,
    portlist: Vec<Port>,
    wires: HashMap<Drain, Source>,
    cells: Vec<Box<dyn Cell>>,
}

impl Module {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            portlist: Vec::new(),
            wires: HashMap::new(),
            cells: Vec::new(),
        }
    }
}

trait Cell {
    /// returns a box containing a deep copy of self
    fn clone_as_box(&self) -> Box<dyn Cell>;

    /// returns a lowered description of the Cell
    fn contents(&self) -> CellContents;

    /// returns a description of the cell interface
    fn interface(&self) -> &[Port];
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct ModuleHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct CellHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct PortHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Address(CellHandle, PortHandle);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Source(Address);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Drain(Address);

#[derive(Clone, Debug)]
struct Port {
    name: String,
    port_type: PortType,
    local_location: Address,
}

#[derive(Clone, Copy, Debug)]
enum PortType {
    Input,
    Output,
}

enum CellContents<'a> {
    Primitive(PrimitiveType),
    UserModule(ModuleHandle),
    BuiltinModule(&'a Module),
}

enum PrimitiveType {
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Input(
        circuit::operation::InputHandler<
            dyn Fn(usize, u128) -> circuit::signal::Signal + Sync + Send,
        >,
    ),
    Output(
        circuit::operation::OutputHandler<
            dyn Fn(usize, u128, circuit::signal::Signal) + Sync + Send,
        >,
    ),
}

enum NetlistLowerError {
    EmptyModule,
    ModuleHandleDNE,
}
