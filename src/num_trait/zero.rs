#![allow(dead_code)]
pub trait Zero {
    fn zero() -> Self;
}
impl Zero for u8 {
    fn zero() -> Self {
        0
    }
}
impl Zero for u16 {
    fn zero() -> Self {
        0
    }
}
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}
impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}
impl Zero for usize {
    fn zero() -> Self {
        0
    }
}
