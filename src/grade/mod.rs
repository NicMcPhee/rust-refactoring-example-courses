pub enum Letter {
    A,
    B,
    C,
    D,
    F,
}

pub struct Numeric {
    grade: u8,
}

pub enum ConversionError {
    Exceeds100,
}

pub enum Grade {
    LetterGrade(Letter),
    NumericGrade(Numeric),
}
