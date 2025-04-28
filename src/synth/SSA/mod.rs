mod cell_types;

use crate::sim::circuit;
use std::rc::Rc;

struct Netlist {
    modules: Vec<Rc<Module>>,
}

struct Module {
    portlist_desc: Vec<Port>,
    layout_desc: Vec<Cell>,
}

struct Cell {
    ports: Vec<Port>,
    pub cell_type: Box<dyn CellType>,
}

impl Cell {
    fn new(cell_type: Box<dyn CellType>) -> Self {
        Self {
            ports: cell_type.ports(),
            cell_type: cell_type,
        }
    }

    fn link_port(
        &mut self,
        local: PortHandle,
        remote: (CellHandle, PortHandle),
    ) -> Result<(), LinkError> {
        let port_to_link = match self.ports.get_mut(local.0) {
            Some(val) => val,
            None => return Result::Err(LinkError::PortDoesNotExist),
        };

        match port_to_link {
            Port::Input(val) => {
                if let None = val {
                    *val = Some(remote);
                } else {
                    return Result::Err(LinkError::InputAlreadyDriven);
                }
            }

            Port::Output(val) => val.push(remote),
        };

        return Result::Ok(());
    }

    fn clear_port(&mut self, port_handle: PortHandle) {
        match self.ports.get_mut(port_handle.0) {
            Some(Port::Input(val)) => {
                *val = Option::None;
            }
            Some(Port::Output(val)) => {
                *val = Vec::new();
            }
            None => (),
        }
    }

    fn get_ports(&self) -> &Vec<Port> {
        &self.ports
    }
}

#[derive(Clone, Copy, Debug)]
struct CellHandle(usize);
#[derive(Clone, Copy, Debug)]
struct PortHandle(usize);

#[derive(Clone, Debug)]
enum Port {
    Input(Option<(CellHandle, PortHandle)>),
    Output(Vec<(CellHandle, PortHandle)>),
}

trait CellType {
    /// returns an empty/unlinked description of the CellType object's portlist
    fn ports(&self) -> Vec<Port>;

    /// returns a lowered description of the CellType object
    fn lower(&self) -> Result<Vec<Cell>, LowerError>;

    /// returns whether or not the CellType is a boundary of the child module
    /// if it is a boundary it provides the port which it is linked to in the parent.
    fn is_boundary(&self) -> Option<PortHandle>;

    /// returns whether or not this CellType is a primitive and what gate level operation
    /// it represents.  
    fn is_primitive(&self) -> Option<circuit::operation::Operation>;
}

enum LinkError {
    PortDoesNotExist,
    InputAlreadyDriven,
}
enum LowerError {}
