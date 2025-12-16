---
post_title: Variables and Data Types in Rust
author: Cristian Rosales
slug: variables-data-types-rust
categories: [Rust, Fundamentals]
tags: ["variables", "mutability", "data-types", "shadowing"]
summary: Learn how to declare and manage variables in Rust, including mutability, shadowing, scalar types, and compound types.
post_date: 2025-12-13
---

## Variables and Data Types in Rust

**Key Concept**: Variables in Rust are immutable by default, but can be made mutable with the `mut` keyword. Rust supports both scalar types (integers, floats, booleans, characters) and compound types (tuples, arrays).

---

### 📚 Fundamental Concepts

- **Immutability by Default**: Variables are immutable unless explicitly declared with `mut`, promoting safe and predictable code
- **Shadowing**: Declaring a new variable with the same name as a previous variable, optionally changing its type while keeping the same logical name
- **Scalar Types**: Single-value types including integers (signed and unsigned), floating-point numbers, booleans, and Unicode characters
- **Compound Types**: Collections that group multiple values, such as tuples (fixed-size with mixed types) and arrays (fixed-size with single type)

---

### 💡 Practical Examples

#### Example 1: Mutable Variables

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This line will now work because x is mutable
    println!("The value of x is: {}", x);
}
```

**Explanation**: By default, variables are immutable. Using `mut` allows you to change the value after assignment. Without `mut`, attempting to reassign would cause a compile-time error.

---

#### Example 2: Shadowing

```rust
fn main() {
    let x = x + 3;
    println!("The value of x after shadowing is: {}", x);
    {
        let x = x * 3;
        println!("The value of x inside the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    // Shadowing aren't limited to the same data type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);
}
```

**Explanation**: Shadowing lets you reuse a variable name while optionally changing its type. This is useful when transforming values. The shadowed variable is only visible in its scope, and outer scopes retain their original bindings.

---

#### Example 3: Scalar Types

```rust
fn main() {
    // Integer types
    let unsigned_integer: u32 = 42;
    let signed_integer: i32 = -42;
    println!(
        "Unsigned integer: {}, Signed integer: {}",
        unsigned_integer, signed_integer
    );

    // Floating-point types
    let float64_number: f64 = 3.14;
    println!("Floating-point 64bit number: {}", float64_number);
    let float32_number: f32 = 2.71;
    println!("Floating-point 32bit number: {}", float32_number);

    // Boolean type
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
    let is_rust_boring: bool = false;
    println!("Is Rust boring? {}", is_rust_boring);

    // Character type
    let letter: char = 'R';
    let smile: char = '😊';
    println!("Character letter: {}, Character smile: {}", letter, smile);
}
```

**Explanation**: Scalar types are single values. Integer types include signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, u16, u32, u64, u128, usize) variants. f32 and f64 are for floating-point numbers. Booleans are `true` or `false`. Characters are four-byte Unicode scalar values.

| Type   | Size    | Range                           | Example               |
| ------ | ------- | ------------------------------- | --------------------- |
| `i32`  | 32-bit  | -2,147,483,648 to 2,147,483,647 | `let x: i32 = -42;`   |
| `u32`  | 32-bit  | 0 to 4,294,967,295              | `let x: u32 = 42;`    |
| `i64`  | 64-bit  | ±9,223,372,036,854,775,807      | `let x: i64 = 500;`   |
| `u8`   | 8-bit   | 0 to 255                        | `let x: u8 = 1;`      |
| `f64`  | 64-bit  | IEEE 754 double precision       | `let x: f64 = 3.14;`  |
| `f32`  | 32-bit  | IEEE 754 single precision       | `let x: f32 = 2.71;`  |
| `bool` | 1 byte  | `true` or `false`               | `let x: bool = true;` |
| `char` | 4 bytes | Unicode scalar value            | `let x: char = 'R';`  |

---

#### Example 4: Compound Types

```rust
fn main() {
    // Tuple type
    let tuple_example: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tuple_example; // Destructuring the tuple
    println!("The value of y from the tuple is: {}", y);
    println!("The first value of the tuple is: {}", tuple_example.0);

    // Array type
    let array_example: [u8; 5] = [1, 2, 3, 4, 5];
    let array_example2 = [3; 5];
    println!("The first element of the array is: {}", array_example[0]);
    println!(
        "The second element of the second array is: {}",
        array_example2[1]
    );
}
```

**Explanation**: Tuples group values of different types in a fixed-size collection. You access tuple elements by index (dot notation) or destructure them into separate variables. Arrays hold multiple values of the same type with fixed length notation `[type; length]`. Both are allocated on the stack.

---

### ⚠️ Important Points

- **Mutability is Explicit**: Rust requires `mut` keyword to modify variables, making intent clear and preventing accidental changes
- **Type Annotations Optional**: Rust's type inference is powerful, but explicit annotations improve code clarity and catch errors early
- **Shadowing vs Mutation**: Shadowing creates a new binding (potentially changing type), while mutation changes the existing value (same type)
- **Integer Overflow**: Debug mode panics on integer overflow; release mode wraps around. Be explicit about your needs
- **Scope Matters**: Variables are dropped when they go out of scope, freeing memory automatically
- **Fixed-Size Collections**: Tuples and arrays have fixed length and cannot be resized after creation

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [02_Hello_Cargo](02_Hello_Cargo.md) - Project structure basics
- [03_Guessing_Game](03_Guessing_Game.md) - Variables in context of interactive programs

**Prerequisites**:
- Basic understanding of Cargo projects
- Familiarity with function syntax

**Follow-up Topics**:
- [05_Functions](05_Functions.md) - Using variables in function parameters and returns
- [08_Ownership](08_Ownership.md) - How variables interact with Rust's ownership system
- [09_Borrow](09_Borrow.md) - Borrowing and references to variables

---

### 📖 References

- [Rust Book - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Rust Book - Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)