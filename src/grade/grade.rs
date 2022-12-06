use super::{GradeConversionError, NumericGrade};

impl TryFrom<u8> for NumericGrade {
    type Error = GradeConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value <= 100 {
            Ok(Self { grade: value })
        } else {
            Err(GradeConversionError::Exceeds100)
        }
    }
}
