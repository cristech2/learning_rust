---
post_title: Functions in Rust
author1: Cristian Rosales
post_slug: functions-in-rust
microsoft_alias:
featured_image: 
categories: [Rust, Programming Basics, Tutorials]
tags: ["functions", "rust", "programming"]
ai_note: No
summary: A summary of knowledge learned about functions in Rust programming language.
post_date: 2025-12-12
---

## Flow Control in Rust

In Rust, flow control is managed through various constructs that allow you to dictate
the order in wich code is executed. This includes conditional statements and loops.

### Conditional Statements

Rust provides `if`, `else` and `else if` statements to execute code based on certain conditions.
The syntax is similar to other programming languages, but with some Rust-specific features.
`if` statements can also be used as expressions, meaning they can return values.
- `if` evaluates a condition and executes the block of code if the condition is true.
- `else` provides an alternative block of code if the condition is false.
- `else if` allows for multiple conditions to be checked in sequence.

#### Example of Conditional Statements

```rust
fn main() {
    let number = 7;
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is equal to 5");
    } else {
        println!("The number is greater than 5");
    }
}
```
---

### Loops
Rust provides three types of loops: `loop`, `while`, and `for`.
1. `loop`: This creates an infinite loop that will continue until explicitly broken out of using the `break` statement.
2. `while`: This loop continues to execute as long as a specified condition is true.
3. `for`: This loop iterates over a range or collection, executing the block of code for each item.

#### Example of Loops

```rust
fn main() {
    // Using loop
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
        println!("Count: {}", count);
    }  
}
```
If you forgot to include the break statement, the loop would run indefinitely.
Be careful when using `loop` to avoid infinite loops.

```rust
fn main() {
    // Using while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
}
```
```rust
fn main() {
    // Using labelled loops
    let mut count = 0;
    'outer: loop {
        println!("Count: {}", count);
        let mut inner_count = 10;
        loop {
            println!("Inner Count: {}", inner_count);
            if inner_count == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            inner_count -= 1;
        }
        count += 1;
    }
}   
```

```rust
fn main() {
    // Using for
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is: {}", element);
    }
}
```

---

### Conclusion

Rust's flow control constructs provide powerful tools for managing the execution of code.
Understanding how to use conditional statements and loops effectively is essential for writing
robust and efficient Rust programs. Remember to always consider edge cases and ensure
that your loops have proper termination conditions to avoid infinite loops.