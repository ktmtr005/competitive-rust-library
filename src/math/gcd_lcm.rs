#![allow(dead_code)]

pub fn gcd<T>(a: T, b: T) -> T
where
    T: std::marker::Copy,
    T: std::cmp::Eq,
    T: std::ops::Rem<Output = T>,
    T: std::default::Default,
{
    if b == T::default() {
        a
    } else {
        gcd::<T>(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: std::marker::Copy,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Rem<Output = T>,
    T: std::cmp::Eq,
    T: std::default::Default,
{
    a / gcd::<T>(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd::<u32>(57, 3), 3);
        assert_eq!(gcd(12usize, 24usize), 12usize);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm::<u32>(3, 19), 57);
        assert_eq!(lcm::<u64>(3, 5), 15)
    }
}
