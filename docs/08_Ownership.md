---
post_title: OwnerShip in Rust
author1: Cristian Rosales
post_slug: 08_Ownership
microsoft_alias: 
featured_image: 
categories: [Rust, Programming, Memory Management]
tags: ["Ownership","memory management", "rust"]
ai_note: No
summary: An in-depth exploration into Rust's Ownership model, its principles, and how it ensures memory safety without a garbage collector.
post_date: 2025-12-13
---

## Understanding Ownership in Rust

Ownership is one of the most distinctive features of Rust, setting it apart from many other programming languages. It is a system that manages memory and ensures safety without the need for a garbage collector. In this article, we will explore the principles of ownership in Rust, how it works, and its implications for memory management.

### Stack vs. Heap Memory

Before diving into ownership, it's essential to understand the difference between stack and heap memory:
- **Stack Memory**: This is where stored data with a known, fixed size at compile time resides.
                    Variables like integers and booleans are stored on the stack. Stack memory is fast and automatically managed. Stack memory is fast and small in size.
- **Heap Memory**: This is used for data that can grow or shrink in size at runtime, such as vectors and strings. Heap memory is larger but requires manual management, which is where Rust's ownership model comes into play.

### How is memory handled when creating a complex data type like a String

When you create a complex data type like a `String`, Rust allocates memory on the heap to store the string data. The `String` type itself, however, is stored on the stack and contains a pointer to the heap memory, along with its length and capacity.
Looking at the following code:

```rust
let s1 = String::from("hello");
```
* **Stack**
  
|   name   | value      |
| :------: | :--------- |
|   ptr    | 0x01234567 |
|  length  | 5          |
| capacity | 5          |

* **Heap 0x01234567**
  
| index | value |
| :---: | :---: |
|   0   |   h   |
|   1   |   e   |
|   2   |   l   |
|   3   |   l   |
|   4   |   o   |

In this example, `s1` is a variable that owns the `String` data. The actual string data ("hello") is stored on the heap, while `s1` holds the pointer to that data, along with its length and capacity on the stack.

Now look at what happens when we assign `s1` to another variable `s2`:

```rust
let s2 = s1;
```
We might expect that both `s1` and `s2` would now point to the same string data on the heap.
However, in Rust, this is not the case. Instead, Rust performs a "move" operation. Rust transfers ownership of the string data from `s1` to `s2`. After this operation, `s1` is no longer valid, and attempting to use it will result in a compile-time error.

### The Three Rules of Ownership

Rust's ownership model is built on three fundamental rules:
1. **Each value in Rust has a variable that’s called its owner.**
2. **There can only be one owner at a time.**
3. **When the owner goes out of scope, the value will be dropped.**

These rules help Rust manage memory efficiently and prevent common issues like dangling pointers and memory leaks.\

#### Rule number 1: Each value in Rust has a variable that’s called its owner.

In Rust, every value has a single owner, which is the variable that holds the value.
For example:
```rust
let x = 5; // x is the owner of the value 5
let s = String::from("hello"); // s is the owner of the String "hello"
```
Never can two variables own the same value at the same time. If you try to assign one variable to another, Rust will move the ownership from the first variable to the second, and the first variable will no longer be valid.

#### Rule number 2: There can only be one owner at a time.

When you assign a value from one variable to another, Rust transfers ownership from the first variable to the second. The first variable becomes invalid and cannot be used anymore.
```rustlet s1 = String::from("hello");
let s2 = s1; // ownership of the String is moved from s1 to s2
// println!("{}", s1); // This line would cause a compile-time error because s1 is no longer valid
```
This rule helps prevent data races and ensures that there is always a clear owner for each value.

#### Rule number 3: When the owner goes out of scope, the value will be dropped.

When a variable goes out of scope, Rust automatically calls the `drop` function to free the memory associated with the value. This is done to prevent memory leaks.
```rust
{
    let s = String::from("hello"); // s is valid within this scope
} // s goes out of scope here, and the memory for the String is freed
```
This automatic memory management is a key feature of Rust's ownership model, ensuring that memory is efficiently used without the need for a garbage collector and minimizing the manual memory management required by the programmer avoiding common pitfalls like dangling pointers and memory leaks.

### Conclusion

Rust's ownership model is a powerful and unique feature that ensures memory safety and efficient memory management without the need for a garbage collector. By adhering to the three rules of ownership, Rust prevents common programming errors related to memory management, such as dangling pointers and memory leaks. Understanding and leveraging Rust's ownership system is crucial for writing safe and efficient Rust programs.