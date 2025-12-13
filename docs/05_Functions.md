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

## Functions in Rust

Functions in Rust are a fundamental building block of the language,
allowing you to encapsulate reusable code and organize your programs effectively.
Functions are defined using the `fn` keyword, followed by the function name,
a set of parentheses for parameters, and a block of code enclosed in curly braces.
Functions can take parameters, return values, and support various features.
The paremeters are defined within the parentheses, and you need to specify the type of each parameter.
Functions can return values using the `->` syntax followed by the return type.
Functions return the value of the last expression in the body without needing an explicit `return` statement,
although you can use `return` if you want to return early from a function.

### Defining a Function

Here is a simple example of defining and calling a function in Rust:

```rust
fn main() {
    print_hi();
}

fn print_hi() {
    println!("Hi!");
}
```

This code defines a function named `print_hi` that prints "Hi!" to the console.

Also, functions can take parameters:

```rust
fn main() {
    add(5, 3);
}

fn add(a: i32, b: i32) {
    println!("The sum is: {}", a + b);
}
```

### Parameters and Return Values

Function parameters must have their types explicitly defined.
Here is an example of a function that takes parameters witout return values:

```rust
fn main(){
    print_chars('A', 5);

}

fn print_chars(c: char, times: u32) {
    for _ in 0..times {
        println!("{}", c);
    }
}
```

In this example, the `print_chars` function takes a character and a number,
printing the character the specified number of times.

Functions can also return values. Here is an example of a function that returns a value:

```rust
fn main() {
    let result = multiply(4, 6);
    println!("The product is: {}", result);
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y // The last expression is returned
}
```

Also, you can use the `return` keyword to return a value explicitly:

```rust
fn main() {
    let result = divide(10, 2);
    println!("The quotient is: {}", result);
}

fn divide(x: i32, y: i32) -> i32 {
    return x / y;
}
```

### Conclusion

Functions in Rust are versatile and powerful, allowing you to create modular and reusable code.
By defining functions with parameters and return values, you can build complex programs
that are easy to read and maintain.