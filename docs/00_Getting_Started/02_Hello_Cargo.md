---
post_title: Hello World with Rust using Cargo
author: Cristian Rosales
slug: hello-world-rust-cargo
categories: [Rust, Fundamentals]
tags: ["cargo", "hello-world", "setup"]
summary: Learn how to create and run your first Rust program using Cargo, the Rust package manager and build system.
post_date: 2025-12-12
---

## Hello World with Rust using Cargo

**Key Concept**: Cargo is Rust's package manager and build system that simplifies project creation, compilation, and execution of Rust programs.

---

### 📚 Fundamental Concepts

- **Cargo Projects**: A standardized project structure with `Cargo.toml` metadata file and organized source code directories
- **Cargo.toml**: A manifest file containing project metadata, version, and dependencies
- **Building and Running**: Using `cargo build` to compile and `cargo run` to execute your program

---

### 💡 Practical Examples

#### Example 1: Basic Hello World Program

```rust
fn main() {
    println!("Hello, world!");
}
```

**Explanation**: This is the simplest Rust program. The `main` function is the entry point where program execution begins. The `println!` macro prints text to the console and adds a newline.

---

#### Example 2: Multiple Output Lines

```rust
fn main() {
    println!("Hello, world!");
    println!("Welcome to Rust programming.");
}
```

**Explanation**: You can call `println!` multiple times to print different messages. Each call produces output on a separate line, making it easy to display multiple lines of text.

---

### ⚠️ Important Points

- **Function Declaration**: Every Rust program needs a `main` function where execution starts
- **Macro Syntax**: `println!` is a macro (denoted by the `!`), not a regular function
- **Semicolons**: Each statement must end with a semicolon (`;`)
- **Cargo Project Structure**: Always use `cargo new` to create projects so the structure is correct and ready to build

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [Hello World](01_Hello_World.md) - Running Rust without Cargo

**Prerequisites**:
- Rust and Cargo installed on your machine

**Follow-up Topics**:
- [Guessing Game](../01_Fundamentals/01_Guessing_Game.md) - Interactive programs and user input

---

### 📖 References

- [Rust Book - Hello, Cargo!](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)