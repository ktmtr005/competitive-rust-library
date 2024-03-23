#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("Scanner")]
struct Scanner<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
    pos: usize,
}

#[snippet("Scanner")]
impl<R: std::io::BufRead> Scanner<R> {
    fn new(reader: R, capacity: usize) -> Self {
        Scanner {
            reader,
            buf: Vec::with_capacity(capacity),
            pos: 0,
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T
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
        let elem = unsafe { std::str::from_utf8_unchecked(&self.buf[start.unwrap()..self.pos]) };
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

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[test]
    fn test_scanner() {
        let cursor = io::Cursor::new(b"123 -456\n0.123 Hello, World!");
        let mut reader = Scanner::new(cursor, 28);

        assert_eq!(123, reader.next::<u32>());
        assert_eq!(-456, reader.next::<i32>());
        assert_eq!(0.123, reader.next::<f64>());
        assert_eq!("Hello,".to_string(), reader.next::<String>());
        assert_eq!("World!".to_string(), reader.next::<String>());
    }
}
