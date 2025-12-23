---
post_title: Packages, Crates, and Modules in Rust
author: Cristian Rosales
slug: packages-crates-modules-rust
categories: [Rust, Fundamentals]
tags: ["packages", "crates", "modules", "code-organization", "scope", "privacy"]
summary: Learn how to organize Rust projects using packages, crates, and modules to manage growing codebases while controlling scope and privacy.
post_date: 2025-12-23
---

## Packages, Crates, and Modules in Rust

**Key Concept**: Rust's module system organizes code into packages, crates, and modules to manage growing projects while controlling scope and privacy, ensuring code reusability and encapsulation through hierarchical organization.

---

### 📚 Fundamental Concepts

The module system is crucial for managing large Rust projects. Understanding the relationship between packages, crates, and modules enables you to write organized, maintainable code while controlling which parts are public and which are private.

- **Packages**: A bundle containing one or more crates, defined by a `Cargo.toml` file. This is the outermost organizational unit that you create with `cargo new`. A package is what users interact with when they use your code.
- **Crates**: A tree of modules that compiles into a library or executable. The compiler treats a crate as a single compilation unit. There are two types: binary crates (with a `main` function that produces an executable) and library crates (without `main`, providing reusable functionality).
- **Modules**: Named groups of code that control scope and privacy. They allow you to organize related functionality together and decide which items are public (accessible from outside the module) or private (internal implementation details).
- **Crate Roots**: The entry point files `src/main.rs` (for binary crates) or `src/lib.rs` (for library crates). These files form the root of the module tree for their respective crates.
- **Privacy Rules**: Code within a module is private by default. Use the `pub` keyword to make modules and items public. Child modules inherit their parent's privacy boundary.

#### The Module System Hierarchy

```
Package (Cargo.toml)
└── Crate (binary or library)
    └── Module Tree
        ├── Parent Module
        │   ├── Child Module
        │   ├── Items (fn, struct, enum, const, etc.)
        │   └── ...
        └── ...
```

---

### 💡 Practical Examples

#### Example 1: Understanding Packages and Crates Structure

When you create a new Rust project with `cargo new my_project`, Cargo generates:

```
my_project/
├── Cargo.toml          # Package manifest
└── src/
    └── main.rs         # Crate root for binary crate
```

**Explanation**: The `Cargo.toml` file defines the package. The `src/main.rs` file is the crate root of a binary crate with the same name as the package. When you compile this, `rustc` uses `src/main.rs` as the starting point.

---

#### Example 2: Package with Binary and Library Crates

A single package can contain both a binary crate and a library crate:

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs         # Binary crate root
    ├── lib.rs          # Library crate root
    └── other_module.rs
```

The binary crate (`src/main.rs`) can use code from the library crate (`src/lib.rs`):

```rust
// src/lib.rs
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// src/main.rs
use my_project::greet;

fn main() {
    greet("World");
}
```

**Explanation**: Both crates share the same package. The library crate exports the `greet` function with `pub`, making it available to the binary crate and external users.

---

#### Example 3: Organizing Code with Modules

Create a restaurant library with modules to organize front-of-house and back-of-house functionality:

```rust
// src/lib.rs
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::seat_at_table();
    
    // Using structs with public and private fields
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries"); // Error: field is private
}
```

**Explanation**: The module structure creates a hierarchy where `hosting` is a child of `front_of_house`, and `serving` is private (not accessible outside the parent module). The `Breakfast` struct has mixed visibility: `toast` is public, `seasonal_fruit` is private. Items are accessed using absolute paths (from the crate root) or relative paths (from the current module).

---

#### Example 4: Using Absolute and Relative Paths

```rust
// src/lib.rs
pub mod garden {
    pub mod vegetables {
        pub struct Asparagus;
    }
}

pub fn example() {
    // Absolute path - starts with 'crate'
    let plant1 = crate::garden::vegetables::Asparagus;
    
    // Relative path - starts from current module
    let plant2 = garden::vegetables::Asparagus;
}
```

**Explanation**: Absolute paths are more explicit and less fragile when code is moved around. Relative paths are more concise within the same module. Choose based on your needs.

---

#### Example 5: Multiple Binary Crates in One Package

Place additional binary files in `src/bin/`:

```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    └── bin/
        ├── tool1.rs    # Separate binary
        └── tool2.rs    # Separate binary
```

Compile and run individual binaries:

```bash
cargo run --bin tool1
cargo run --bin tool2
```

**Explanation**: Each file in `src/bin/` becomes a separate binary crate. This is useful for building related command-line tools within a single package.

---

### ⚠️ Important Points

Common pitfalls and critical concepts to master:

- **Default Privacy**: Modules and their contents are private by default. You must explicitly use `pub` to make them public. This principle of "deny by default" ensures you're intentional about your API surface.
- **Parent-Child Visibility**: Child modules can see items in their parents, but parents cannot see private items in children. This asymmetry protects encapsulation.
- **Re-exporting with `pub use`**: Use `pub use` to bring an item into your module's namespace and re-export it publicly. This is useful for creating convenient public APIs while organizing internal code differently:
  ```rust
  // Internal organization
  pub use crate::garden::vegetables::Asparagus;
  ```
- **Module File Organization**: You can define a module inline with `mod { }` or in a separate file. For larger modules, use separate files to keep code manageable.
- **Crate vs Package Confusion**: Remember that a package is the outer shell (defined by `Cargo.toml`), while crates are the compilation units inside. A package typically contains one library crate and zero or more binary crates.

---

### 🔗 Relations and Context

**Related Previous Topics**:
- This chapter builds on basic Rust syntax and functions from Chapters 1-5. You need to understand functions and ownership before effectively using modules.

**Prerequisites**:
- Basic Rust syntax and function definitions
- Understanding of variable scope in Rust
- Familiarity with `Cargo` and project structure from Chapter 1

**Follow-up Topics**:
- **Paths for Referring to Items**: The next section covers paths (absolute and relative) in detail, showing how to navigate the module tree.
- **Controlling Scope with `use`**: Learn how to bring items into scope to avoid repetitive path references.
- **Separating Modules into Different Files**: See how to organize large projects by splitting modules across files.
- **Workspaces**: In Chapter 14, learn how to manage multiple related packages together in a single workspace.

---

### 📖 References

- [Rust Book - Chapter 7: Packages, Crates, and Modules](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust Book - Chapter 7.1: Packages and Crates](https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html)
- [Rust Book - Chapter 7.2: Defining Modules to Control Scope and Privacy](https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- [Cargo Documentation - Projects](https://doc.rust-lang.org/cargo/guide/project-layout.html)
