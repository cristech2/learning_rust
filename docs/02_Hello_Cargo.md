## Hello World with Rust using Cargo

This is a simple "Hello, World!" program written in Rust using Cargo, the Rust package manager and build system.

### Prerequisites

Before you begin, ensure you have Rust and Cargo installed on your machine. You can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Steps to Create the Project

1. **Create a new Cargo project**:
   Open your terminal and run the following command to create a new Rust project named `hello_world`:
   ```bash
   cargo new hello_world
   ```
   This will create a new directory called `hello_world` with the necessary files.
2. **Navigate to the project directory**:
3. ```bash
   cd hello_world
   ```
4. **Open the `main.rs` file**:
   Open the `src/main.rs` file in your favorite text editor. You will see a default "Hello, World!" program.
5. **Modify the `main.rs` file (optional)**:
   You can modify the `main.rs` file to customize the message. For example:
   ```rust
   fn main() {
       println!("Hello, World!");
       println!("Welcome to Rust programming!");
    }
    ```
6. **Build and run the project**:
   In the terminal, run the following command to build 
   ```bash
    cargo build
    Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
    ```
    and execute the program:
    ```bash
    cargo run
    Hello, World!
    Welcome to Rust programming!
    ```


### Conclusion
You have successfully created and run a simple "Hello, World!" program in Rust using Cargo.

### Additional Resources
The information provided here is based on the official Rust documentation. For more details and advanced topics, refer to the [Rust Book](https://book.rustlang-es.org/ch01-03-hello-cargo).