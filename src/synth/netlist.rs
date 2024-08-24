/// A list of all netlists used by a design Index 0 is considered the top level module.
/// Instantiations called in the top level module there after will search through the
/// list of instantiated designs incrementing appending to the netlists vec if they have not already been instantiated.
/// This allows for cyclic references between netlists where for example netlist 0 references netlist 1 and vice versa
/// without causing parsing to hang until attempting ot flatten the netlist, where it may be properly recongized and warn the user.
pub struct Design {
    netlists: Vec<Netlist>,
}

/// The Highest level building  block of a deign
pub struct Netlist {
    name: String,
    interface: Vec<NetReference>,
    clock_domains: Vec<(SensativityList, Vec<NetReference>)>,
    nets: Vec<Net>,
    cells: Vec<Cell>,
}

impl Netlist {
    /// Constructs an empty netlist object with given name
    pub fn new(name: String) -> Self {
        Self {
            name,
            interface: Vec::new(),
            clock_domains: Vec::new(),
            nets: Vec::new(),
            cells: Vec::new(),
        }
    }

    // # The following section describes functions relating to the creation and assignment
    // of Net objects within a Netlist

    /// Creates a new wire net and returns the handle to that net.
    /// Will error if a net of size 0 is requested
    pub fn mk_wire(&mut self, size: usize) -> Result<NetReference, NetlistErr> {
        let net_stack_size = self.nets.len();
        return match size {
            0 => Result::Err(NetlistErr::SizeZeroNet()),
            1 => {
                self.nets.push(Net::Scalar(Scalar::Wire {
                    driven_by: None,
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
            _ => {
                self.nets.push(Net::Vector(Vector::Wire {
                    size,
                    driven_by: None,
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
        };
    }

    /// Creates a new register net and returns the handle to that net.
    /// Will error if a net of size 0 is requested
    pub fn mk_reg(&mut self, size: usize) -> Result<NetReference, NetlistErr> {
        let net_stack_size = self.nets.len();
        return match size {
            0 => Result::Err(NetlistErr::SizeZeroNet()),
            1 => {
                self.nets.push(Net::Scalar(Scalar::Register {
                    driven_by: None,
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
            _ => {
                self.nets.push(Net::Vector(Vector::Register {
                    size,
                    driven_by: None,
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
        };
    }

    /// Creates a new constant net and returns the handle to that net.
    /// Will error if a net of size 0 is requested
    pub fn mk_const(
        &mut self,
        pattern: Box<[crate::sim::circuit::signal::Signal]>,
    ) -> Result<NetReference, NetlistErr> {
        let net_stack_size = self.nets.len();
        return match pattern.len() {
            0 => Result::Err(NetlistErr::SizeZeroNet()),
            1 => {
                self.nets.push(Net::Scalar(Scalar::Constant {
                    pattern: pattern.as_ref()[0],
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
            _ => {
                self.nets.push(Net::Vector(Vector::Constant {
                    pattern,
                    driving: Vec::new(),
                }));
                Result::Ok(NetReference(net_stack_size))
            }
        };
    }

    fn get_net(&mut self, net_ref: NetReference) -> Result<&mut Net, NetlistErr> {
        match self.nets.get_mut(net_ref.0) {
            Some(found_net) => Result::Ok(found_net),
            None => Result::Err(NetlistErr::NetDNE(net_ref)),
        }
    }

    /// Assigns driving value produced by some Cell to a Net.
    /// Will error if, the location at Nethandle was never allocated,
    /// the size of the Port does not match the size of the Net,
    /// or the Net has already has a driving value assigned.
    pub fn assig_net(&mut self, net_ref: NetReference, value: PortList) -> Result<(), NetlistErr> {
        match self.get_net(net_ref) {
            Ok(net) => match net {
                Net::Scalar(scalar) => {
                    if value.0.len() == 1 {
                        match scalar {
                            Scalar::Constant { .. } => Result::Err(NetlistErr::ConstAssig(net_ref)),
                            Scalar::Register { driven_by, .. } => {
                                if let None = driven_by {
                                    *driven_by = Some(value.0[0]);
                                    Result::Ok(())
                                } else {
                                    Result::Err(NetlistErr::RepeatAssig(net_ref))
                                }
                            }
                            Scalar::Wire { driven_by, .. } => {
                                if let None = driven_by {
                                    *driven_by = Some(value.0[0]);
                                    Result::Ok(())
                                } else {
                                    Result::Err(NetlistErr::RepeatAssig(net_ref))
                                }
                            }
                        }
                    } else {
                        Result::Err(NetlistErr::NetSizeMismatch(net_ref, value))
                    }
                }
                Net::Vector(vector) => match vector {
                    Vector::Constant { .. } => Result::Err(NetlistErr::ConstAssig(net_ref)),
                    Vector::Register {
                        size, driven_by, ..
                    } => {
                        if let None = driven_by {
                            if *size == value.0.len() {
                                *driven_by = Some(value);
                                Result::Ok(())
                            } else {
                                Result::Err(NetlistErr::NetSizeMismatch(net_ref, value))
                            }
                        } else {
                            Result::Err(NetlistErr::RepeatAssig(net_ref))
                        }
                    }
                    Vector::Wire {
                        size, driven_by, ..
                    } => {
                        if let None = driven_by {
                            if *size == value.0.len() {
                                *driven_by = Some(value);
                                Result::Ok(())
                            } else {
                                Result::Err(NetlistErr::NetSizeMismatch(net_ref, value))
                            }
                        } else {
                            Result::Err(NetlistErr::RepeatAssig(net_ref))
                        }
                    }
                },
            },
            Err(msg) => Result::Err(msg),
        }
    }

    /// Adds a port_list or port_reference to the list of locations which a net is driving
    fn read_net(&mut self, net_ref: NetReference, reader: PortList) -> Result<(), NetlistErr> {
        let net = self.get_net(net_ref)?;

        return match net {
            Net::Scalar(net_type) => match net_type {
                Scalar::Constant { driving, .. } => {
                    if reader.0.len() == 1 {
                        driving.push(reader.0[0]);
                        Result::Ok(())
                    } else {
                        Result::Err(NetlistErr::NetSizeMismatch(net_ref, reader))
                    }
                }
                Scalar::Register { driving, .. } => {
                    if reader.0.len() == 1 {
                        driving.push(reader.0[0]);
                        Result::Ok(())
                    } else {
                        Result::Err(NetlistErr::NetSizeMismatch(net_ref, reader))
                    }
                }
                Scalar::Wire { driving, .. } => {
                    if reader.0.len() == 1 {
                        driving.push(reader.0[0]);
                        Result::Ok(())
                    } else {
                        Result::Err(NetlistErr::NetSizeMismatch(net_ref, reader))
                    }
                }
            },

            Net::Vector(net_type) => match net_type {
                Vector::Constant { driving, .. } => {
                    todo!("finish writing each branch");
                    // driving.push();
                    Result::Ok(())
                }
                Vector::Register { driving, .. } => {
                    todo!("finish writing each branch");
                    // driving.push();
                    Result::Ok(())
                }
                Vector::Wire { driving, .. } => {
                    todo!("finish writing each branch");
                    // driving.push();
                    Result::Ok(())
                }
            },
        };
    }

    // # The following section provides functions relating to the creation of Operation Cells

    /// Creates an and cell
    pub fn mk_and_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a nand cell
    pub fn mk_nand_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a or cell
    pub fn mk_or_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a nor cell
    pub fn mk_nor_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a xor cell
    pub fn mk_xor_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a xnor cell
    pub fn mk_xnor_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates an add cell
    pub fn mk_add_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a subtract cell
    pub fn mk_sub_cell(
        &mut self,
        lhs: NetReference,
        rhs: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a shl cell
    pub fn mk_shl_cell(
        &mut self,
        data: NetReference,
        shift: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    /// Creates a shr cell
    pub fn mk_shr_cell(
        &mut self,
        data: NetReference,
        shift: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    pub fn mk_mux_cell(
        &mut self,
        data: Box<[NetReference]>,
        select: NetReference,
        result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    pub fn mk_splice_cell(
        &mut self,
        data: Box<[NetReference]>,
        splice_result: NetReference,
    ) -> Result<(), NetlistErr> {
        todo!()
    }

    // # The following section provides functions relating to the creation of Interface Cells

    /// Creates an input which writes to loc from the netlist interface
    pub fn mk_input_cell(
        &mut self,
        data_loc: NetReference,
    ) -> Result<InterfaceReference, NetlistErr> {
        // note down next cell and interface locations
        let cell_ref = CellReference(self.cells.len());
        let interface_ref = InterfaceReference(self.interface.len());

        // create portlist for input cell net assignment
        let port_list = match self.get_net(data_loc) {
            Ok(net) => match net {
                Net::Scalar(scalar) => match scalar {
                    Scalar::Constant { .. } => {
                        return Result::Err(NetlistErr::ConstAssig(data_loc));
                    }
                    Scalar::Register { .. } => {
                        return Result::Err(NetlistErr::IllegalRegisterNet(data_loc));
                    }
                    Scalar::Wire { .. } => PortList(Box::new([PortReference(cell_ref, 0)])),
                },
                Net::Vector(vector) => match vector {
                    Vector::Constant { .. } => {
                        return Result::Err(NetlistErr::ConstAssig(data_loc));
                    }
                    Vector::Register { .. } => {
                        return Result::Err(NetlistErr::IllegalRegisterNet(data_loc));
                    }
                    Vector::Wire { size, .. } => {
                        let mut list = Vec::new();
                        let mut port_cntr = 0;
                        list.resize_with(*size, || {
                            let next_port = PortReference(cell_ref, port_cntr);
                            port_cntr += 1;
                            next_port
                        });

                        PortList(list.into_boxed_slice())
                    }
                },
            },
            Err(msg) => return Result::Err(msg),
        };

        // create input cell object
        self.cells
            .push(Cell::Interface(Interface::Input(interface_ref, data_loc)));

        // link new input cell to input net or propagate error
        if let Err(err) = self.assig_net(data_loc, port_list) {
            return Result::Err(err);
        }

        // link input net to interface
        self.interface.push(data_loc);

        Result::Ok(interface_ref)
    }

    /// Creates an output cell which reads loc to the netlist interface
    pub fn mk_output_cell(
        &mut self,
        net_ref: NetReference,
    ) -> Result<InterfaceReference, NetlistErr> {
        todo!("")
    }

    // # The following section provides functions relating to the creation of sub-modules within a netlist

    /// Creates a sub-module cell
    pub fn mk_mod_cell(&mut self, interface: Box<[NetReference]>) -> Result<(), NetlistErr> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct NetlistHandle(usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct NetReference(usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct CellReference(usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct PortReference(CellReference, usize);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PortList(Box<[PortReference]>);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct InterfaceReference(usize);

struct SensativityList(Vec<EventComponent>);
enum EventComponent {
    Posedge(NetReference),
    Negedge(NetReference),
    LevelSensative(NetReference),
}

/// A collection data either constant or forwarded from Cell ports
enum Net {
    Scalar(Scalar),
    Vector(Vector),
}

/// A net containing a single bit of information
enum Scalar {
    Constant {
        pattern: crate::sim::circuit::signal::Signal,
        driving: Vec<PortReference>,
    },
    Wire {
        driven_by: Option<PortReference>,
        driving: Vec<PortReference>,
    },
    Register {
        driven_by: Option<PortReference>,
        driving: Vec<PortReference>,
    },
}

/// A net containing a series of bits of information
enum Vector {
    Constant {
        pattern: Box<[crate::sim::circuit::signal::Signal]>,
        driving: Vec<PortList>,
    },
    Wire {
        size: usize,
        driven_by: Option<PortList>,
        driving: Vec<PortList>,
    },
    Register {
        size: usize,
        driven_by: Option<PortList>,
        driving: Vec<PortList>,
    },
}

enum Cell {
    Interface(Interface),
    Module(Module),
    Operator(Operator),
    /// follows form ```(data, select, result)```, and must assert that ```log2(data.len()) == select.len()```
    Multiplexor(Box<[NetReference]>, NetReference, NetReference),
    /// follows form ```(data_source_list, spliced_result)``` and must assert that ```spliced_result.size == sum (data_source_list.size))```
    Splice(Box<[NetReference]>, NetReference),
}

enum Interface {
    Input(InterfaceReference, NetReference),
    Output(InterfaceReference, NetReference),
}

struct Module {
    module_name: String,
    netlist: NetlistHandle,
    interface: Box<[NetReference]>,
}

enum Operator {
    Unary(Unary),
    Binary(Binary),
}

/// A unary operator primitive
/// The portlist follows the standard format ```(Input, Result)```
enum Unary {
    Not(NetReference, NetReference),
}

/// A binary operator primitive
/// The portlist follows the standard format ```(LHS, RHS, Result)```
enum Binary {
    And(NetReference, NetReference, NetReference),
    Nand(NetReference, NetReference, NetReference),
    Or(NetReference, NetReference, NetReference),
    Nor(NetReference, NetReference, NetReference),
    Xor(NetReference, NetReference, NetReference),
    Xnor(NetReference, NetReference, NetReference),
    Add(NetReference, NetReference, NetReference),
    Subtract(NetReference, NetReference, NetReference),
    ShiftLeft(NetReference, NetReference, NetReference),
    ShiftRight(NetReference, NetReference, NetReference),
}

#[derive(Debug, PartialEq, Eq)]
enum NetlistErr {
    ConstAssig(NetReference),
    IllegalRegisterNet(NetReference),
    NetDNE(NetReference),
    NetSizeMismatch(NetReference, PortList),
    RepeatAssig(NetReference),
    SizeZeroNet(),
}

#[cfg(test)]
mod tests {
    use crate::synth::netlist::{NetReference, NetlistErr};

    use super::Netlist;

    #[test]
    fn shift_register_integration_testing() {}

    #[test]
    fn mk_net() {}

    #[test]
    fn assig() {}

    #[test]
    fn input() {
        use self::*;
        use crate::sim::circuit::signal::Signal;

        {
            // scalare wire testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let scalar_wire = test_netlist.mk_wire(1).unwrap();
            let interface_ref = test_netlist.mk_input_cell(scalar_wire).unwrap();
            assert!(test_netlist.interface.len() == 1);
            assert!(interface_ref.0 == 0);
            println!("scalar wire pass");
        }

        {
            // vector wire testcase
            let size = 21;
            let mut test_netlist = Netlist::new("input_test".to_string());
            let vector_wire = test_netlist.mk_wire(size).unwrap();
            let interface_ref = test_netlist.mk_input_cell(vector_wire).unwrap();
            assert!(test_netlist.interface.len() == 1);
            assert!(interface_ref.0 == 0);
            println!("vector wire pass");
        }

        {
            // scalar register testcase - negative testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let scalar_reg = test_netlist.mk_reg(1).unwrap();
            let err = test_netlist.mk_input_cell(scalar_reg).unwrap_err();
            assert!(err == NetlistErr::IllegalRegisterNet(scalar_reg));
            println!("scalar register pass");
        }

        {
            // vector register testcase - negative testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let vector_reg = test_netlist.mk_reg(21).unwrap();
            let err = test_netlist.mk_input_cell(vector_reg).unwrap_err();
            assert!(err == NetlistErr::IllegalRegisterNet(vector_reg));
            println!("vector register pass");
        }

        {
            // scalar constant testcase - negative testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let scalar_constant = test_netlist
                .mk_const(vec![Signal::False].into_boxed_slice())
                .unwrap();
            let err = test_netlist.mk_input_cell(scalar_constant).unwrap_err();
            assert!(err == NetlistErr::ConstAssig(scalar_constant));
            println!("scalar constant pass");
        }

        {
            // vector constant testcase - negative testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let vector_constant = test_netlist
                .mk_const(vec![Signal::False, Signal::False, Signal::False].into_boxed_slice())
                .unwrap();
            let err = test_netlist.mk_input_cell(vector_constant).unwrap_err();
            assert!(err == NetlistErr::ConstAssig(vector_constant));
            println!("vector constant pass");
        }

        {
            // unallocated testcase - negative testcase
            let mut test_netlist = Netlist::new("input_test".to_string());
            let unallocated_net = NetReference(777);
            let err = test_netlist.mk_input_cell(unallocated_net).unwrap_err();
            assert!(err == NetlistErr::NetDNE(unallocated_net));
            println!("unallocated net pass");
        }
    }
}
