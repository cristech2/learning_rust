//! Ownership in Rust
//!
//! This module demonstrates the concept of ownership in Rust,
//! which is a key feature of the language that ensures memory safety
//! without needing a garbage collector.

/// A function that illustrates ownership with a String type.
/// When the function ends, the String is dropped and its memory is freed.
fn get_ownership_example(some_string: String) {
    // `some_string` comes into scope and takes ownership of the String
    println!("Hi!, i'm inside the function get_ownership_example()");
    println!("I'm the owner of the string: {}", some_string);
    println!("When this function ends, the string will be dropped.");
    // When the function ends, `some_string` goes out of scope and is dropped
}

/// A function that illustrates copying with a primitive type.
/// Primitive types like i32 implement the Copy trait, so they are copied rather than moved.
fn copy_example(number: i32) {
    // `number` is copied because i32 implements the Copy trait
    println!("The copied number is: {}", number);
}

/// Functions to demonstrate how by returning values, ownership can be transferred back.
/// This function creates a String and returns it, transferring ownership to the caller.
fn give_a_ownership() -> String {
    let some_string: String = String::from("Hello, I'm gifted to you!");
    println!("Hi!, i'm give_a_ownership(). I'm creating a string and giving you ownership of it.");
    println!("The string is: {}", some_string);
    println!("Enjoy your new string!");
    some_string // Ownership is transferred to the caller
}

/// Function that takes ownership of a String and then returns it back to the caller.
/// This allows the caller to regain ownership.
fn take_and_give_back_ownership(a_string: String) -> String {
    println!("Hi!, i'm take_and_give_back_ownership(). I'm taking ownership of your string.");
    println!("The string is: {}", a_string);
    println!("Now, I'm giving the ownership back to you.");
    println!("Enjoy your string again!. Thank you for trusting me with it.");
    a_string // Ownership is transferred back to the caller
}

fn main() {
    // First declare a String and demonstrate ownership
    let my_string: String = String::from("Hello, Rust!");
    println!("Original string: {}", my_string);
    get_ownership_example(my_string);
    // If we try to use `my_string` here, it would result in a compile-time error
    // e.g., println!("Trying to use my_string: {}", my_string);

    // Now demonstrate copying with a primitive type
    let my_number: i32 = 42;
    println!(
        "Original number before function copy_example() call: {}",
        my_number
    );
    copy_example(my_number); //This function just copies the value
    println!("Original number after function call: {}", my_number); // This is still valid

    // Demonstrate giving ownership
    let new_string: String = give_a_ownership();
    println!("Received string from give_a_ownership(): {}", new_string);

    // Demonstrate taking and giving back ownership
    let returned_string: String = take_and_give_back_ownership(new_string);
    println!(
        "Received string back from take_and_give_back_ownership(): {}",
        returned_string
    );
}
