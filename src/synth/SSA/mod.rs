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
        port_allocations: Vec<SignalID>,
    ) -> Result<(), NetlistLowerError> {
        // we cannot instantiate a module which is empty.
        if module.cells.is_empty() {
            return Err(NetlistLowerError::EmptyModule);
        };

        // Setup namespace to keep track of where has been allocated.
        let mut name_space: HashMap<Address, SignalID> = HashMap::new();

        // intitialize namespace by ensuring all allocations from
        // parent module are added to namespace.
        for (port_idx, port_desc) in module.portlist.iter().enumerate() {
            let alloc = port_allocations
                .get(port_idx)
                .expect("unallocated port passed in during lowering process");

            name_space.insert(port_desc.local_location, *alloc);
        }

        for (cell_idx, cell) in module.cells.iter().enumerate() {
            let cell_handle = CellHandle(cell_idx);

            // handle
            match cell.contents() {
                CellContents::BuiltinModule(module) => {
                    todo!()
                }
                CellContents::UserModule(module_handle) => {
                    todo!()
                }
                CellContents::Primitive(contents) => {
                    todo!()
                }
                CellContents::InputPlaceholder => {
                    // literally do nothing for this branch do not allocate don't pass go don't collect
                    // 200$.

                    // This is a special case of cell (a hack lol) so that cells local to
                    // the current module have something to link to. It should be given an allocation
                    // passed in from the parent module.

                    // There is no analogue for output ports because circuit representation used for simulation
                    // is singally linked meaning each node is only aware of nodes which it depends on. So the
                    // local adress which said output links to will simply be given an allocation from the parent
                    // up front.
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
    InputPlaceholder,
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
    PortDNE,
}
