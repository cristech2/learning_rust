---
post_title: Practice 1 - Temperature Units Conversion
author: Cristian Rosales
slug: practice-1-temperature-conversion
categories: [Rust, Fundamentals]
tags: ["practice", "functions", "user-input", "control-flow", "constants"]
summary: Combine variables, functions, user input, loops, and conditionals to build a temperature conversion program that handles multiple unit conversions interactively.
post_date: 2025-12-13
---

## Practice 1: Temperature Units Conversion

**Key Concept**: This practice combines all concepts learned so far—functions with parameters and returns, global constants, user input handling, control flow with loops and conditionals, and input validation—into a complete interactive temperature conversion application.

---

### 📚 Fundamental Concepts

- **Global Constants**: Fixed values defined outside functions using `const` keyword that can be reused across multiple conversion functions (e.g., `SCALE_FACTOR`, `FAHRENHEIT_OFFSET`, `KELVIN_OFFSET`)
- **Reusable Functions**: Specialized functions for each conversion direction that encapsulate conversion logic and can be composed (e.g., `fahrenheit_to_kelvin` uses both `fahrenheit_to_celsius` and `celsius_to_kelvin`)
- **User Input Validation**: Recursive functions that re-prompt the user when input parsing fails, preventing the program from crashing on invalid data
- **Main Loop Control**: Using a `while` loop with a mutable boolean flag to control program flow and allow multiple operations until user chooses to exit
- **Conditional Logic**: Chained `if-else if` statements to route user choices to appropriate conversion functions

---

### 💡 Practical Examples

#### Example 1: Defining Global Constants and Simple Conversion Functions

```rust
const SCALE_FACTOR: f64 = 1.8;
const FAHRENHEIT_OFFSET: f64 = 32.0;
const KELVIN_OFFSET: f64 = 273.15;

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Formula to convert Celsius to Fahrenheit
    (celsius * SCALE_FACTOR) + FAHRENHEIT_OFFSET
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    // Formula to convert Celsius to Kelvin
    celsius + KELVIN_OFFSET
}
```

**Explanation**: Global constants make conversion formulas maintainable and consistent across functions. Each conversion function takes a temperature value and returns the converted result. Constants are reused across multiple functions, reducing duplication and making changes easier.

---

#### Example 2: Composing Functions for Multi-Step Conversions

```rust
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Formula to convert Fahrenheit to Celsius
    (fahrenheit - FAHRENHEIT_OFFSET) / SCALE_FACTOR
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    // First convert Fahrenheit to Celsius, then Celsius to Kelvin
    let celsius = fahrenheit_to_celsius(fahrenheit);
    celsius_to_kelvin(celsius)
}
```

**Explanation**: Larger conversions can be built by composing smaller functions. Instead of duplicating the conversion logic, `fahrenheit_to_kelvin` reuses existing functions. This follows the DRY (Don't Repeat Yourself) principle and makes the code maintainable.

---

#### Example 3: Reading and Validating User Input Recursively

```rust
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
```

**Explanation**: These functions read user input and validate it. If parsing fails, they recursively call themselves to re-prompt the user. The `match` expression handles both success (`Ok`) and failure (`Err`) cases. Validation prevents invalid data from breaking the program.

---

#### Example 4: Main Loop with Flag-Controlled Flow

```rust
fn main() {
    let mut is_finished: bool = false;

    println!("------ Temperature Conversion ------");

    while !is_finished {
        println!("------ Select Conversion Type ------");
        println!("1: Celsius to Fahrenheit");
        println!("2: Celsius to Kelvin");
        println!("3: Fahrenheit to Celsius");
        println!("4: Fahrenheit to Kelvin");
        println!("5: Kelvin to Celsius");
        println!("6: Kelvin to Fahrenheit");
        println!("0: Exit");

        let user_choice: u32 = read_user_choice();

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
            println!("Please enter a valid option from the menu!");
        }
    }
}
```

**Explanation**: The main function uses a `while` loop with a boolean flag `is_finished` to control program execution. Chained `if-else if` statements route each user choice to the appropriate conversion function. When the user enters 0, the flag is set to true, ending the loop. Invalid options prompt the user to try again without exiting the program.

---

### ⚠️ Important Points

- **Type Consistency**: All temperature values are `f64` to handle decimal conversions accurately. Integer values would lose precision
- **Flag-Controlled Loops**: Using a boolean flag to control loops is clear but can be replaced with `loop` and `break` for simpler logic
- **Input String Consumption**: The `read_line()` method includes the newline character, so `trim()` is necessary before parsing to avoid errors
- **Validation Before Conversion**: Always validate user input before passing to conversion functions. Garbage input should never reach your calculations
- **Function Composition**: Building complex conversions from simple ones reduces code duplication and makes changes easier to maintain

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [Variables and Mutability](02_Variables.md) - Variable declaration and mutability for storing input values
- [Functions](03_Functions.md) - Function definition with parameters and return values for conversions
- [Flow Control](04_Control_Flow.md) - Loops and conditionals to manage user interactions

**Prerequisites**:
- Understanding of variables and data types (especially `f64` for decimal numbers)
- Knowledge of function definition with parameters and return types
- Familiarity with `if-else if` chains and `while` loops
- Understanding of error handling with `match` expressions
- Basic knowledge of `std::io` for user input

**Follow-up Topics**:
- [Ownership](../02_Ownership_System/01_Ownership.md) - Understanding how ownership works with String input handling
- [Borrowing](../02_Ownership_System/02_Borrowing.md) - References and borrowing in more complex scenarios

---

### 📖 References

- [Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Standard Library - std::io](https://doc.rust-lang.org/std/io/)
- [Rust Book - Match Expressions](https://doc.rust-lang.org/book/ch06-02-match.html)

