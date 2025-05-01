mod cell_types;

struct Netlist {
    modules: Vec<Module>,
}

impl Netlist {
    fn flatten(&self, module_id: usize) -> Module {
        todo!()
    }
}
struct Module {
    name: Box<str>,
    portlist: Vec<Port>,
    wires: Vec<Wire>,
    cells: Vec<Box<dyn Cell>>,
}

impl Module {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            portlist: Vec::new(),
            wires: Vec::new(),
            cells: Vec::new(),
        }
    }
}

impl Cell for Module {
    fn clone_as_box(&self) -> Box<dyn Cell> {
        let mut clone_cells = Vec::with_capacity(self.cells.len());

        for cell in &self.cells {
            clone_cells.push(cell.clone_as_box());
        }

        Box::new(Module {
            name: self.name.clone(),
            portlist: self.portlist.clone(),
            wires: self.wires.clone(),
            cells: clone_cells,
        })
    }

    fn lower(&self) -> Result<&Module, LowerError> {
        Ok(&self)
        // in most cases conversion from Self to Module likely via a constant reference
        // design would happen here. However since Self is already module we just need
        // to provide a reference to self.
    }

    fn ports(&self) -> &Vec<Port> {
        &self.ports()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct CellHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct PortHandle(usize);

#[derive(Clone, Debug)]
struct Port {
    name: String,
    port_type: PortType,
    local_address: (CellHandle, PortHandle),
}

#[derive(Clone, Copy, Debug)]
enum PortType {
    Input,
    Output,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Wire {
    source: (CellHandle, PortHandle),
    drain: (CellHandle, PortHandle),
}

trait Cell {
    /// returns a box containing a deep copy of self
    fn clone_as_box(&self) -> Box<dyn Cell>;

    /// returns a lowered description of the Cell
    fn lower(&self) -> Result<&Module, LowerError>;

    /// returns a description of the Portlist to a Cell
    fn ports(&self) -> &Vec<Port>;
}

enum LowerError {
    IsPrimitive,
}
