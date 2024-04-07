#![allow(dead_code)]

pub fn sieve_of_eratosthenes(end: usize) -> Vec<bool> {
    let mut is_prime = vec![true; end + 1];
    (is_prime[0], is_prime[1]) = (false, false);
    for i in 2..=((end as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * 2..=end).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    is_prime
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_prime_check() {
        let is_prime = sieve_of_eratosthenes(10);
        assert_eq!(
            is_prime,
            vec![false, false, true, true, false, true, false, true, false, false, false]
        );
    }
}
