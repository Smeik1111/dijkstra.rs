use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::Add;
use std::fmt;
use serde::{Deserialize, Serialize};

// a wrapper around an integer or float type
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Cost<T>(pub T);

impl<T: PartialEq> Eq for Cost<T> {}
impl<T: PartialOrd> Ord for Cost<T> {
    fn cmp(&self, other: &Cost<T>) -> Ordering {
        // panic if any of the values are NaN
        self.partial_cmp(other).unwrap()
    }
}
impl<T: Add<Output=T>> Add for Cost<T> {
    type Output = Cost<T>;
    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl<T: Sum> Sum for Cost<T> {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Cost<T>>,
    {
        Cost(iter.map(|cost| cost.0).sum())
    }
}

impl<T: fmt::Display> fmt::Display for Cost<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
