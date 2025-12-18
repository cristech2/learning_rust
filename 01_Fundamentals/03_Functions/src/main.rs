//! # Functions
//!
//! This is an example of defining and using functions in Rust.

fn main() {

    println!("This is the main function.");
    print_message();
    print_number(33);
    println!("The sum of 6 and 5 is: {}", add_numbers(6, 5));
    println!("The sum of 10 and 15 is: {}", add_numbers_explicit_return(10, 15));
}

/// Just another function that prints a message to the console.
fn print_message() {
    println!("This is a message from the print_message function.");
}

/// Function that print a number.
/// 
/// # Arguments
///
/// * `num` - An integer number to be printed.
fn print_number(num: i32) {
    println!("The number is: {}", num);
}

/// Function that adds two numbers and returns the result.
///
/// # Arguments
/// * `num1` - The first integer number.
/// * `num2` - The second integer number.
///
/// # Returns
/// * The sum of `num1` and `num2`.
fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2 //the last expression is returned without a semicolon
}


/// Function that adds two numbers and returns the result.
///
/// # Arguments
/// * `num1` - The first integer number.
/// * `num2` - The second integer number.
///
/// # Returns
/// * The sum of `num1` and `num2`.
fn add_numbers_explicit_return(num1: i32, num2: i32) -> i32 {
    return num1 + num2; //using explicit return statement
}
