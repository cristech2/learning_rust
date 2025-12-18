//! Structs in Rust
//!
//! This module demonstrates the use of structs in Rust, including defining structs,
//! creating instances, and implementing methods.

// Define a struct named `Student`
struct Student {
    active: bool,
    name: String,
    age: u8,
    grade: char,
}

// Define a struct named `Course`
struct Course {
    title: String,
    credits: u8,
    students_enrolled: u32,
}

// Define a tuple struct named `Point`
struct Point(i32, i32, i32);

/// Build a function to create a new `Student`
///
/// # Arguments
///
/// * `name` - A String representing the student's name
/// * `age` - A u8 representing the student's age
/// * `grade` - A char representing the student's grade
///
/// # Returns
///
/// * A `Student` struct instance
fn build_student(name: String, age: u8, grade: char) -> Student {
    // Create and return a new `Student` instance
    // Student fields are initialized with the provided arguments
    // The `active` field is set to true by default
    // Using field init shorthand for `name`, `age`, and `grade`
    // this way is available when the parameter names match the field names
    Student {
        active: true,
        name,
        age,
        grade,
    }
}

/// Function to print student information
///
/// # Arguments
///     
/// * `student` - A reference to a `Student` struct
fn print_student_info(student: &Student) {
    println!("======== Student Information ========");
    println!("Student Name: {}", student.name);
    println!("Student Age: {}", student.age);
    println!("Student Grade: {}", student.grade);
    println!("=====================================");
}

/// Function to print Course information
///
/// # Arguments
///
/// * `course` - A reference to a `Course` struct
fn print_course_info(course: &Course) {
    println!("======== Course Information ========");
    println!("Course Title: {}", course.title);
    println!("Course Credits: {}", course.credits);
    println!("Students Enrolled: {}", course.students_enrolled);
    println!("=====================================");
}
fn main() {
    // Create an instance of the `Student` struct
    let student1 = Student {
        active: true,
        name: String::from("Cristian"),
        age: 33,
        grade: 'B',
    };

    // Access and print the fields of the struct
    if student1.active {
        print_student_info(&student1);
    }

    // Using a update syntax to create a new instance of the `Student` struct
    // This new instance will copy the `active` and `name` fields from `student1`
    let student2 = Student {
        age: 28,
        grade: 'A',
        ..student1
    };

    print_student_info(&student2);
    // This line would cause a compile-time error because `student1` has been partially moved
    // student1.name is no longer valid here because String does not implement the Copy trait
    // print_student_info(&student1);

    // Create another instance of the `Student` struct using the builder function
    let student3: Student = build_student(String::from("Eduardo"), 32, 'C');

    if student2.active {
        print_student_info(&student3);
    }

    // Create a mutable instance of the `Course` struct
    let mut course1 = Course {
        title: String::from("Rust Programming"),
        credits: 4,
        students_enrolled: 30,
    };

    // Access and print the fields of the struct
    print_course_info(&course1);

    // Modify the fields of the mutable struct
    course1.students_enrolled += 5;

    // Print the updated number of students enrolled
    println!("Updated Students Enrolled: {}", course1.students_enrolled);

    // Create an instance of the `Point` tuple struct
    let point1 = Point(10, 20, 30);
    // Access and print the fields of the tuple struct
    println!(
        "The cordinates of point1 are: x: {}, y: {}, z: {}",
        point1.0, point1.1, point1.2
    );
}
