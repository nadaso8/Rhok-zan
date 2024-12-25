// This module is related to the construction of a netlist using MLIR
// as the underlying SSA implementation. It Should be called by other
// modules within synth in order to construct and lower the user input
// source code into the necesarry gate level representation.

use melior::{
    dialect::{arith, cf, DialectRegistry},
    utility::register_all_dialects,
    Context,
};

/*fn generate() -> Context {
    let registry = DialectRegistry::new();
    register_all_dialects(&registry);

    let context = Context::new();
    context.append_dialect_registry(&registry);
    context.load_all_available_dialects();
    context
}

/// Test function: Checks which dialects have been loaded into memory.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialect_registration() {
        // Call the generate function to initialize the context with registered dialects
        let context = generate();

        // Verify that specific operations from key dialects are registered
        assert!(
            context.is_registered_operation("arith.addi"),
            "Expected 'arith.addi' operation to be registered (from 'arith' dialect)."
        );
        assert!(
            context.is_registered_operation("cf.cond_br"),
            "Expected 'cf.cond_br' operation to be registered (from 'cf' dialect)."
        );

        // Optional: Add checks for other operations from other dialects as needed
        println!("Verified that 'arith.addi' and 'cf.cond_br' operations are registered.");
    }
}*/
