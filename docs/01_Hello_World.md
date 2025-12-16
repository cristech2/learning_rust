---
post_title: Hello World in Rust
author: Cristian Rosales
slug: hello-world-rust
categories: [Rust, Programming Basics]
tags: ["rust", "hello world", "programming"]
summary: Your first Rust program - understanding the basic structure and compilation process.
post_date: 2025-12-12
---

## Hello World in Rust

**Key Concept**: Your first Rust program. Learn how to write, compile, and run the simplest possible Rust code.

---

### 📚 Fundamental Concepts

- **The `main` Function**: Entry point of every Rust program where execution begins.
- **The `println!` Macro**: Built-in macro that prints text to the console.
- **Compilation**: Converting `.rs` source code into an executable using `rustc`.

---

### 💡 Practical Examples

#### Example: Simple Hello World Program

```rust
fn main() {
    println!("Hello, World!");
    println!("This is my first Rust program.");
}
```

**Explanation**: The `fn main()` defines where your program starts. The `println!` statements output text. Save this as `main.rs`, compile with `rustc main.rs`, and run with `./main`.

---

### ⚠️ Important Points

- **`fn` declares a function**: `main()` is the required entry point.
- **`println!` is a macro**: Indicated by the `!` at the end.
- **Rust requires compilation**: You must compile before running the program.

---

### 🔗 Relations and Context

**Prerequisites**:
- Rust compiler installed on your system (downloadable from [rust-lang.org](https://www.rust-lang.org/))
- Basic familiarity with a terminal/command prompt
- A text editor to write code

**Follow-up Topics**:
- Variables and data types (how to store and manipulate data)
- Functions with parameters and return values
- Control flow (if/else, loops)
- Ownership and borrowing (Rust's unique memory safety system)

---

### 📖 References

- [Rust Book - Chapter 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [Official Rust Installation Guide](https://www.rust-lang.org/tools/install)
- [Rust Standard Library Documentation - println!](https://doc.rust-lang.org/std/macro.println.html)