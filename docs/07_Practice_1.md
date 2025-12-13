---
post_title: Practica n1
author1: Cristian Rosales
post_slug: 07_Practice_1
microsoft_alias: 
featured_image: 
categories: [Rust, Programming Basics]
tags: ["practice", "rust", "temperature conversion"]
ai_note: No
summary: A simple Rust program to convert temperature units between Celsius, Fahrenheit, and Kelvin.
post_date: 2025-12-13
---

## Practice n1: Temperature Units Conversion

In this practice, we will create a simple Rust program that converts temperature
units between Celsius, Fahrenheit, and Kelvin.
This exercise will help you understand basic Rust syntax, functions, and user input handling.
Only use the concepts learned so far: variables, data types, functions, conditionals, and loops.

### Requirements

1. Create a Rust program that prompts the user to enter a temperature value and its current unit (Celsius, Fahrenheit, or Kelvin).
2. The program should then ask the user to specify the target unit for conversion.
3. Implement the conversion logic for the following conversions:
    - Celsius to Fahrenheit
    - Celsius to Kelvin
    - Fahrenheit to Celsius
    - Fahrenheit to Kelvin
    - Kelvin to Celsius
    - Kelvin to Fahrenheit
4. Display the converted temperature value along with the target unit.
5. Handle invalid inputs gracefully by prompting the user to enter valid values.
6. Allow the user to perform multiple conversions until they choose to exit the program.
7. Comment your code to explain the logic and flow of the program.

### Example Interaction

```
------ Temperature Conversion ------
------ Select Conversion Type ------
1. Celsius to Fahrenheit
2. Celsius to Kelvin
3. Fahrenheit to Celsius
4. Fahrenheit to Kelvin
5. Kelvin to Celsius
6. Kelvin to Fahrenheit
0. Exit
Enter your choice: 1
Enter temperature in Celsius: 25
25.0 Celsius is 77.0 Fahrenheit.
------ Select Conversion Type ------
1. Celsius to Fahrenheit
2. Celsius to Kelvin
3. Fahrenheit to Celsius
4. Fahrenheit to Kelvin
5. Kelvin to Celsius
6. Kelvin to Fahrenheit
0. Exit
Enter your choice: 0
------ Exiting Program. Goodbye! :) ------
```

### Hints

- Use `std::io` for handling user input.
- Define functions for each conversion to keep your code organized.
- Use a loop to allow multiple conversions until the user decides to exit.
- Validate user input to ensure the temperature value is a number and the units are valid.
- Only use basic Rust features covered so far to complete this practice.

