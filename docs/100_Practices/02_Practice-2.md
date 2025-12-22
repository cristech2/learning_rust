---
post_title: Practice 2 - Temperature Converter with Structs and Enums
author: Cristian Rosales
slug: practice-2-temperature-converter-structs-enums
categories: [Rust, Fundamentals]
tags: ["practice", "structs", "enums", "pattern-matching", "refactoring"]
summary: Refactor the temperature converter using structs and enums to create a more organized, maintainable, and extensible application that demonstrates advanced pattern matching and type safety.
post_date: 2025-12-13
---

## Practice 2: Temperature Converter with Structs and Enums

**Key Concept**: This practice takes the temperature converter from Practice 1 and
refactors it using Rust's powerful enums and structs. By representing temperature
units, menu options, and conversion requests as types, the code becomes more
organized, type-safe, and easier to extend with new conversions without modifying
existing logic.

---

### 📚 Fundamental Concepts

- **Enums for Variants**: Using enums to represent distinct options like menu choices
and temperature units, providing exhaustive pattern matching that catches missing cases
at compile time
- **Associated Data in Enums**: Storing values directly in enum variants (e.g.,
`Celsius(f64)`, `Fahrenheit(f64)`) to keep related data together
- **Structs for Composition**: Creating struct types that combine multiple related
fields (e.g., `ConversionRequest` containing source temperature and target unit)
- **Pattern Matching with Enums**: Using `match` expressions to handle all enum
variants, ensuring no case is forgotten and improving code clarity
- **Methods on Custom Types**: Implementing `impl` blocks to add behavior to enums and
structs, encapsulating logic within types
- **Type-Driven Design**: Leveraging Rust's type system to make invalid states
impossible to represent, catching errors at compile time rather than runtime

---

### 💡 Practical Examples

#### Example 1: Defining Enums for Menu Options

```rust
/// Enum to represent different options in the menu
#[derive(PartialEq)] // Enable comparison for MenuOptions
enum MenuOptions {
    CelsiusToFahrenheit,
    CelsiusToKelvin,
    FahrenheitToCelsius,
    FahrenheitToKelvin,
    KelvinToCelsius,
    KelvinToFahrenheit,
    Exit,
}

impl MenuOptions {
    /// Parse a u32 into a MenuOptions enum
    fn from_u32(input: u32) -> Option<Self> {
        match input {
            1 => Some(MenuOptions::CelsiusToFahrenheit),
            2 => Some(MenuOptions::CelsiusToKelvin),
            3 => Some(MenuOptions::FahrenheitToCelsius),
            4 => Some(MenuOptions::FahrenheitToKelvin),
            5 => Some(MenuOptions::KelvinToCelsius),
            6 => Some(MenuOptions::KelvinToFahrenheit),
            0 => Some(MenuOptions::Exit),
            _ => None,
        }
    }
}
```

**Explanation**: Enums represent all valid menu choices as distinct types. The
`from_u32` method converts user input to the appropriate enum variant, ensuring only
valid options are created. Using `Option<Self>` allows returning `None` for invalid
input, which is clearer than using sentinel values.

---

#### Example 2: Enums with Associated Data for Temperature Units

```rust
/// Enum to represent different temperature units with their values
enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl TemperatureUnit {
    /// Constructor for TemperatureUnit based on menu option
    fn new(option: &MenuOptions, value: f64) -> Option<Self> {
        match option {
            MenuOptions::CelsiusToFahrenheit | MenuOptions::CelsiusToKelvin => {
                Some(TemperatureUnit::Celsius(value))
            }
            MenuOptions::FahrenheitToCelsius | MenuOptions::FahrenheitToKelvin => {
                Some(TemperatureUnit::Fahrenheit(value))
            }
            MenuOptions::KelvinToCelsius | MenuOptions::KelvinToFahrenheit => {
                Some(TemperatureUnit::Kelvin(value))
            }
            _ => None,
        }
    }

    /// Extract the numeric value from the temperature unit
    fn value(&self) -> f64 {
        match self {
            TemperatureUnit::Celsius(v) => *v,
            TemperatureUnit::Fahrenheit(v) => *v,
            TemperatureUnit::Kelvin(v) => *v,
        }
    }
}
```

**Explanation**: Each enum variant stores its temperature value. The `new` method
uses pattern matching on menu options to create the correct temperature unit type.
The `value` method extracts the numeric temperature from any variant, demonstrating
how to work with associated data in enums.

---

#### Example 3: Structs for Conversion Requests

```rust
/// Enum to represent conversion target units
enum ConversionUnit {
    ToCelsius,
    ToFahrenheit,
    ToKelvin,
}

/// Struct to represent a conversion request
struct ConversionRequest {
    from: TemperatureUnit,
    to: ConversionUnit,
}

impl ConversionRequest {
    /// Constructor for ConversionRequest
    fn new(from: TemperatureUnit, to: Option<ConversionUnit>) -> Self {
        ConversionRequest {
            from,
            to: to.expect("Invalid conversion unit"),
        }
    }

    /// Perform the conversion and return the result
    fn convert(&self) -> f64 {
        match (&self.from, &self.to) {
            (TemperatureUnit::Celsius(value), ConversionUnit::ToFahrenheit) => {
                celsius_to_fahrenheit(*value)
            }
            (TemperatureUnit::Fahrenheit(value), ConversionUnit::ToCelsius) => {
                fahrenheit_to_celsius(*value)
            }
            (TemperatureUnit::Kelvin(value), ConversionUnit::ToCelsius) => {
                kelvin_to_celsius(*value)
            }
            // ...additional conversion patterns...
            _ => 0.0,
        }
    }
}
```

**Explanation**: The `ConversionRequest` struct combines the source temperature and
target unit into a single cohesive type. The `convert` method uses nested pattern
matching on both the `from` and `to` fields to determine which conversion function to
call. This design makes it clear what data is needed for each conversion.

---

#### Example 4: Pattern Matching in the Main Loop

```rust
fn main() {
    clear_console();
    let mut program_state: Option<MenuOptions> = None;

    while program_state != Some(MenuOptions::Exit) {
        MenuOptions::display_menu();
        program_state = MenuOptions::read_menu_options();

        match &program_state {
            // Handle temperature conversions
            Some(MenuOptions::CelsiusToFahrenheit)
            | Some(MenuOptions::CelsiusToKelvin)
            | Some(MenuOptions::FahrenheitToCelsius)
            | Some(MenuOptions::FahrenheitToKelvin)
            | Some(MenuOptions::KelvinToCelsius)
            | Some(MenuOptions::KelvinToFahrenheit) => {
                if let Some(program_state) = &program_state {
                    if let Some(temperature) = program_state.read_temperature_value() {
                        program_state.perform_conversion(temperature);
                    }
                }
            }
            // Handle exit option
            Some(MenuOptions::Exit) => {
                println!("Exiting the program. Goodbye!");
            }
            // Handle invalid menu option
            None => {
                clear_console();
                println!("Invalid choice, please try again.");
            }
        }
    }
}
```

**Explanation**: The main loop uses pattern matching to handle all possible enum
states. Using `|` in patterns groups multiple conversion options together. The `if
let` syntax simplifies working with `Option` types, executing code only when a value
is present. This structure is more readable and type-safe than the chained `if-else`
statements from Practice 1.

---

### ⚠️ Important Points

- **Exhaustive Pattern Matching**: The compiler ensures all enum variants are handled
in `match` expressions. Forgetting a case results in a compile error, not a runtime
bug
- **Type Safety**: By representing distinct concepts as enums, invalid combinations
(like converting Celsius to an unknown unit) become impossible to express in code
- **Code Organization**: Grouping related data (temperature value and unit) in enums
makes the code structure match the problem domain more closely
- **Deriving Traits**: Using `#[derive(PartialEq)]` on enums enables comparison
operations, which is useful for control flow like `while program_state !=
Some(MenuOptions::Exit)`
- **Pattern Guards**: The `|` operator in patterns allows handling multiple cases with
the same logic, reducing code duplication
- **Nested Pattern Matching**: Matching on multiple fields simultaneously (`&self.from,
&self.to`) makes conversion logic explicit and prevents errors

---

### 🔄 Refactoring from Practice 1

**Key Differences**:

1. **Conditional Logic**: Replaced chained `if-else if` statements with exhaustive
`match` expressions
2. **Type Representation**: Replaced string-based user choices with strongly-typed
enums
3. **Data Organization**: Grouped related values into structs and enum variants
instead of passing multiple parameters
4. **Error Handling**: Used `Option` types to represent potentially missing values
instead of recursive retry functions
5. **Code Encapsulation**: Moved logic into `impl` blocks on types, improving code
organization and readability

---

### 🔗 Relations and Context

**Related Previous Topics**:
- [Practice 1 - Basic Temperature Converter](./01_Practice-1.md)
- [Enums in Rust](../03_Structs_Enums/03_Enums.md)
- [Structs in Rust](../03_Structs_Enums/01_Structs.md)
- [Ownership](../02_Ownership_System/01_Ownership.md)

**Prerequisites**:
- Understanding of basic temperature conversion formulas
- Knowledge of function definition and composition
- Familiarity with `match` expressions and pattern matching
- Understanding of Rust's `Option` type and `if let` syntax
- Basic knowledge of trait derivation with `#[derive]`

**Follow-up Topics**:
- [Package Managment](https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

### 📖 References

- [Rust Enums](https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html)
- [Rust Structs](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html)
- [Understanding Ownership](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)
- [Rust Book](https://doc.rust-lang.org/stable/book/)