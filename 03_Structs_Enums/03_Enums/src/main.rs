//! Enums in Rust
//!
//! This example demonstrates how to define and use enums in Rust.

// Define an basic enum to represent different types of fruits
// enum Fruit {
//     Apple,
//     Banana,
//     Orange,
// }

// Define the same enum with associated data
enum Fruit {
    Apple(String),       // Variety of apple
    Banana(u8),          // Ripeness level (0-10)
    Orange { size: u8 }, // Size in centimeters
}

// Define some methods for the Fruit enum
impl Fruit {
    /// Describe the fruit based on its type and associated data
    /// Examples
    ///
    /// let my_fruit = Fruit::Apple(String::from("Granny Smith"));
    /// my_fruit.describe(); // This is a Granny Smith apple.
    fn describe(&self) {
        match self {
            Fruit::Apple(variety) => println!("This is a {} apple.", variety),
            Fruit::Banana(ripeness) => {
                println!("This banana has a ripeness level of {}.", ripeness);
            }
            Fruit::Orange { size } => println!("This orange is {} cm in size.", size),
        }
    }
}

/// Create some example fruits based on string input
///
/// # Arguments
///
/// * `nombre` - A string slice that holds the name of the fruit
///
/// # Returns
///
/// * `Option<Fruit>` - Some(Fruit) if the fruit exists, None otherwise
fn create_fruit_examples(nombre: &str) -> Option<Fruit> {
    match nombre {
        "apple" => Some(Fruit::Apple(String::from("Gren Smith"))),
        "banana" => Some(Fruit::Banana(5)),
        "orange" => Some(Fruit::Orange { size: 6 }),
        _ => None,
    }
}

/// Search for a fruit and describe it if it exists
///
/// # Arguments     
///
/// * `fruit_search` - A reference to an Option<Fruit> to search
fn search_fruit_examples(fruit_search: &Option<Fruit>) {
    // We use the new let-else syntax to handle the Option
    let Some(fruit) = fruit_search else {
        // We handle the None case here
        println!("The requested fruit does not exist.");
        return;
    };
    // Now we know fruit is Some, we can describe it
    fruit.describe();
}

fn main() {
    // Create an instance of each fruit type
    let my_apple = Fruit::Apple(String::from("Red Delicious"));
    let my_banana = Fruit::Banana(7);
    let my_orange = Fruit::Orange { size: 10 };

    // Describe each fruit
    my_apple.describe();
    my_banana.describe();
    my_orange.describe();

    // Now tries to create fruits based on string input
    // we will create a fruit that does not exist
    // This is to show how Rust handles Option types
    // and what happens when we try to use a None value.
    // Create a fruit that doesn't exist
    let my_fruit = create_fruit_examples("pineapple");

    // Now we try to use anyway - this will cause a panic at runtime
    // my_fruit.describe();
    // error[E0599]: no method named `describe` found for enum `Option<T>` in the current scope

    // Now we handle the Option properly
    // if let Some(fruit) = my_fruit {
    //     fruit.describe();
    // } else {
    //     println!("The requested fruit does not exist.");
    // }
    search_fruit_examples(&my_fruit);
}
