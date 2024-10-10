//This module will eventualaly be for the synthesis and optimization of
// high level HDL code into GLD the graph structures used by the simulation code.

use std::collections::{HashMap, HashSet};

struct Design {
    netlists: HashMap<NetlistHandle, Netlist>,
    discontinuities: Vec<NetlistHandle>,
}
impl Design {
    fn new_netlist(&mut self, name: String) -> Result<(), NetlistErr> {
        let netlist = Netlist::new(name);
        let handle = self
            .discontinuities
            .pop()
            .unwrap_or_else(|| NetlistHandle(self.netlists.len() as usize));
        match self.netlists.insert(handle, netlist) {
            Some(_) => Result::Err(NetlistErr::ReUsedHandle),
            None => Result::Ok(()),
        }
    }
}
struct Netlist {
    components: HashMap<ComponentHandle, Component>,
    discontinuities: Vec<ComponentHandle>,
    inputs: HashSet<PortHandle>,
    name: String,
    outputs: HashSet<PortHandle>,
}

impl Netlist {
    fn new(name: String) -> Self {
        Self {
            components: HashMap::new(),
            discontinuities: Vec::new(),
            inputs: HashSet::new(),
            name,
            outputs: HashSet::new(),
        }
    }

    fn insert(&mut self, component: Component) -> Result<ComponentHandle, NetlistErr> {
        let handle = self
            .discontinuities
            .pop()
            .unwrap_or_else(|| ComponentHandle(self.components.len()));

        match component.variant {
            CompoenentType::Input(input_handle) => {
                self.inputs.insert(input_handle);
            }
            CompoenentType::Output(output_handle) => {
                self.outputs.insert(output_handle);
            }
            _ => (),
        }

        match self.components.insert(handle, component) {
            Some(_) => Result::Err(NetlistErr::ReUsedHandle),
            None => Result::Ok(handle),
        }
    }

    fn delete(&mut self, component: ComponentHandle) {
        todo!("Un-Link neighbors, remove component, and add handle to discontinuities for re-use")
    }

    fn get_mut_component(
        &mut self,
        component: ComponentHandle,
    ) -> Result<&mut Component, NetlistErr> {
        todo!()
    }
}

struct Component {
    variant: CompoenentType,
    input: HashMap<PortHandle, Option<Net>>,
    output: HashMap<PortHandle, Vec<Net>>,
}

impl Component {
    fn new(variant: CompoenentType) -> Self {
        match variant {
            CompoenentType::And
            | CompoenentType::Nand
            | CompoenentType::Or
            | CompoenentType::Nor
            | CompoenentType::Xor
            | CompoenentType::Xnor => {
                // most common case is boolean opperators so that pattern is uses as default
                let mut input = HashMap::new();
                input.insert(PortHandle(1), None);
                input.insert(PortHandle(2), None);
                let mut output = HashMap::new();
                output.insert(PortHandle(0), Vec::new());
                Component {
                    variant,
                    input,
                    output,
                }
            }
            CompoenentType::Not => {
                let mut input = HashMap::new();
                input.insert(PortHandle(1), None);
                let mut output = HashMap::new();
                output.insert(PortHandle(0), Vec::new());
                Component {
                    variant,
                    input,
                    output,
                }
            }
            CompoenentType::Input(port) => {
                let mut input = HashMap::new();
                let mut output = HashMap::new();
                output.insert(PortHandle(0), Vec::new());
                Component {
                    variant,
                    input,
                    output,
                }
            }
            CompoenentType::Output(port) => {
                let mut input = HashMap::new();
                input.insert(PortHandle(1), None);
                let mut output = HashMap::new();
                Component {
                    variant,
                    input,
                    output,
                }
            }
            CompoenentType::Module(_) => {
                // modules will not offer any ports until the "update_module_ports" method is called.
                // this is because the design netlists are not in scope at time of module component creation.
                let mut input = HashMap::new();
                let mut output = HashMap::new();
                Component {
                    variant,
                    input,
                    output,
                }
            }
        }
    }

    fn link(&mut self, port: PortHandle) {}

    fn update_module_ports(&mut self, desig: &Vec<Netlist>) {}
}

enum CompoenentType {
    Not,
    Nand,
    And,
    Nor,
    Or,
    Xor,
    Xnor,
    Input(PortHandle),
    Output(PortHandle),
    Module(NetlistHandle),
}

struct Net(ComponentHandle, PortHandle);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ComponentHandle(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PortHandle(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct NetlistHandle(usize);
enum NetlistErr {
    SomeErr,
    ReUsedHandle,
}

impl From<NetlistErr> for String {
    fn from(value: NetlistErr) -> Self {
        match value {
            NetlistErr::SomeErr => "Some Error".to_string(),
            NetlistErr::ReUsedHandle => "Re-used Handle".to_string(),
        }
    }
}
