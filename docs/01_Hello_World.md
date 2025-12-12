## Hello World Project
This is a simple "Hello World" project in Rust. This is the first program that many people write when learning a new programming language.

### Getting Started

1. Make sure you hace Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
2. Create a new folder for your project and navigate into it:
   ```bash
   mkdir hello_world
   cd hello_world
   ```
3. Create a new Rust file named `main.rs`:
   ```bash
   touch main.rs
   ```
4. Open `main.rs` in your favorite text editor and add the following code:
```rust
fn main() {
    println!("Hello, World!");
    println!("This is my first Rust program.");
}
```
5. Save the file and go to your favorite terminal, in my case i use `zsh`.
6. Compile and run the program using the Rust Compiler (`rustc`):
    ```bash
    rustc main.rs
    ./main
    ```
7. You should see the following output:
    ```
    Hello, World!
    This is my first Rust program.
    ``` 

### Congratulations!
You have successfully written and executed your first Rust program! Feel free to modify the code and experiment with different messages. Happy coding!


### License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.