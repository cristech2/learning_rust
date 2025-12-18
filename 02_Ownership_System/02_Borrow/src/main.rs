//! Borrow in Rust
//!     
//! This module demonstrates the concept of borrowing in Rust, which allows
//! multiple references to data without taking ownership. It covers both
//! immutable and mutable borrowing, along with the rules that govern them.

/// Immutable Borrowing Example
///
/// # Arguments
/// * `s` - A reference to a String
///
/// # Returns
/// * The length of the String
fn calculate_length(some_string: &String) -> usize {
    //& indicates a reference
    // Now we can use some_string without taking ownership of it.
    println!("Inside calculate_length: {}", some_string);
    some_string.len()
    // We can't modify some_string here because it's an immutable reference.
    // By default, references are immutable like a variable declared with "let".
} // We return the length of the String without taking ownership

fn modify_string(s: &mut String) {
    // We can modify the String because we have a mutable reference
    println!("Inside modify_string before modification: {}", s);
    println!("Modifying the string Muejeje...");
    // Append text to the String
    s.push_str(", dude!");
    println!("Now the string is: {}", s);
    println!("Godbye from modify_string!")
}

fn main() {
    // Create a String
    let mut my_string = String::from("hello");

    // Create an inmutable borrowing using a reference
    let reference1 = &my_string;
    // Now can create another inmutable borrowing
    let reference2 = &my_string;
    // We can have multiple inmutable references to the same data at the same time
    println!("Reference 1: {}", reference1);
    println!("Reference 2: {}", reference2);

    // let mutable_reference = &mut my_string;
    // We cannot have a mutable reference while we have immutable references
    // Uncommenting the next line will cause a compile-time error
    // let another_reference = &my_string; // Error!
    println!(" Using reference1 again: {}", reference1);
    // Create an inmutable length using a reference to my_string
    // Here, we are borrowing my_string immutably, DO NOT take ownership.
    let len = calculate_length(&my_string);

    // Now we can create a inmutable reference after the previous ones are no longer used.
    let mutable_reference = &mut my_string;
    // We can now use the mutable reference
    mutable_reference.push_str(" world");
    println!("After using mutable reference: {}", mutable_reference);
    //Now we can still use my_string because we only borrowed it.
    println!("The length of '{}' is {}.", my_string, len);

    // Mutable Borrowing Example
    println!("\n--- Mutable Borrowing Example ---");
    println!("The original string is: {}", my_string);
    // Borrow my_string mutably
    modify_string(&mut my_string);
    // Now we can see the modified string

    println!("The modified string is: {}", my_string);
}
