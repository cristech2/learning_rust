---
post_title: Guessing Game - Interactive Program in Rust
author: Cristian Rosales
slug: guessing-game-rust
categories: [Rust, Fundamentals]
tags: ["user-input", "loops", "pattern-matching", "error-handling"]
summary: Learn how to build an interactive command-line guessing game using user input, random number generation, loops, and pattern matching.
post_date: 2025-12-12
---

## Guessing Game - Interactive Program in Rust

**Key Concept**: A guessing game teaches multiple core Rust concepts by combining user input handling, random number generation, control flow with loops and pattern matching, and error handling in a single interactive program.

---

### 📚 Fundamental Concepts

- **User Input with `std::io`**: Reading and storing text input from the user using `stdin().read_line()`
- **Random Numbers with `rand` Crate**: Generating random numbers within a range using `thread_rng().gen_range()`
- **Pattern Matching with `match`**: Comparing values and responding to different outcomes with `cmp()` and `Ordering`
- **Infinite Loops with `loop`**: Creating loops that continue until a break statement is encountered
- **Error Handling with `Result`**: Managing parsing errors gracefully using `match` with `Ok` and `Err` variants

---

### 💡 Practical Examples

#### Example 1: Generating a Random Number

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("I'm thinking of a number between 1 and 100");
    println!("Secret (debug): {}", secret_number);
}
```

**Explanation**: This creates a random number between 1 and 100. The `thread_rng()` function gets the random number generator, and `gen_range(1..=100)` generates a value in that range. The secret is printed for demonstration purposes.

---

#### Example 2: Reading User Input

```rust
use std::io;

fn main() {
    println!("Please input the number you guess:");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
```

**Explanation**: This creates a mutable `String` to store user input. `read_line()` reads from standard input and appends to the string. The `&mut` syntax is necessary because we're borrowing the string mutably to modify it.

---

#### Example 3: Comparing Guesses with Pattern Matching

```rust
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = 50;
    
    println!("Please input your guess:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

**Explanation**: The guess is parsed from a string to a `u32` number. The `cmp()` method compares it to the secret and returns an `Ordering`. The `match` expression handles all three outcomes: Less, Greater, or Equal.

---

#### Example 4: Complete Game with Loop and Error Handling

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number if you can! :)");
    
    loop {
        println!("Please input the number you guess:");
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! Good job!");
                break;
            }
        }
    }
}
```

**Explanation**: The complete game uses a `loop` that continues until the user guesses correctly and triggers `break`. Inside the loop, error handling with `match` validates the input. Invalid inputs print an error and use `continue` to skip to the next iteration. Valid guesses are compared to the secret number, and the game ends when they match.

---

### ⚠️ Important Points

- **Mutable Strings**: Variables must be declared with `mut` to be modified, including strings that store user input
- **Type Conversion**: The `parse()` method converts strings to numbers and returns a `Result` that must be handled
- **Error Handling**: Use `match` with `Ok` and `Err` to gracefully handle parsing failures instead of panicking
- **String Cleanup**: The `trim()` method removes whitespace and newlines from input before parsing
- **Infinite Loops**: The `loop` keyword creates an infinite loop that only exits with `break`
- **External Crates**: The `rand` crate must be added to `Cargo.toml` as a dependency before use

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [Hello Cargo](../00_Getting_Started/02_Hello_Cargo.md) - Project structure and Cargo basics
- [Variables and Mutability](02_Variables.md) - Variable declaration and mutability
- [Functions](03_Functions.md) - Function definition and organization

**Prerequisites**:
- Understanding of Cargo projects and project structure
- Basic understanding of variables and types
- Familiarity with simple expressions and output

**Follow-up Topics**:
- [Flow Control](04_Control_Flow.md) - Advanced control flow patterns
- [Ownership](../02_Ownership_System/01_Ownership.md) - Deeper understanding of borrowing in complex scenarios

---

### 📖 References

- [Rust Book - Guessing Game Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [Rand Crate Documentation](https://docs.rs/rand/latest/rand/)
- [Standard Library - std::io](https://doc.rust-lang.org/std/io/)
- [Standard Library - std::cmp::Ordering](https://doc.rust-lang.org/std/cmp/enum.Ordering.html)