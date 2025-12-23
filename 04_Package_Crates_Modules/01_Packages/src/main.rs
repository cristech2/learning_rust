//! Binary crate for demonstrating packages, crates, and modules in Rust.
//! This is a practical example to illustrate how to structure a Rust project.
// Importing the library crate defined in the same package.
// This syntax are used to reduce the need for repetitive long paths.
// We use the path::{module1, module2} to import multiple modules in one line.
// also we can use two lines to import modules and submodules separately.
use packages_crates_modules::{module, module::submodule};

fn main() {
    // Calling a function from the module.
    module::greet_from_module();

    // Calling a function from the submodule.
    submodule::greet_from_submodule();
}
