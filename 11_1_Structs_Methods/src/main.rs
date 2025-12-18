//! Structs Methods in Rust
//! 
//! This example demonstrates how to define and use methods associated with a struct in Rust.

// Define a struct named `User`
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Implement methods for the `User` struct
impl User {
    // Method to create a new User instance
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }

    // Method to display user information
    fn display_info(&self) {
        if self.active {
            println!("====== User Information ======");
            println!("Username: {}", self.username);
            println!("Email: {}", self.email);
            println!("Sign-in Count: {}", self.sign_in_count);
            println!("===============================");
        }else {
            println!("Ups! This user is not active.");
        }
    }