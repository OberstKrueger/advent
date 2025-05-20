use std::default::Default;
use std::fmt::{Display, Formatter, Result};

pub struct Answer {
    pub first: Option<i64>,
    pub second: Option<i64>,
}

impl Default for Answer {
    fn default() -> Self {
        Answer {
            first: None,
            second: None,
        }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match (&self.first, &self.second) {
            (Some(first), Some(second)) => write!(f, "{}, {}", first, second),
            (Some(first), None)         => write!(f, "{}, n/a", first),
            (None, Some(second))        => write!(f, "n/a, {}", second),
            (None, None)                => write!(f, "n/a, n/a"),
        }
    }
}
