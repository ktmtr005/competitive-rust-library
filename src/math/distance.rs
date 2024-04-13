#![allow(dead_code)]

fn dist2<T>(&(x1, y1): &(T, T), &(x2, y2): &(T, T)) -> T
where
    T: Copy,
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Mul<Output = T>,
{
    let diff_x = x1 - x2;
    let diff_y = y1 - y2;
    diff_x * diff_x + diff_y * diff_y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dist2() {
        assert_eq!(dist2::<i32>(&(0, 0), &(3, 4)), 25);
    }
}
