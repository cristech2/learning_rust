//! # Flow Control
//!
//! This module demonstrates various flow control constructs in Rust,
//! including conditional statements and loops.
fn main (){
    let number: i32 = 4;

    // Conditional statement using if-else
    if number < 2 {
        println!("Condition was true, number is less than 2. Number: {}", number);
    }
    else {
        println!("Condition was false, number is 2 or greater. Number: {}", number);
    }

    else_if_example();
    usin_if_as_expression();
    infinite_loop_with_break_example();
    loop_with_return_value_example();
    labeled_loop_example();
    while_loop_example();
    for_loop_example();
}

/// Demonstrates the use of else if in conditional statements.
fn else_if_example() {
    let number: i32 = 16;

    // Using else if to check multiple conditions
    if number % 8 == 0 {
        println!("Number is divisible by 8. Number: {}", number);
    }
    else if number % 4 == 0 {
        println!("Number is divisible by 4. Number: {}", number);
    }
    else if number % 2 == 0 {
        println!("Number is divisible by 2. Number: {}", number);
    }
    else {
        println!("Number is not divisible by 4, 3, or 2. Number: {}", number);
    }
}
/// Demonstrates using if as an expression to assign a value.
fn usin_if_as_expression() {
    let condition: bool = true;

    // Using if as an expression to assign a value
    let number: i32 = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
/// Demostrates a infinite loop with break condition.
fn infinite_loop_with_break_example() {
    let mut count: i32 = 0;

    // Loop that runs indefinitely until a break condition is met
    loop {
        count += 1;

        if count == 4 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }
    println!("Final count after loop: {}", count);
}
/// Demonstrates a loop that returns a value upon breaking.
fn loop_with_return_value_example() {
    let mut counter: i32 = 0;

    // Loop that returns a value when it breaks
    let result: i32 = loop {
        counter += 2;
        if counter == 8 {
            break counter * 2; // Returning a value from the loop when breaking
            // if you use `return` here, it will exit the entire function
            // not just the loop. Be careful!
        }
    }; 
    println!("The result from the loop is: {}", result);
}
/// Function to demostrate a labeled loop.
fn labeled_loop_example() {
    let mut count: i32 = 0;
    'first_loop: loop{
        println!("Count: {}", count);
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break; // this breaks the inner loop
            }
            if count == 2 {
                break 'first_loop; // this breaks the outer loop
            }
            remaining -=1;
        }
        count +=1;
    }
}
/// Function to demonstrate a while loop.
fn while_loop_example() {
    let mut countdown: i32 = 4;

    // Using a while loop to count down
    while countdown != 0 {
        println!("Countdown: {}", countdown);

        countdown -= 1;
    }
    println!("BOOM!!!");
}
/// Function to demostrarte a for loop iterating over an array.
fn for_loop_example() {
    let counts: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    for count in counts {
        println!("Count: {}", count);
    }
}