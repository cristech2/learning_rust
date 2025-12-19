//! Enums in Rust
//!
//! This example demonstrates how to define and use enums in Rust.

// Define an enum to represent different types of payment methods
enum PaymentMethod {
    CreditCard,
    PayPal,
    BankTransfer,
    MercadoPago,
}

// Define an enum with a lot of types to illustrate scalability
enum LargeEnum {
    Type1,
    Type2 { field1: i32, field2: bool },
    Type3(String),
    Type4(f64, f64),
}

/// Functions to process payments
///
/// # Arguments
///
/// * `method` - A PaymentMethod enum variant representing the payment method to be processed
/// * `amount` - A floating-point number representing the amount to be processed
fn process_payment(method: PaymentMethod, amount: f64) {
    match method {
        PaymentMethod::CreditCard => {
            println!("Processing credit card payment of ${}", amount);
            // Add logic for processing credit card payment
        }
        PaymentMethod::PayPal => {
            println!("Processing PayPal payment of ${}", amount);
            // Add logic for processing PayPal payment
        }
        PaymentMethod::BankTransfer => {
            println!("Processing bank transfer payment of ${}", amount);
            // Add logic for processing bank transfer payment
        }
        PaymentMethod::MercadoPago => {
            println!("Processing MercadoPago payment of ${}", amount);
            // Add logic for processing MercadoPago payment
        }
    }
}

fn main() {
    // Example usage of the PaymentMethod enum
    let payment = PaymentMethod::CreditCard;
    process_payment(payment, 150.0);

    let payment = PaymentMethod::PayPal;
    process_payment(payment, 75.5);

    // Example usage of the LargeEnum
    let large_enum_instance = LargeEnum::Type2 {
        field1: 42,
        field2: true,
    };
    match large_enum_instance {
        LargeEnum::Type1 => println!("LargeEnum Type1"),
        LargeEnum::Type2 { field1, field2 } => {
            println!(
                "LargeEnum Type2 with field1: {} and field2: {}",
                field1, field2
            );
        }
        LargeEnum::Type3(s) => println!("LargeEnum Type3 with string: {}", s),
        LargeEnum::Type4(x, y) => println!("LargeEnum Type4 with values: {}, {}", x, y),
    }
}
