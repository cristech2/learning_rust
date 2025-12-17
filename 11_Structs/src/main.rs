//! Structs in Rust
//!
//! This module demonstrates the use of structs in Rust, including defining structs,
//! creating instances, and implementing methods.

// Define a immutable struct named `Student`
struct Student {
    name: String,
    age: u8,
    grade: char,
}

// Define a mutable struct named `Course`
struct Course {
    title: String,
    credits: u8,
    students_enrolled: u32,
}

fn main() {
    // Create an instance of the `Student` struct
    let student1 = Student {
        name: String::from("Cristian"),
        age: 33,
        grade: 'B',
    };

    // Access and print the fields of the struct
    println!("Student Name: {}", student1.name);
    println!("Student Age: {}", student1.age);
    println!("Student Grade: {}", student1.grade);

    // Create a mutable instance of the `Course` struct
    let mut course1 = Course {
        title: String::from("Rust Programming"),
        credits: 4,
        students_enrolled: 30,
    };

    // Access and print the fields of the struct
    println!("Course Title: {}", course1.title);
    println!("Course Credits: {}", course1.credits);
    println!("Students Enrolled: {}", course1.students_enrolled);

    // Modify the fields of the mutable struct
    course1.students_enrolled += 5;

    // Print the updated number of students enrolled
    println!("Updated Students Enrolled: {}", course1.students_enrolled);
}
