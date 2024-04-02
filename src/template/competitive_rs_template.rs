mod lib {
    #![allow(dead_code)]
    pub mod stdin {
        pub struct Scanner<R: std::io::BufRead> {
            reader: R,
            buf: Vec<u8>,
            pos: usize,
        }
        impl<R: std::io::BufRead> Scanner<R> {
            pub fn new(reader: R) -> Self {
                Scanner {
                    reader,
                    buf: Vec::new(),
                    pos: 0,
                }
            }
            pub fn with_capacity(reader: R, capacity: usize) -> Self {
                Scanner {
                    reader,
                    buf: Vec::with_capacity(capacity),
                    pos: 0,
                }
            }
            pub fn next<T: std::str::FromStr>(&mut self) -> T
            where
                T::Err: std::fmt::Debug,
            {
                if self.buf.is_empty() {
                    self.read_next_line();
                }
                let mut start = None;
                loop {
                    if self.pos == self.buf.len() {
                        break;
                    }
                    match (self.buf[self.pos], start.is_some()) {
                        (b' ', true) | (b'\n', true) => break,
                        (_, true) | (b' ', false) => self.pos += 1,
                        (b'\n', false) => self.read_next_line(),
                        (_, false) => start = Some(self.pos),
                    }
                }
                let elem =
                    unsafe { std::str::from_utf8_unchecked(&self.buf[start.unwrap()..self.pos]) };
                elem.parse()
                    .unwrap_or_else(|_| panic!("{}", format!("failed parsing: {}", elem)))
            }
            fn read_next_line(&mut self) {
                self.pos = 0;
                self.buf.clear();
                if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                    panic!("Reached EOF");
                }
            }
        }
    }
    pub mod stdout {
        pub fn fastout(s: &str) {
            use std::io::{stdout, BufWriter, Write};
            let mut out = BufWriter::new(stdout().lock());
            writeln!(out, "{}", s).expect("failed to write data.");
        }
    }
    pub mod num {
        pub trait Zero {
            fn zero() -> Self;
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
    }
    pub mod math {
        use super::num::Zero;
        pub fn gcd<T>(a: T, b: T) -> T
        where
            T: std::marker::Copy,
            T: std::cmp::Eq,
            T: std::ops::Rem<Output = T>,
            T: Zero,
        {
            if b == T::zero() {
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
            T: Zero,
        {
            a / gcd::<T>(a, b) * b
        }
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
    }
}
