//! Slice type example in Rust.
//!
//! This program demonstrates how to use string slices in Rust

/// Returns the first word of the given string slice.
///
/// # Arguments
///
/// * `s` - A string slice from which to extract the first word.
///
/// # Returns
///
/// * A string slice representing the first word in the input string slice.
fn first_word(s: &str) -> &str {
    // Convert the string slice to a byte array
    let bytes = s.as_bytes();

    // Iterate over the byte array to find the first space character
    for (i, &item) in bytes.iter().enumerate() {
        // If a space is found, return the slice from the start to the index of the space
        if item == b' ' {
            return &s[0..i];
        }
    }
    // If no space is found, return the entire string slice
    &s[..]
}

fn main() {
    let mut my_string = String::from("hello world");
    let array = [1, 2, 3, 4, 5];

    // Using a slice of the array
    // slice sintax: &data[start_index..end_index]
    let slice_array = &array[1..3];

    println!("Original array: {:?}", array);
    println!("Array slice: {:?}", slice_array);

    // Using the first_word function on a String
    let word = first_word(&my_string);
    println!("Original string: {}", my_string);
    println!("The first word is: {}", word);

    let my_string_literal = "hello Rust";
    // Using the first_word function on a string literal
    let word_literal = first_word(my_string_literal);
    println!("Original string literal: {}", my_string_literal);
    println!("The first word is: {}", word_literal);

    // Bad example (uncommenting the following lines will cause a compile-time error)
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear(); // This would invalidate the slice `word`
    // println!("The first word is: {}", word);
    // The above code is commented out to prevent compilation errors.
    // We cannot modify `s` while `word` is still in use. This prevents dangling references.
}
