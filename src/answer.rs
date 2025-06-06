use std::default::Default;
use std::fmt::{Display, Formatter, Result};

pub struct Answer {
    pub first: i64,
    pub second: i64,
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            first: 0,
            second: 0,
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}, {}", self.first, self.second)
    }
}
