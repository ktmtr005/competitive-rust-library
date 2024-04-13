#![allow(dead_code)]

pub fn dist2<T>(&(x1, y1): &(T, T), &(x2, y2): &(T, T)) -> T
where
    T: Copy,
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: PartialOrd,
{
    use std::cmp::Ordering;
    let (x_greater, x_less) =
        if x1.partial_cmp(&x2).expect("Comparison with NaN") == Ordering::Greater {
            (x1, x2)
        } else {
            (x2, x1)
        };
    let (y_greater, y_less) =
        if y1.partial_cmp(&y2).expect("Comparison with NaN") == Ordering::Greater {
            (y1, y2)
        } else {
            (y2, y1)
        };
    let diff_x = x_greater - x_less;
    let diff_y = y_greater - y_less;
    diff_x * diff_x + diff_y * diff_y
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dist2() {
        assert_eq!(dist2::<i32>(&(0, 0), &(3, 4)), 25);
        assert_eq!(dist2::<usize>(&(0, 0), &(1, 2)), 5);
        assert_eq!(dist2::<u32>(&(0, 0), &(0, 0)), 0);
        assert_eq!(dist2::<f64>(&(0.0, 0.0), &(3.0, 4.0)), 25.0);
    }
}
