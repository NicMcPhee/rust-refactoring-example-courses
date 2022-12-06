use std::iter::Map;

use crate::{person::{Student, Faculty}, grade::Grade};

pub struct Course {
    title: String,
    instructor: Faculty,
    students: Vec<Student>,
}

pub struct Ungraded {
    course: Course,
}

pub struct Graded {
    course: Course,
    grades: Map<Student, Option<Grade>>,
}
