//! A not-so-simple  Rust program that converts Temperature Units.
//!
//! In the anoter Rust program, we created a simple temperature converter that converted
//! between Celsius and Fahrenheit.
//! In this program, we will refactor the code to use Structs and Enums to make it more organized and extensible.
use std::io::{self, Write}; //  Use this to handle user input and clear the console

///Global constants for temperature conversion
const SCALE_FACTOR: f64 = 1.8;
const FAHRENHEIT_OFFSET: f64 = 32.0;
const KELVIN_OFFSET: f64 = 273.15;

/// Function to clear the console screen
fn clear_console() {
    // Clear the console screen
    print!("\x1B[2J\x1B[1;1H");
    match io::stdout().flush() {
        Ok(_) => {}
        Err(e) => println!("Error flushing stdout: {}", e),
    }
}

/// Enum to represent different options in the menu
#[derive(PartialEq)] // Derive PartialEq to compare enum variants
enum MenuOptions {
    CelsiusToFahrenheit,
    CelsiusToKelvin,
    FahrenheitToCelsius,
    FahrenheitToKelvin,
    KelvinToCelsius,
    KelvinToFahrenheit,
    Exit,
}
/// Implement methods for MenuOptions
impl MenuOptions {
    /// Function to display the menu options to the user
    fn display_menu() {
        println!("====== Temperature Converter ======");
        println!("1. Celsius to Fahrenheit");
        println!("2. Celsius to Kelvin");
        println!("3. Fahrenheit to Celsius");
        println!("4. Fahrenheit to Kelvin");
        println!("5. Kelvin to Celsius");
        println!("6. Kelvin to Fahrenheit");
        println!("0. Exit");
        println!("===================================");
    }

    /// Parse a u32 into a MenuOptions enum
    ///
    /// # Arguments
    ///
    /// * `input` - A u32 representing the menu option
    ///
    /// # Returns
    ///
    /// * A menuOptions enum
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

    /// Read user input and return the selected menu option
    ///
    /// # Returns
    ///
    /// * Selected menu option as `MenuOptions` enum
    fn read_menu_options() -> Option<Self> {
        println!("Enter your choice: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line, please try again.");
        match input.trim().parse::<u32>() {
            Ok(num) => match MenuOptions::from_u32(num) {
                Some(option) => Some(option),
                None => {
                    println!("Invalid choice, please try again.");
                    None
                }
            },
            Err(_) => {
                println!("Invalid choice, please try again.");
                None
            }
        }
    }

    /// Read temperature value from user input
    ///
    /// # Returns
    ///
    /// * A TemperatureUnit enum with the input value
    fn read_temperature_value(&self) -> Option<TemperatureUnit> {
        // If the option is Exit, return None
        if let Self::Exit = self {
            return None;
        }
        println!("Enter the temperature value: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line, please try again.");
        // match input.trim().parse::<f64>() {
        //     Ok(value) => TemperatureUnit::new(self, value),
        //     Err(_) => {
        //         println!("Invalid temperature value, please try again.");
        //         None
        //     }
        // }
        let Ok(value) = input.trim().parse::<f64>() else {
            println!("Invalid temperature value, please try again.");
            return None;
        };
        TemperatureUnit::new(self, value)
    }

    /// Function to perform conversion based on the menu option
    ///
    /// # Arguments
    ///
    /// * `temperature` - A TemperatureUnit enum representing the input temperature
    fn perform_conversion(&self, temperature: TemperatureUnit) {
        let (to_unit, from_label, to_label) = match self {
            MenuOptions::CelsiusToFahrenheit => (ConversionUnit::ToFahrenheit, "°C", "°F"),
            MenuOptions::CelsiusToKelvin => (ConversionUnit::ToKelvin, "°C", "K"),
            MenuOptions::FahrenheitToCelsius => (ConversionUnit::ToCelsius, "°F", "°C"),
            MenuOptions::FahrenheitToKelvin => (ConversionUnit::ToKelvin, "°F", "K"),
            MenuOptions::KelvinToCelsius => (ConversionUnit::ToCelsius, "K", "°C"),
            MenuOptions::KelvinToFahrenheit => (ConversionUnit::ToFahrenheit, "K", "°F"),
            _ => return,
        };
        let request = ConversionRequest::new(temperature, Some(to_unit));
        let result = request.convert();
        clear_console();
        println!("====== Conversion Result ======");
        println!(
            "{:.2} {} is {:.2} {}",
            request.from.value(),
            from_label,
            result,
            to_label
        );
        println!("===============================");
    }
}
/// Enum to represent different temperature units
enum TemperatureUnit {
    Celsius(f64),
    Fahrenheit(f64),
    Kelvin(f64),
}

impl TemperatureUnit {
    /// Constructor for TemperatureUnit
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
    fn value(&self) -> f64 {
        match self {
            TemperatureUnit::Celsius(v) => *v,
            TemperatureUnit::Fahrenheit(v) => *v,
            TemperatureUnit::Kelvin(v) => *v,
        }
    }
}

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
/// Implement methods for ConversionRequest
impl ConversionRequest {
    /// Constructor for ConversionRequest
    fn new(from: TemperatureUnit, to: Option<ConversionUnit>) -> Self {
        let from_unit = from;
        let to_unit = to.expect("Invalid conversion unit");
        ConversionRequest {
            from: from_unit,
            to: to_unit,
        }
    }

    /// Perform the conversion and return the result
    ///
    /// # Returns
    ///
    /// * Converted temperature value as f64
    fn convert(&self) -> f64 {
        match (&self.from, &self.to) {
            (TemperatureUnit::Celsius(value), ConversionUnit::ToFahrenheit) => {
                celsius_to_fahrenheit(*value)
            }
            (TemperatureUnit::Celsius(value), ConversionUnit::ToKelvin) => {
                celsius_to_kelvin(*value)
            }
            (TemperatureUnit::Fahrenheit(value), ConversionUnit::ToCelsius) => {
                fahrenheit_to_celsius(*value)
            }
            (TemperatureUnit::Fahrenheit(value), ConversionUnit::ToKelvin) => {
                fahrenheit_to_kelvin(*value)
            }
            (TemperatureUnit::Kelvin(value), ConversionUnit::ToCelsius) => {
                kelvin_to_celsius(*value)
            }
            (TemperatureUnit::Kelvin(value), ConversionUnit::ToFahrenheit) => {
                kelvin_to_fahrenheit(*value)
            }
            _ => 0.0,
        }
    }
}

/// Converts Celsius to Fahrenheit.
///
/// # Arguments
/// * `celsius` - Temperature in Celsius.
///
/// # Returns
/// * Temperature in Fahrenheit.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    // Formula to convert Celsius to Fahrenheit
    (celsius * SCALE_FACTOR) + FAHRENHEIT_OFFSET
}

/// Converts Celsius to Kelvin.
///
/// # Arguments
/// * `celsius` - Temperature in Celsius.
///
/// # Returns
/// * Temperature in Kelvin.
fn celsius_to_kelvin(celsius: f64) -> f64 {
    // Formula to convert Celsius to Kelvin
    celsius + KELVIN_OFFSET
}

/// Converts Fahrenheit to Celsius.
///
/// # Arguments
/// * `fahrenheit` - Temperature in Fahrenheit.
///
/// # Returns
/// * Temperature in Celsius.
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    // Formula to convert Fahrenheit to Celsius
    (fahrenheit - FAHRENHEIT_OFFSET) / SCALE_FACTOR
}

/// Converts Fahrenheit to Kelvin.
///
/// # Arguments
/// * `fahrenheit` - Temperature in Fahrenheit.
///
/// # Returns
/// * Temperature in Kelvin.
fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    // First convert Fahrenheit to Celsius, then Celsius to Kelvin
    let celsius = fahrenheit_to_celsius(fahrenheit);
    celsius_to_kelvin(celsius)
}

/// Converts Kelvin to Celsius.
///
/// # Arguments
/// * `kelvin` - Temperature in Kelvin.
///
/// # Returns
/// * Temperature in Celsius.
fn kelvin_to_celsius(kelvin: f64) -> f64 {
    // Formula to convert Kelvin to Celsius
    kelvin - KELVIN_OFFSET
}

/// Converts Kelvin to Fahrenheit.
///
/// # Arguments
/// * `kelvin` - Temperature in Kelvin.
///
/// # Returns
/// * Temperature in Fahrenheit.
fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    // First convert Kelvin to Celsius, then Celsius to Fahrenheit
    let celsius = kelvin_to_celsius(kelvin);
    celsius_to_fahrenheit(celsius)
}

fn main() {
    // Clear the console screen at the start of the program
    clear_console();
    // Flag to control the main loop
    let mut program_state: Option<MenuOptions> = None;

    // Main loop
    while program_state != Some(MenuOptions::Exit) {
        // Display the menu options
        MenuOptions::display_menu();
        // Read user input
        program_state = MenuOptions::read_menu_options();

        // Handle the selected menu option
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
                // Handle invalid menu option
                clear_console();
                println!("Invalid choice, please try again.");
            }
        }
    }
}
