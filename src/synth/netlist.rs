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
        let mut cut = builder::Module::new();
        let top = value.get_netlist(NetlistID(0));

        cut.into_desc()
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
    pub struct NetlistID(pub usize);
    pub struct NetID(pub usize);
    pub struct CellID(pub usize);
    pub struct PortID(pub usize);
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
