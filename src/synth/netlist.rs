use handle_types::*;

use crate::sim::circuit::{operation::SignalID, signal::Signal};

pub struct Design {
    netlists: Vec<Netlist>,
    inputs: Vec<
        Box<dyn Fn(usize, u128) -> Signal + Sync + Send + 'static>,
    >,
    outputs:
        Vec<Box<dyn Fn(usize, u128, Signal) + Sync + Send + 'static>>,
}

fn instantiate_cells(
    design: &Design,
    netlist_id: NetlistID,
    input_ports: &[SignalID],
    output_ports: &[SignalID],
    circuit: &mut crate::sim::circuit::builder::Module,
) -> Result<(), NetlistErr> {
    let netlist = design.get_netlist(netlist_id);
    let mut net_id_translation = std::collections::HashMap::new();
    for cell in netlist.cells {
        match cell {
            Cell::And { lhs, rhs, result } => {
                let result_sid = net_id_translation.entry(result).or_insert(default)
            }
            Cell::Nand { lhs, rhs, result } => {}
            Cell::Or { lhs, rhs, result } => {}
            Cell::Nor { lhs, rhs, result } => {}
            Cell::Xor { lhs, rhs, result } => {}
            Cell::Xnor { lhs, rhs, result } => {}
            Cell::Not { input, result } => {}
            Cell::Input { port } => {}
            Cell::Output { input, port } => {}
            Cell::Module {
                description,
                inputs,
                outputs,
            } => {},
        }
    }
    return Ok(());
}
impl Design {
    fn get_netlist(&self, netlist_handle: NetlistID) -> &Netlist {
        self.netlists.get(netlist_handle.0).unwrap()
    }

    fn as_box_slice(&self) -> Box<[crate::sim::circuit::operation::Operation]> {
        use crate::sim::circuit;
        let instantiate_cells = |design: &Design, netlist_id, circuit| {};

        const TOP_NETLIST_ID: NetlistID = NetlistID(0);
        let mut circuit = circuit::builder::Module::new();
        for input in self.inputs {
            let input_loc = circuit.rz_alloc();
            circuit.mk_input(loc, expr)
        }
        instantiate_cells(self, TOP_NETLIST_ID, , ,circuit);
        return circuit.into_desc();
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
        inputs: Box<[NetID]>,
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

pub enum NetlistErr {
    Unknown(String),
    DuplicateNetID,
}

impl From<String> for NetlistErr {
    fn from(value: String) -> Self {
        NetlistErr::Unknown(value)
    }
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
