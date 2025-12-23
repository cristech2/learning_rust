//! Module to demostrate how to create modules and submodules in Rust.
//!  This is a parent module that contains a submodule.
//! The submodule is defined in a separate file named `module/submodule.rs`.

pub mod submodule;

/// Function that prints a greeting message from the parent module.
pub fn greet_from_module() {
    println!("Hi, i'm the parent module!");
}
