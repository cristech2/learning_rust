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
    /// Method to create a new User instance
    ///
    /// # Arguments
    ///
    /// * `username` - A String representing the username
    /// * `email` - A String representing the email address
    ///
    /// # Returns
    ///     
    /// * A new instance of `User`
    fn new(username: String, email: String) -> User {
        User {
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }

    /// Method to display user information

    fn display_info(&self) {
        if self.active {
            println!("====== User Information ======");
            println!("Username: {}", self.username);
            println!("Email: {}", self.email);
            println!("Sign-in Count: {}", self.sign_in_count);
            println!("===============================");
        } else {
            println!("Ups! This user is not active.");
        }
    }
    /// Method to deactivate the user
    fn deactivate(&mut self) {
        self.active = false;
    }
    /// Method to activate the user
    fn activate(&mut self) {
        self.active = true;
    }
    /// Method to increment the sign-in count
    fn increment_sign_in(&mut self) {
        self.sign_in_count += 1;
    }
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
    // Create a new User instance using the `new` method
    let mut user1 = User::new(
        String::from("Cristian"),
        String::from("cristian@example.com"),
    );

    // Using methods on the User Struct
    // Display user information
    user1.display_info();

    // Increment sign-in count
    user1.increment_sign_in();

    // Display updated user information
    user1.display_info();

    // Deactivate the user
    user1.deactivate();

    // try to display user information for an inactive user
    user1.display_info();

    // Activate the user again
    user1.activate();

    // Display user information after reactivation
    user1.display_info();

    // Create another User instance
    let user2 = User::new(String::from("Pedro"), String::from("pedro@example.com"));

    // Using the is_same_email method to compare users
    if user1.is_same_email(&user2) {
        println!("Both users have the same email.");
    } else {
        println!("The users have different emails.");
    }
}
