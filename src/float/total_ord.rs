#![allow(dead_code)]

use std::cmp::Ordering;
use std::cmp::PartialOrd;

#[derive(PartialEq)]
pub struct Total<T>(pub T)
where
    T: PartialOrd;

impl<T: PartialEq + PartialOrd> Eq for Total<T> {}

impl<T: PartialEq + PartialOrd> PartialOrd for Total<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).expect("Comparison for NaN")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ord() {
        let mut arr = [2.0, 1.0, 1.5];
        arr.sort_by_key(|&x| Total(x));
        assert_eq!(arr, [1.0, 1.5, 2.0]);
    }
}
