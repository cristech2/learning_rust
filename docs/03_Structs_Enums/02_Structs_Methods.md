---
post_title: Struct Methods in Rust
author: Cristian Rosales
slug: struct-methods-rust
categories: [Rust, Advanced Concepts]
tags: ["structs", "methods", "impl blocks", "associated functions"]
summary: Learn about struct methods in Rust, including method syntax, implementation blocks, and associated functions for creating organized and ergonomic code.
post_date: 2025-12-18
---

## Struct Methods in Rust

**Key Concept**: Methods are functions associated with structs that operate on instances of the struct. They are defined within `impl` blocks and provide a more organized and ergonomic way to interact with your custom types compared to standalone functions.

---

### 📚 Fundamental Concepts

Methods bring behavior to your structs by allowing you to define functions that operate on instances of your type. They organize code logically by grouping related operations together.

- **Methods**: Functions that operate on instances of a struct, always taking `self` as their first parameter. They use dot notation for calling (e.g., `instance.method()`).
- **The `impl` Block**: Short for "implementation block", where you define methods and associated functions for a struct. All functions within an `impl` block are associated with that struct type.
- **Associated Functions**: Functions defined within an `impl` block that do not take `self` as a parameter. They are called using the `::` syntax (e.g., `Struct::function()`) and are commonly used as constructors.

#### Method Receivers

Methods use different forms of `self` depending on how they need to interact with the struct instance:

- **`&self` (Immutable Borrow)**: The method reads data from the struct but doesn't modify it. The instance remains usable after the method call.
- **`&mut self` (Mutable Borrow)**: The method can modify the struct's data. The instance remains usable after the method call, but the variable must be declared as `mut`.
- **`self` (Ownership)**: The method takes ownership of the instance. The original instance cannot be used after the method call (rare use case).

---

### 💡 Practical Examples

#### Example 1: Basic Methods with `&self`

```rust
// Define the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Implementation block for the User struct
impl User {
    // Method that reads data immutably
    fn display_info(&self) {
        if self.active {
            println!("====== User Information ======");
            println!("Username: {}", self.username);
            println!("Email: {}", self.email);
            println!("Sign-in Count: {}", self.sign_in_count);
            println!("===============================");
        } else {
            println!("This user is not active.");
        }
    }
}

fn main() {
    // Create an instance of User
    let user1 = User {
        username: String::from("Cristian"),
        email: String::from("cristian@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // Call method using dot notation
    user1.display_info();
    
    // user1 can still be used after the method call
    user1.display_info();
}
```

**Explanation**: Methods that read data use `&self`, which borrows the instance immutably. This allows the instance to be used multiple times without any issues. The dot notation (`instance.method()`) is syntactic sugar that Rust automatically handles for us.

---

#### Example 2: Methods with `&mut self`

```rust
// Define the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // Method that modifies the struct
    fn deactivate(&mut self) {
        self.active = false;
    }

    // Another method that modifies the struct
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }

    // Method that reads data immutably
    fn display_info(&self) {
        println!("Active: {}, Sign-in Count: {}", self.active, self.sign_in_count);
    }
}

fn main() {
    // Variable must be declared as `mut` to call methods with `&mut self`
    let mut user1 = User {
        username: String::from("Cristian"),
        email: String::from("cristian@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.display_info();      // Output: Active: true, Sign-in Count: 1
    user1.increment_sign_in(); // Modifies the instance
    user1.display_info();      // Output: Active: true, Sign-in Count: 2
    user1.deactivate();        // Modifies the instance
    user1.display_info();      // Output: Active: false, Sign-in Count: 2
}
```

**Explanation**: Methods that modify a struct use `&mut self`. The instance variable must be declared as `mut`. These methods borrow the instance mutably, preventing other uses during the method call, but the instance remains usable afterward.

---

#### Example 3: Associated Functions as Constructors

```rust
// Define the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    // Associated function (no `self` parameter)
    // Used as a constructor
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    // Another associated function
    fn premium(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 0,
            active: true,
        }
    }

    // Method to display user info
    fn display_info(&self) {
        println!("User: {} | Email: {} | Active: {}", 
                 self.username, self.email, self.active);
    }
}

fn main() {
    // ceate instances using associated functions
    // Call associated functions using `::` syntax
    let user1 = User::new(
        String::from("Cristian"),
        String::from("cristian@example.com"),
    );
    user1.display_info(); // Output: User: Cristian | Email: cristian@example.com | Active: true

    let user2 = User::premium(
        String::from("Maria"),
        String::from("maria@example.com"),
    );
    user2.display_info(); // Output: User: Maria | Email: maria@example.com | Active: true
}
```

**Explanation**: Associated functions don't take `self` as a parameter and are called using the `::` syntax. They are commonly used as constructors (like `new`) to create new instances with specific initialization logic. The keyword `Self` can be used as an alias for the struct type.

---

#### Example 4: Methods with Additional Parameters

```rust
// Define the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    /// Method to compare emails of two users
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another User instance
    ///
    /// # Returns
    ///
    /// * `true` if the emails are the same, `false` otherwise
    fn is_same_email(&self, other: &User) -> bool {
        self.email == other.email
    }
}

fn main() {
    // Create two instances of User
    let user1 = User {
        username: String::from("Cristian"),
        email: String::from("Cristian@example.com"),
        sign_in_count: 1,
        active: true,
    };
    let user2 = User {
        username: String::from("Maria"),
        email: String::from("maria@example.com"),
        sign_in_count: 1,
        active: true,
    };

    // Call method with additional parameter
    if user1.is_same_email(&user2) {
        println!("Users have the same email.");
    } else {
        println!("Users have different emails.");
    }
}
```

**Explanation**: Methods can take additional parameters beyond `self`. In this example, the `is_same_email` method compares the email of the current instance with another `User` instance passed as a parameter.

---

#### Example 5: Multiple `impl` Blocks

```rust
// Define the struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// First impl block
impl User {
    fn activate(&mut self) {
        self.active = true;
    }
}
// Second impl block
impl User {
    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

**Explanation**: A struct can have multiple `impl` blocks. Methods from all blocks are available on instances of the struct. While unnecessary here, this pattern becomes useful when working with generic types and traits (advanced concepts).

---

### ⚠️ Important Points

- **Method Receivers Define Behavior**: The choice between `&self`, `&mut self`, and `self` determines how your method interacts with the instance. In Example 1, `display_info(&self)` reads data without modification, while in Example 2, `deactivate(&mut self)` and `increment_sign_in(&mut self)` require the variable to be declared as `mut` because they modify the struct's state.

- **Mutability Must Be Declared Upfront**: To call methods that take `&mut self` (like those in Example 2), the instance variable must be declared as `mut` from the start. You cannot later make it mutable; this is enforced at compile-time to ensure intentionality about mutations.

- **Associated Functions Construct Instances**: Unlike methods that operate on instances, associated functions (Example 3: `User::new()`, `User::premium()`) don't take `self` and use the `::` syntax. They are commonly used as constructors to initialize structs with specific defaults or logic.

- **Multiple Parameters Work Like Functions**: Methods can take additional parameters beyond `self` (Example 4: `is_same_email(&self, other: &User)`). These parameters follow standard function rules and allow methods to compare or operate on multiple data sources.

- **Automatic Referencing and Dereferencing**: When you call `user1.is_same_email(&user2)` in Example 4, you explicitly pass `&user2`. Rust automatically handles the necessary referencing based on the method signature, so you don't need to manually dereference or reference the receiver.

- **Multiple `impl` Blocks Organize Code**: As shown in Example 5, you can split related methods across multiple `impl` blocks for the same struct. This is particularly useful with generics and traits, though all methods remain equally accessible on instances.

---

### 🔗 Relations and Context

**Related Previous Topics**: 
- [Structs](01_Structs.md) - Struct definition and instantiation are essential prerequisites for learning methods
- [Ownership](../02_Ownership_System/01_Ownership.md) - Ownership rules directly govern method receivers (`self` vs `&self` vs `&mut self`)
- [Borrowing](../02_Ownership_System/02_Borrowing.md) - Borrowing and references are core to understanding `&self` and `&mut self` method receivers
- [Functions](../01_Fundamentals/03_Functions.md) - Methods are functions, so understanding parameters and return types is crucial

**Prerequisites**:
- Struct definition and usage patterns
- Ownership and move semantics
- Borrowing and reference concepts (`&`, `&mut`)
- Function parameters and return types
- Mutable variables and mutability patterns

**Follow-up Topics**:
- [Rust Book - Enums](https://doc.rust-lang.org/book/ch06-00-enums.html) - Creating custom types that represent different variants with their own methods

---

### 📖 References

- [Rust Book - Chapter 5.3: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust Book - Associated Functions](https://doc.rust-lang.org/book/ch05-03-method-syntax.html#associated-functions)
