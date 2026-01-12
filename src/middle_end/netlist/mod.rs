/*
Beware all ye mortals who enter here I this file is born from the ravings of a madwoman
long since lost to the abominiation of the elder gods which is recursively flattening
a the farce of a netlist which was the graph structure designed by her own huberis.
There is no god and no master in this domain. Simply the horrors and turmoil of handle
translation.
*/

use crate::back_end::circuit::operation::{CircuitInput, CircuitOutput};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug)]
pub struct Netlist {
    pub root_module: ModuleHandle,
    pub modules: HashMap<ModuleHandle, Module>,
}

impl Netlist {
    /// Returns the number of primitive variant Cells that are present in the netlist.
    /// this should be equivalent to cell_layout().enumerate().last().0 + 1 but with
    /// a slightly more work efficient implementation
    fn count_primitives(&self) -> NetlistSize {
        let mut counts: Vec<(ModuleHandle, usize, HashMap<ModuleHandle, usize>)> = self
            .modules
            .iter()
            .map(|(handle, module)| {
                let mut primitives: usize = 0;
                let mut sub_modules: HashMap<ModuleHandle, usize> = HashMap::new();
                for (_, cell) in &module.cells {
                    match cell {
                        Cell::Primitive(_) => primitives += 1,
                        Cell::ModuleLink(link) => {
                            if let Some(value) = sub_modules.get_mut(&link) {
                                *value += 1;
                            } else {
                                sub_modules.insert(*link, 1);
                            }
                        }
                        Cell::InputProxy(_) => (),
                    }
                }
                (*handle, primitives, sub_modules)
            })
            .collect();

        while (counts.len() > 1) {
            let (index, (terminal_module, terminal_primitives, _)) = match counts
                .iter()
                .enumerate()
                .find(|(_, (.., sub_modules))| sub_modules.is_empty())
            {
                Some((index, terminal)) => (index, terminal.clone()),
                None => return NetlistSize::Indefinite(),
            };

            for (_, primitives, sub_modules) in &mut counts {
                match sub_modules.remove(&terminal_module) {
                    Some(module_instance_count) => {
                        *primitives += terminal_primitives * module_instance_count;
                    }
                    None => (),
                };
            }

            counts.remove(index);
        }

        assert_eq!(counts.len(), 1);
        let (final_module, final_count, final_sub_modules) = counts.first().unwrap();

        assert_eq!(*final_module, self.root_module);
        assert!(final_sub_modules.is_empty());

        return NetlistSize::Definite(*final_count);
    }
    /// Returns an iterator over each location in a netlist which contains a primitive gate type.
    ///
    fn cell_layout(&self) -> CellLayoutIterator {
        CellLayoutIterator {
            source_netlist: &self,
            module_iterator_stack: Vec::from([self
                .modules
                .get(&self.root_module)
                .unwrap()
                .cells
                .iter()]),
            cursor: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum NetlistSize {
    Definite(usize),
    Indefinite(),
}
/// An iterator over all locations in a netlist as a
struct CellLayoutIterator<'a> {
    source_netlist: &'a Netlist,
    module_iterator_stack: Vec<std::collections::hash_map::Iter<'a, CellHandle, Cell>>,
    cursor: Vec<CellHandle>,
}

impl Iterator for CellLayoutIterator<'_> {
    type Item = Vec<CellHandle>;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(iterator) = self.module_iterator_stack.last_mut() {
            match iterator.next() {
                Some((cell_handle, cell)) => {
                    // If the cursor is ever longer than the module iterator stack there's a
                    // logical error that could cause cascading errors. it's best to panic early.
                    assert!(self.module_iterator_stack.len() >= self.cursor.len());

                    match cell {
                        Cell::ModuleLink(module_handle) => {
                            // new module found update cursor with cell_handle for ModuleLink Cell
                            // then push it's iterator onto stack.
                            if self.module_iterator_stack.len() == self.cursor.len() {
                                self.cursor.pop();
                            }
                            self.cursor.push(*cell_handle);
                            self.module_iterator_stack.push(
                                self.source_netlist
                                    .modules
                                    .get(module_handle)
                                    .unwrap()
                                    .cells
                                    .iter(),
                            );
                            continue;
                        }
                        Cell::Primitive(_) => {
                            // Next primitive found update cursor and return next entry
                            if self.module_iterator_stack.len() == self.cursor.len() {
                                self.cursor.pop();
                            }
                            self.cursor.push(*cell_handle);
                            return Some(self.cursor.clone());
                        }
                        Cell::InputProxy(_) => continue, // Proxies arent instantiable and should be skipped
                    }
                }
                None => {
                    // end of module continue one level down the stack
                    self.cursor.pop();
                    self.module_iterator_stack.pop();
                    continue;
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct Module {
    name: String,
    portlist: Vec<Port>,
    wires: HashMap<Drain, Source>,
    cells: HashMap<CellHandle, Cell>,
}

impl Module {
    fn new(name: String) -> Self {
        Self {
            name: name,
            portlist: Vec::new(),
            wires: HashMap::new(),
            cells: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Cell {
    ModuleLink(ModuleHandle),
    Primitive(PrimitiveType),
    InputProxy(PortHandle),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ModuleHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CellHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PortHandle(usize);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Address(CellHandle, PortHandle);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Source(Address);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Drain(Address);

#[derive(Clone, Debug)]
pub struct Port {
    name: String,
    port_type: PortType,
    local_location: Address,
}

#[derive(Clone, Copy, Debug)]
pub enum PortType {
    Input,
    Output,
}

#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Not,
    And,
    Nand,
    Or,
    Nor,
    Xor,
    Xnor,
    Input(),
    Output(),
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        thread::panicking,
    };

    use crate::middle_end::netlist::{Netlist, NetlistSize};

    use super::{Cell, CellHandle, Module, ModuleHandle};
    /// This just de-dublicates some of my test data
    fn test_netlist() -> Netlist {
        Netlist {
            root_module: ModuleHandle(0),
            modules: HashMap::from_iter(
                Vec::from([
                    (
                        ModuleHandle(0),
                        Module {
                            name: "A".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(4327980),
                                        Cell::ModuleLink(super::ModuleHandle(1)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                    (
                                        CellHandle(130945672037),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                    (
                                        CellHandle(50978234790),
                                        Cell::Primitive(super::PrimitiveType::Xnor),
                                    ),
                                    (
                                        CellHandle(4325630976),
                                        Cell::ModuleLink(super::ModuleHandle(1)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                    (
                        ModuleHandle(1),
                        Module {
                            name: "B".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(4327980),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                    (
                                        CellHandle(130945672037),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                    (
                        ModuleHandle(2),
                        Module {
                            name: "C".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                ])
                .into_iter(),
            ),
        }
    }

    /// Returns a netlist which is infinite in size due to a cyclic module reference
    fn indefinite_netlist() -> Netlist {
        Netlist {
            root_module: ModuleHandle(0),
            modules: HashMap::from_iter(
                Vec::from([
                    (
                        ModuleHandle(0),
                        Module {
                            name: "A".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(4327980),
                                        Cell::ModuleLink(super::ModuleHandle(1)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                    (
                                        CellHandle(130945672037),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                    (
                                        CellHandle(50978234790),
                                        Cell::Primitive(super::PrimitiveType::Xnor),
                                    ),
                                    (
                                        CellHandle(4325630976),
                                        Cell::ModuleLink(super::ModuleHandle(1)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                    (
                        ModuleHandle(1),
                        Module {
                            name: "B".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(4327980),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                    (
                                        CellHandle(130945672037),
                                        Cell::ModuleLink(super::ModuleHandle(2)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                    (
                        ModuleHandle(2),
                        Module {
                            name: "C".to_string(),
                            portlist: Vec::new(),
                            wires: HashMap::new(),
                            cells: HashMap::from_iter(
                                Vec::from([
                                    (
                                        CellHandle(473980281),
                                        Cell::InputProxy(super::PortHandle(0)),
                                    ),
                                    (
                                        CellHandle(0796596123),
                                        Cell::Primitive(super::PrimitiveType::And),
                                    ),
                                    (
                                        CellHandle(2039845657),
                                        Cell::InputProxy(super::PortHandle(1)),
                                    ),
                                    (
                                        CellHandle(130945672037),
                                        Cell::ModuleLink(super::ModuleHandle(1)),
                                    ),
                                ])
                                .into_iter(),
                            ),
                        },
                    ),
                ])
                .into_iter(),
            ),
        }
    }

    #[test]
    fn iterator_test() {
        let test_netlist = test_netlist();
        let mut expected_result = Vec::from([
            Vec::from([CellHandle(4327980), CellHandle(796596123)]),
            Vec::from([
                CellHandle(4327980),
                CellHandle(130945672037),
                CellHandle(796596123),
            ]),
            Vec::from([
                CellHandle(4327980),
                CellHandle(4327980),
                CellHandle(796596123),
            ]),
            Vec::from([CellHandle(50978234790)]),
            Vec::from([CellHandle(796596123)]),
            Vec::from([CellHandle(130945672037), CellHandle(796596123)]),
            Vec::from([CellHandle(4325630976), CellHandle(796596123)]),
            Vec::from([
                CellHandle(4325630976),
                CellHandle(130945672037),
                CellHandle(796596123),
            ]),
            Vec::from([
                CellHandle(4325630976),
                CellHandle(4327980),
                CellHandle(796596123),
            ]),
        ]);

        for (path_no, path) in test_netlist.cell_layout().enumerate() {
            // visualization
            print!("path({path_no}) = [");
            for (cell_handle_no, cell_handle) in path.iter().enumerate() {
                print!("{}", cell_handle.0);
                if cell_handle_no < path.len() - 1 {
                    print!(", ");
                }
            }
            println!("]");

            // actual testing against expected results
            let expected_index = expected_result
                .iter()
                .position(|x| path == *x)
                .expect("Last path was not in expected result");
            expected_result.remove(expected_index);
        }

        // all expected results should be consumed after test is done
        assert!(expected_result.is_empty())
    }

    #[test]
    fn primitive_count_test_hard_coded() {
        assert_eq!(NetlistSize::Definite(9), test_netlist().count_primitives())
    }

    #[test]
    fn infinite_netlist_test() {
        assert_eq!(
            NetlistSize::Indefinite(),
            indefinite_netlist().count_primitives()
        )
    }
}
