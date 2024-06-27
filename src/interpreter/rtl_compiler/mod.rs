// Converts source tokenstream into RTL representation for
// optimization and sanitization pre entry to graph gen interface
use crate::sim::circuit::operation::{Operation, SignalID};
struct Netlist {
    nets: Vec<Component>

}

struct ComponentID(usize);
struct PortID(usize);
enum Component {
    Module{
        // list which contains external component connections
        input_list: Box<[(String, ComponentID, PortID)]>,
        // list which contains connections to sub components 
        output_list: Box<[(String, ComponentID, PortID)]>,
        // description of subcomponents
        netlist: Box<[Component]>
    }
}

enum SensitivityType {
    Posedge(ComponentID, PortID),
    Negedge(ComponentID, PortID),
    LevelSensitive(ComponentID, PortID)
}