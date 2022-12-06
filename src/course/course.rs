use crate::person::Faculty;

use super::Course;

impl Course {
    #[must_use]
    pub const fn instructor(&self) -> &Faculty {
        &self.instructor
    }

    #[must_use]
    pub fn num_students(&self) -> usize {
        self.students.len()
    }
}
