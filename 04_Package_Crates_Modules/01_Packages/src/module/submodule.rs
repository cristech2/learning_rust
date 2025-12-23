//! Module to demostrate how Rust modules work.
//! This is a submodule within the `module` module.
//! It contains a simple function that can be called from other modules.

/// Function that prints a greeting message from the submodule.
pub fn greet_from_submodule() {
    println!("Wi, Welcome! i'm the submodule!");
}
