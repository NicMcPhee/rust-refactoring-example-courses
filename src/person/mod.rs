pub struct Name {
    first_name: String,
    last_name: String,
}

pub enum Department {
    ComputerScience,
    StudioArt,
    History,
}

pub enum Position {
    Instructor,
    AssistantProfessor,
    AssociateProfessor,
    Professor,
}

pub struct Faculty {
    name: Name,
    department: Department,
    position: Position,
}

pub enum Major {
    ComputerScience,
    NativeAmericanAndIndigenousStudies,
    StudioArt,
    History,
}

pub struct Student {
    name: Name,
    major: Major,
}

pub enum Individual {
    Faculty(Faculty),
    Student(Student),
}
