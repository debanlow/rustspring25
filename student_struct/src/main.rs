// DEBANHI SILVA 20592937
// in class assignment 

// create struct Student (major)
struct Student {
    major: String,
}

// first order functions, assign_major(student, major_declared)
fn assign_major(student: &mut Student, major_declared: String) {
    student.major = major_declared;
}

// higher order functions update majors
fn update_majors(mut collection: Vec<Student>, behavior: fn(&mut Student, String)) {
    for student in collection.iter_mut() {
        behavior(student, "Computer Science".to_string());
    }

    for (i, student) in collection.iter().enumerate() {
        println!("Student {} major: {}", i + 1, student.major);
    }
}

// create a vector of students 1, 2, 3 and update all students major
fn main() {
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];

    update_majors(students, assign_major);
}