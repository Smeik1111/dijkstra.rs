use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::Add;
use std::fmt;
use serde::{Deserialize, Serialize};

// a wrapper for float types; floats cannot be used directly as they do not implement Ord
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Float<T>(pub T);

impl<T: PartialEq> Eq for Float<T> {}
impl<T: PartialOrd> Ord for Float<T> {
    fn cmp(&self, other: &Float<T>) -> Ordering {
        // panic if any of the values are NaN
        self.partial_cmp(other).unwrap()
    }
}
impl<T: Add<Output=T>> Add for Float<T> {
    type Output = Float<T>;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<T: Sum> Sum for Float<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Float<T>>,
    {
        Float(iter.map(|cost| cost.0).sum())
    }
}

impl<T: fmt::Display> fmt::Display for Float<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
