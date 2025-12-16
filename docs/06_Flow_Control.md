---
post_title: Flow Control in Rust
author: Cristian Rosales
slug: flow-control-rust
categories: [Rust, Fundamentals]
tags: ["conditionals", "if-else", "loops", "loop-control"]
summary: Learn how to control program flow in Rust using conditional statements (if, else, else if), if expressions, and three types of loops (loop, while, for).
post_date: 2025-12-13
---

## Flow Control in Rust

**Key Concept**: Flow control in Rust allows you to execute code conditionally with `if`, `else`, and `else if`, and repeat code with three loop types: `loop` (infinite), `while` (conditional), and `for` (iterative).

---

### 📚 Fundamental Concepts

- **Conditional Statements**: `if`, `else`, and `else if` execute code blocks based on boolean conditions. Conditions don't require parentheses in Rust
- **If as Expression**: Unlike many languages, `if` can be used as an expression to assign values, returning the result of one of its branches
- **Loop Types**: Three loops serve different purposes—`loop` runs indefinitely until `break`, `while` continues while a condition is true, and `for` iterates over collections
- **Labeled Loops**: Nested loops can be labeled with a lifetime syntax (e.g., `'outer:`) to break from outer loops specifically

---

### 💡 Practical Examples

#### Example 1: Simple If-Else Statement

```rust
fn main() {
    let number: i32 = 4;

    if number < 2 {
        println!("Condition was true, number is less than 2. Number: {}", number);
    }
    else {
        println!("Condition was false, number is 2 or greater. Number: {}", number);
    }
}
```

**Explanation**: The `if` statement evaluates the condition `number < 2`. If true, it executes the first block; otherwise, it executes the `else` block. Note that the condition is a boolean expression without requiring parentheses.

---

#### Example 2: Else If for Multiple Conditions

```rust
fn else_if_example() {
    let number: i32 = 16;

    if number % 8 == 0 {
        println!("Number is divisible by 8. Number: {}", number);
    }
    else if number % 4 == 0 {
        println!("Number is divisible by 4. Number: {}", number);
    }
    else if number % 2 == 0 {
        println!("Number is divisible by 2. Number: {}", number);
    }
    else {
        println!("Number is not divisible by 4, 3, or 2. Number: {}", number);
    }
}
```

**Explanation**: `else if` allows checking multiple conditions in sequence. Each condition is evaluated until one is true, then that block executes. The final `else` handles the case where no conditions match.

---

#### Example 3: If as an Expression

```rust
fn usin_if_as_expression() {
    let condition: bool = true;

    let number: i32 = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

**Explanation**: `if` can be used as an expression to assign a value to a variable. The branches must return the same type. If `condition` is true, `number` gets 5; otherwise, it gets 6. This is more concise than a separate `if` statement.

---

#### Example 4: Infinite Loop with Break

```rust
fn infinite_loop_with_break_example() {
    let mut count: i32 = 0;

    loop {
        count += 1;

        if count == 4 {
            println!("Breaking the loop at count: {}", count);
            break;
        }
    }
    println!("Final count after loop: {}", count);
}
```

**Explanation**: The `loop` keyword creates an infinite loop. Inside the loop, we check a condition and use `break` to exit. Without `break`, the loop would run indefinitely. A `while` loop is cleaner when you have a clear termination condition.

---

#### Example 5: Loop Returning a Value

```rust
fn loop_with_return_value_example() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 2;
        if counter == 8 {
            break counter * 2;
        }
    };
    println!("The result from the loop is: {}", result);
}
```

**Explanation**: A loop can return a value using `break` with an expression. When `counter == 8`, we break and return `counter * 2` (which is 16). This value is assigned to `result`. Note that `return` would exit the entire function, not just the loop.

---

#### Example 6: Labeled Loops

```rust
fn labeled_loop_example() {
    let mut count: i32 = 0;
    'first_loop: loop {
        println!("Count: {}", count);
        let mut remaining: i32 = 10;

        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'first_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
}
```

**Explanation**: By default, `break` only exits the innermost (most immediate) loop it's in. In this example, the first `break` statement only exits the inner loop, not the outer one. To break from the outer loop, you need to label it with a lifetime-like syntax (`'first_loop:`) and specify which loop to break with `break 'first_loop`. This is why labels are useful in nested loops—without them, you can only break the immediately enclosing loop.

---

#### Example 7: While Loop

```rust
fn while_loop_example() {
    let mut countdown: i32 = 4;

    while countdown != 0 {
        println!("Countdown: {}", countdown);
        countdown -= 1;
    }
    println!("BOOM!!!");
}
```

**Explanation**: The `while` loop continues executing as long as the condition `countdown != 0` is true. It's cleaner than `loop` with `break` when you have a clear boolean condition to check before each iteration.

---

#### Example 8: For Loop Over Array

```rust
fn for_loop_example() {
    let counts: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];

    for count in counts {
        println!("Count: {}", count);
    }
}
```

**Explanation**: The `for` loop iterates over each element in an array. The variable `count` takes each value in turn. `for` is the safest loop for iterating collections because it can't go out of bounds like manual indexing.

---

### ⚠️ Important Points

- **Boolean Conditions**: `if` conditions must evaluate to a boolean; Rust won't automatically convert other types
- **Expression vs Statement**: The branches of an `if` expression must return the same type when used to assign values
- **Break vs Return**: `break` exits only the loop; `return` exits the entire function. Be careful not to confuse them in nested loops
- **Labeled Loops**: Labels only work with `break` and `continue`; they're useful for nested loops but can make code harder to follow
- **For Over While**: Prefer `for` loops when iterating collections; they're safer and more idiomatic than `while` with manual indices

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [04_Variables](04_Variables.md) - Variable types and mutability used in flow control
- [05_Functions](05_Functions.md) - Functions containing flow control logic

**Prerequisites**:
- Understanding of boolean values and comparison operators
- Familiarity with variables and variable types
- Knowledge of functions

**Follow-up Topics**:
- [07_Practice_1](07_Practice_1.md) - Combining flow control with previous concepts
- [08_Ownership](08_Ownership.md) - How flow control interacts with ownership rules

---

### 📖 References

- [Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust Book - If Expressions](https://doc.rust-lang.org/book/ch03-05-control-flow.html#if-expressions)
- [Rust Book - Loops](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loops)