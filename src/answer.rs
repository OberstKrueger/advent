use std::fmt;

pub struct Answer {
    pub first: Option<i64>,
    pub second: Option<i64>
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (&self.first, &self.second) {
            (Some(first), Some(second)) => write!(f, "{}, {}", first, second),
            (Some(first), None)         => write!(f, "{}, n/a", first),
            (None, Some(second))        => write!(f, "n/a, {}", second),
            (None, None)                => write!(f, "n/a, n/a"),
        }
    }
}
