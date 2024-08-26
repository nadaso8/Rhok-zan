use handle_types::*;

pub struct Design {
    netlists: Vec<Netlist>,
}

impl Design {
    fn get_netlist(&self, netlist_handle: NetlistID) -> &Netlist {
        self.netlists.get(netlist_handle.0).unwrap()
    }
}

impl From<Design> for Box<[crate::sim::circuit::operation::Operation]> {
    fn from(value: Design) -> Self {
        use crate::sim::circuit::builder;

        let mut circuit_under_construction = builder::Module::new();
        let top_level_module = value.get_netlist(NetlistID(0));
        let mut handle_translation = std::collections::hash_map::HashMap::new();

        for cell in top_level_module.cells.as_slice() {
            match cell {
                Cell::Not { input, result } => {
                    let new_loc = circuit_under_construction.rz_alloc();
                    handle_translation.insert((NetlistID(0), result), new_loc);
                    let input_signal = handle_translation.get(&(NetlistID(0), input)).unwrap();
                    circuit_under_construction
                        .mk_not(new_loc, *input_signal)
                        .unwrap();
                }
                Cell::And { lhs, rhs, result } => {}
                Cell::Nand { lhs, rhs, result } => {}
                Cell::Or { lhs, rhs, result } => {}
                Cell::Nor { lhs, rhs, result } => {}
                Cell::Xor { lhs, rhs, result } => {}
                Cell::Xnor { lhs, rhs, result } => {}
                Cell::Input { port } => {}
                Cell::Output { input, port } => {}
                Cell::Module {
                    description,
                    inputs,
                    outputs,
                } => {}
            }
        }
        circuit_under_construction.into_desc()
    }
}

pub struct Netlist {
    nets: Vec<Net>,
    cells: Vec<Cell>,
    clocks: Vec<ClockDomain>,
}

pub enum Net {
    Constant {
        value: crate::sim::circuit::signal::Signal,
    },
    Wire {
        driver: (CellID, PortID),
    },
    Register {
        driver: (CellID, PortID),
        clock: ClockID,
    },
}

pub enum Cell {
    Not {
        input: NetID,
        result: NetID,
    },
    And {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Nand {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Or {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Nor {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Xor {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Xnor {
        lhs: NetID,
        rhs: NetID,
        result: NetID,
    },
    Input {
        port: PortID,
    },
    Output {
        input: NetID,
        port: PortID,
    },
    Module {
        description: NetlistID,
        inputs: Vec<NetID>,
        outputs: Vec<NetID>,
    },
}

pub struct ClockDomain {
    sensativitylist: Vec<Sensativity>,
}

pub enum Sensativity {
    Posedge(NetID),
    Negedge(NetID),
    LevelSensistive(NetID),
}

mod handle_types {
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NetlistID(pub usize);
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NetID(pub usize);
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CellID(pub usize);
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PortID(pub usize);
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ClockID(pub usize);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_adder() {}
    #[test]
    fn ripple_adder() {}
}
