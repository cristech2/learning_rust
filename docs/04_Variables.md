---
post_title: Variables on Rust
author1: Cristian Rosales
post_slug: variables-on-rust
microsoft_alias:
featured_image: 
categories: [Rust, Programming Basics, Tutorials]
tags: ["variables", "rust", "programming"]
ai_note: No
summary: A summary of knowledge learned about variables in Rust programming language.
post_date: 2025-12-12
---

## Variables on Rust

In Rust, variables are a fundamental part of the language, allowing you to store and manipulate data.
By default, variables in Rust are immutable, meaning their values cannot be changed once assigned.
However, you can make a variable mutable by using the `mut` keyword.
In addition, Rust have Constants, which are fixed values that cannot be changed and must be annotated with a type.
Here are some key points about variables in Rust:
1. **Declaration and Initialization**:
   You can declare and initialize a variable using the `let` keyword.
   ```rust
   let x = 5; // Immutable variable
   let mut y = 10; // Mutable variable
   ```
2. **Mutability**:
   By default, variables are immutable. To make a variable mutable, use the `mut` keyword.
   ```rust
   let mut z = 15;
   z = 20; // This is allowed
   ```
3. **Constants**:
   Constants are declared using the `const` keyword and must have a type annotation.
   ```rust
   const MAX_POINTS: u32 = 100_000;
   ```
4. **Shadowing**:
   Rust allows you to declare a new variable with the same name as a previous variable, 
   effectively "shadowing" the previous variable.
   ```rust
   let a = 5;
   let a = a + 1; // Shadows the previous 'a'
   ```
5. **Data Types**:
   Rust is a statically typed language, so you must specify the type of a variable if it cannot be inferred.
   ```rust
   let b: i32 = 10; // Explicit type annotation
   let c = 3.14; // Type inferred as f64
   ```
   - **Scalar Types**:
      - *Integers*: signed (i8, i16, i32, i64, i128, isize), unsigned( u8, u16, u32, u64, u128, usize)
      - *Floating-point numbers*: f32, f64
      - *Boolean*: bool (true or false)
      - *Character*: char (Unicode scalar value)
    - **Compound Types**:
      - *Tuples*: fixed-size collection of values of different types.
      - *Arrays*: fixed-size collection of values of the same type.
6. **Type Inference**:
   Rust can often infer the type of a variable based on the value assigned to it.
   ```rust
   let d = 42; // Inferred as i32
   let e = 3.14; // Inferred as f64
   ```
7. **Scope and Lifetime**:
   Variables in Rust have a scope, which is the region of the code where the variable is valid.
   When a variable goes out of scope, it is dropped and its memory is freed.
   ```rust
   {
       let f = 100; // f is valid within this block
   } // f goes out of scope here
   ```

Understanding how variables work in Rust is crucial for effective programming in the language.
By mastering variable declaration, mutability, constants, shadowing, and data types,
you can write more efficient and safe Rust code.