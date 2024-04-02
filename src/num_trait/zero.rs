use cargo_snippet::snippet;

#[snippet("Zero")]
pub trait Zero {
    fn zero() -> Self;
}
#[snippet("Zero")]
impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}
#[snippet("Zero")]
impl Zero for u64 {
    fn zero() -> Self {
        0
    }
}
#[snippet("Zero")]
impl Zero for usize {
    fn zero() -> Self {
        0
    }
}
