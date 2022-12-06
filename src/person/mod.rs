mod individual;

use serde::Deserialize;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Name {
    first_name: String,
    last_name: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum Department {
    ComputerScience,
    StudioArt,
    History,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum Position {
    Instructor,
    AssistantProfessor,
    AssociateProfessor,
    Professor,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Faculty {
    name: Name,
    department: Department,
    position: Position,
}

#[derive(Deserialize, Debug)]
pub enum Major {
    ComputerScience,
    NativeAmericanAndIndigenousStudies,
    StudioArt,
    History,
}

#[derive(Deserialize, Debug)]
pub struct Student {
    name: Name,
    major: Major,
}

pub enum Individual {
    Faculty(Faculty),
    Student(Student),
}
