// In this Rust program, we declare a variable and attempt to modify it.
// Variables in Rust are inmutable by default, so trying to change its value will result in a compile-time error. To make a variable mutable, we need to use the `mut` keyword.

fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // This line will now work because x is mutable
    println!("The value of x is: {}", x);

    // Shaddowing example
    // We can also shadow a variable by declaring a new variable with the same name.
    let x = x +3;
    println!("The value of x after shadowing is: {}", x);
    {
        let x = x * 3;
        println!("The value of x inside the inner scope is: {}", x);
    }

    println!("The value of x in the outer scope is: {}", x);

    // Shaddowing aren't limited to the same data type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces is: {}", spaces);

    // This is useful when we want to transform a value while keeping the same name. 
    // Eviting the need to come up with new variable names. 
    // e.g. spaces -> number_of_spaces

    // This is the list of datatypes in Rust:
    // Scalar types: integers, floating-point numbers, Booleans, and characters.
    // Compound types: tuples and arrays.

    // Integer types
    // Signed integers: i8, i16, i32, i64, i128, isize
    // Unsigned integers: u8, u16, u32, u64, u128, usize

    let unsigned_integer: u32 = 42;
    let signed_integer: i32 = -42;
    println!("Unsigned integer: {}, Signed integer: {}", unsigned_integer, signed_integer);

    // Floating-point types
    // f32 and f64, f64 is the default
    let float64_number: f64 = 3.14;
    println!("Floating-point 64bit number: {}", float64_number);
    let float32_number: f32 = 2.71;
    println!("Floating-point 32bit number: {}", float32_number);

    // Boolean type. Booleans can be either true or false. 
    // Useful for conditional testing.
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
    let is_rust_boring: bool = false;
    println!("Is Rust boring? {}", is_rust_boring);

    // Character type. Characters are specified with single quotes.
    // Rust's char type is four bytes in size and represents a Unicode Scalar Value.
    let letter: char = 'R';
    let smile: char = '😊';

    println!("Character letter: {}, Character smile: {}", letter, smile);


    // Coumpound types

    // Tuple type. A tuple is a general way of grouping together a number of values with a variety of types into one compound type. the length of a tuple is fixed once declared.

    let tuple_example: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple_example; // Destructuring the tuple
    println!("The value of y from the tuple is: {}", y);
    println!("The first value of the tuple is: {}", tuple_example.0);

    // Array type. An array is a collection of values of the same type. 
    // Arrays in Rust have a fixed length. Arrays use [] brackets to denote them.
    // Arrays are useful when you want your data allocated on the stack rather than the heap.

    let array_example: [u8; 5] = [1, 2, 3, 4, 5];
    //  Now creates an array of length 5, all elements initialized to 3
    let array_example2 = [3; 5]; 
    println!("The first element of the array is: {}", array_example[0]);
    println!("The second element of the second array is: {}", array_example2[1]);


}