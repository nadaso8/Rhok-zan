use std::rc::Rc;



struct ComponentID(usize);
struct PortID(usize);
struct NetID(ComponentID, PortID);
struct Netlist {
    desc: Vec<Component>
}


enum Component {
    SubModule(Rc<Netlist>),
    Primitive(Primitive)
}

struct SubModule {
    desc: Rc<Netlist>,
    inputs: Vec<NetID>,
    outptus: Vec<NetID>
}

enum Primitive {
    Not(NetID),
    And(NetID, NetID),
    Nand(NetID, NetID),
    Or(NetID, NetID),
    Nor(NetID, NetID),
    Xor(NetID, NetID),
    Xnor(NetID, NetID),
    Multiplex{
        //
        default: Box<[NetID]>,
        case: Box<[(usize, )]>
    },
    Reg,
    Add,
    Subtract,
    Shift,
    Multiply,
    Divide
}