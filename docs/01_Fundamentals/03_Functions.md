---
post_title: Functions in Rust
author: Cristian Rosales
slug: functions-rust
categories: [Rust, Fundamentals]
tags: ["functions", "parameters", "return-values", "expressions"]
summary: Learn how to define and use functions in Rust, including parameters, return values, and the difference between implicit and explicit returns.
post_date: 2025-12-13
---

## Functions in Rust

**Key Concept**: Functions in Rust are defined with the `fn` keyword and can accept typed parameters and return values using the `->` syntax. Rust functions return the last expression implicitly without needing an explicit `return` statement.

---

### 📚 Fundamental Concepts

- **Function Definition**: Functions are declared with `fn` keyword, followed by name, parameters in parentheses, and a code block in curly braces
- **Parameters with Types**: Each parameter must have its type explicitly specified; parameters are comma-separated (e.g., `num1: i32, num2: i32`)
- **Return Type Syntax**: Functions specify their return type using `->` followed by the type (e.g., `-> i32`). The last expression is returned automatically
- **Implicit vs Explicit Returns**: Expressions without semicolons return their value automatically; use `return` keyword for explicit early returns

---

### 💡 Practical Examples

#### Example 1: Function Without Parameters or Return Value

```rust
fn main() {
    println!("This is the main function.");
    print_message();
}

fn print_message() {
    println!("This is a message from the print_message function.");
}
```

**Explanation**: The `print_message` function takes no parameters and returns nothing. It simply executes the code inside its body. Functions are called using their name followed by parentheses.

---

#### Example 2: Function With Parameters

```rust
fn main() {
    print_number(33);
}

fn print_number(num: i32) {
    println!("The number is: {}", num);
}
```

**Explanation**: The `print_number` function takes a single parameter `num` of type `i32`. The parameter must have its type explicitly specified. When calling the function, you provide the argument value.

---

#### Example 3: Function With Return Value (Implicit)

```rust
fn main() {
    println!("The sum of 6 and 5 is: {}", add_numbers(6, 5));
}

fn add_numbers(num1: i32, num2: i32) -> i32 {
    num1 + num2 //the last expression is returned without a semicolon
}
```

**Explanation**: The `add_numbers` function takes two `i32` parameters and returns an `i32`. The return type is specified with `->`. The last expression `num1 + num2` is returned automatically because it has no semicolon. A semicolon would make it a statement instead of an expression, causing a compilation error.

---

#### Example 4: Function With Return Value (Explicit)

```rust
fn main() {
    println!("The sum of 10 and 15 is: {}", add_numbers_explicit_return(10, 15));
}

fn add_numbers_explicit_return(num1: i32, num2: i32) -> i32 {
    return num1 + num2; //using explicit return statement
}
```

**Explanation**: This function achieves the same result using an explicit `return` statement. Both approaches work; the implicit return (Example 3) is more idiomatic in Rust and is preferred when returning the last value of a function.

---

### ⚠️ Important Points

- **Semicolons Matter**: An expression without a semicolon returns its value; a statement with a semicolon returns nothing (unit type `()`)
- **Type Annotations Required**: All parameters must have explicit type annotations; Rust does not infer parameter types
- **Return Type on Signature**: When a function returns a value, the return type must be declared in the function signature with `->`
- **Last Expression Returns**: The final expression in a function body is automatically returned, making `return` optional for the final value
- **Function Order**: Functions can be defined before or after `main()` in the same file; Rust doesn't care about declaration order

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [Variables and Mutability](02_Variables.md) - Understanding variable types used in function parameters
- [Hello Cargo](../00_Getting_Started/02_Hello_Cargo.md) - Basic program structure

**Prerequisites**:
- Understanding of variables and data types
- Familiarity with the main function concept

**Follow-up Topics**:
- [Flow Control](04_Control_Flow.md) - Control flow within function bodies
- [Ownership](../02_Ownership_System/01_Ownership.md) - How function parameters interact with ownership

---

### 📖 References

- [Rust Book - Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Rust Book - Function Parameters](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#parameters)
- [Rust Book - Return Values](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#return-values)