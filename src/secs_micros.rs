use std::fmt;
use std::time::Duration;

/// A newtype wrapper for printing a `Duration` with microseconds,
/// meaning six digits after the decimal point.
#[derive(Copy, Clone, Debug)]
pub struct SecsMicros(pub Duration);

impl fmt::Display for SecsMicros {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{:06}", self.0.as_secs(), self.0.subsec_nanos() / 1000)
    }
}

