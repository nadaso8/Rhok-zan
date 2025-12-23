/*
Thoughts on redesign: Implement the into <backend::circuit::Circuit> trait for the intermediate representation
This should be possible by doing a planning/reserve phase followed by an assignment phase. The planning phase is
used to calculate the size of the end circuit by taking the sum of the size of every cell contained in the top
level module. This is a recursive process meaning that the size must be calculated for all sub modules. Once
the overall size is known the resulting array can be annotated dividing it up into what series of Cell handles are
related to any given index in the overall array. This series of Cell handles from the higherarchical representation
should function as a unique global identifier, and used to convert from the netlist address space to the end circuit
address space. The original higherarchical description can be iterated over treating each location as the description
for the contents at that location. By follwoing the dependency chain back along the higherarchical representation
recursing (up for encountered input nodes or down for module ouptuts) back to the first encountered primitive
connected to each input. The tripplet of the global identifier for each location as well as the input global
identifiers may be converted into the address space of the end circuit based on the conversion table produced
earlier in the reservation phase. At any point if an input isn't found it should instead point to an input primitive
which will always use a high impedance signal this should be easy to insert at the last index of the circuit.
*/

impl From<crate::middle_end::netlist::Netlist> for crate::back_end::circuit::Circuit {
    fn from(value: crate::middle_end::netlist::Netlist) -> Self {}
}
