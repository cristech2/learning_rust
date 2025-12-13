//! A simple Rust program that converts Temperature Units.
//!
//! This program demonstrates basic Rust syntax and structure.
//! Converts temperatures between Celsius, Fahrenheit, and Kelvin.
use std::io::{self, Write}; //  Use this to handle user input and clear the console
//Global constants for temperature conversion
const SCALE_FACTOR: f64 = 1.8;
const FAHRENHEIT_OFFSET: f64 = 32.0;
const KELVIN_OFFSET: f64 = 273.15;

/// Converts Celsius to Fahrenheit.
///
/// # Arguments
/// * `celsius` - Temperature in Celsius.
///
/// # Returns
/// * Temperature in Fahrenheit.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Formula to convert Celsius to Fahrenheit
    (celsius * SCALE_FACTOR) + FAHRENHEIT_OFFSET
}

/// Converts Celsius to Kelvin.
///
/// # Arguments
/// * `celsius` - Temperature in Celsius.
///
/// # Returns
/// * Temperature in Kelvin.
fn celsius_to_kelvin(celsius: f64) -> f64 {
    // Formula to convert Celsius to Kelvin
    celsius + KELVIN_OFFSET
}

/// Converts Fahrenheit to Celsius.
///
/// # Arguments
/// * `fahrenheit` - Temperature in Fahrenheit.
///
/// # Returns
/// * Temperature in Celsius.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Formula to convert Fahrenheit to Celsius
    (fahrenheit - FAHRENHEIT_OFFSET) / SCALE_FACTOR
}

/// Converts Fahrenheit to Kelvin.
///
/// # Arguments
/// * `fahrenheit` - Temperature in Fahrenheit.
///
/// # Returns
/// * Temperature in Kelvin.
fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    // First convert Fahrenheit to Celsius, then Celsius to Kelvin
    let celsius = fahrenheit_to_celsius(fahrenheit);
    celsius_to_kelvin(celsius)
}

/// Converts Kelvin to Celsius.
///
/// # Arguments
/// * `kelvin` - Temperature in Kelvin.
///
/// # Returns
/// * Temperature in Celsius.
fn kelvin_to_celsius(kelvin: f64) -> f64 {
    // Formula to convert Kelvin to Celsius
    kelvin - KELVIN_OFFSET
}

/// Converts Kelvin to Fahrenheit.
///
/// # Arguments
/// * `kelvin` - Temperature in Kelvin.
///
/// # Returns
/// * Temperature in Fahrenheit.
fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    // First convert Kelvin to Celsius, then Celsius to Fahrenheit
    let celsius = kelvin_to_celsius(kelvin);
    celsius_to_fahrenheit(celsius)
}

/// Clear the console screen
fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Read the user input and check if it's a valid option
///
/// # Returns
/// * A u32 representing the user's choice.
fn read_user_choice() -> u32 {
    let mut choice: String = String::new();
    println!("Enter your choice:");
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return read_user_choice();
        }
    };
    choice
}

/// Read the temperature input from the user
///
/// # Returns
/// * A f64 representing the temperature input by the user.
fn read_temperature_input() -> f64 {
    let mut temp_input: String = String::new();
    io::stdin()
        .read_line(&mut temp_input)
        .expect("Failed to read line");
    let temperature: f64 = match temp_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return read_temperature_input();
        }
    };
    temperature
}

fn main() {
    // Flag to control the main loop
    let mut is_finished: bool = false;
    clear_screen();

    println!("------ Temperature Conversion ------");

    // Main loop to keep the program running until the user decides to exit
    while !is_finished {
        println!("------ Select Conversion Type ------");
        println!("1: Celsius to Fahrenheit");
        println!("2: Celsius to Kelvin");
        println!("3: Fahrenheit to Celsius");
        println!("4: Fahrenheit to Kelvin");
        println!("5: Kelvin to Celsius");
        println!("6: Kelvin to Fahrenheit");
        println!("0: Exit");

        // Read user choice
        let user_choice: u32 = read_user_choice();

        // Handle user choice only with IF statements
        if user_choice == 0 {
            is_finished = true;
            println!("------ Exiting Program. Goodbye! :) ------");
        } else if user_choice == 1 {
            println!("Enter temperature in Celsius:");
            let celsius: f64 = read_temperature_input();
            let fahrenheit = celsius_to_fahrenheit(celsius);
            println!("{celsius} °C is {fahrenheit} °F");
        } else if user_choice == 2 {
            println!("Enter temperature in Celsius:");
            let celsius: f64 = read_temperature_input();
            let kelvin = celsius_to_kelvin(celsius);
            println!("{celsius} °C is {kelvin} K");
        } else if user_choice == 3 {
            println!("Enter temperature in Fahrenheit:");
            let fahrenheit: f64 = read_temperature_input();
            let celsius = fahrenheit_to_celsius(fahrenheit);
            println!("{fahrenheit} °F is {celsius} °C");
        } else if user_choice == 4 {
            println!("Enter temperature in Fahrenheit:");
            let fahrenheit: f64 = read_temperature_input();
            let kelvin = fahrenheit_to_kelvin(fahrenheit);
            println!("{fahrenheit} °F is {kelvin} K");
        } else if user_choice == 5 {
            println!("Enter temperature in Kelvin:");
            let kelvin: f64 = read_temperature_input();
            let celsius = kelvin_to_celsius(kelvin);
            println!("{kelvin} K is {celsius} °C");
        } else if user_choice == 6 {
            println!("Enter temperature in Kelvin:");
            let kelvin: f64 = read_temperature_input();
            let fahrenheit = kelvin_to_fahrenheit(kelvin);
            println!("{kelvin} K is {fahrenheit} °F");
        } else {
            clear_screen();
            println!("Please enter a valid option from the menu!");
        }
    }
}
