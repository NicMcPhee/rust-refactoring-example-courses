use std::{fs::File, io::Read, collections::HashMap};

use courses::{course::Course, person::Faculty};

fn main() {
    let mut file = File::open("data.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let courses: Vec<Course> = serde_json::from_str(&data).unwrap();

    let mut faculty_student_count: HashMap<&Faculty, usize> = HashMap::new();
    for course in &courses {
        let faculty = course.instructor();
        let num_students = course.num_students();
        faculty_student_count.entry(faculty).and_modify(|counter| *counter += num_students).or_insert(num_students);
    }

    for (faculty, num_students) in faculty_student_count {
        println!("Faculty member {} has {} students.", faculty, num_students);
    }
}
