pub struct Design {
    netlists: Vec<Netlist>,
}

impl Design {
    fn get_netlist(&self, netlist: NetlistID) -> &Netlist {
        todo!()
    }
}
pub struct NetlistID(usize);

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

pub struct NetID(usize);
pub struct CellID(usize);
pub struct PortID(usize);
pub struct ClockID(usize);

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
    Module {
        description: NetlistID,
        inputs: Vec<NetID>,
        outputs: Vec<NetID>,
    },
    Input {
        port: PortID,
    },
    Output {
        port: PortID,
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ripple_adder() {}
}
