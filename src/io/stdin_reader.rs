#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet("StdinReader")]
struct StdinReader<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
}

#[snippet("StdinReader")]
impl<R: std::io::BufRead> StdinReader<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
        }
    }

    fn read_space<T: std::str::FromStr>(&mut self) -> T {
        self.read_until(b' ')
    }

    fn read_line<T: std::str::FromStr>(&mut self) -> T {
        self.read_until(b'\n')
    }

    fn read_until<T: std::str::FromStr>(&mut self, delim: u8) -> T {
        // self.bufに次のトークンをセットする
        loop {
            self.buf.clear();
            let len = self
                .reader
                .read_until(delim, &mut self.buf)
                .expect("failed to reading bytes");
            match len {
                0 => panic!("early eof"),
                1 if self.buf[0] == delim => (), //区切り文字だけなのでもう一度ループ
                _ => {
                    // 最後の文字が区切り文字なら削除
                    if self.buf[len - 1] == delim {
                        self.buf.truncate(len - 1);
                    }
                    break;
                }
            }
        }
        let elem = unsafe { std::str::from_utf8_unchecked(&self.buf) };
        elem.parse()
            .unwrap_or_else(|_| panic!("{}", format!("failed parsing: {}", elem)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;

    #[test]
    fn test_reader() {
        let cursor = io::Cursor::new(b"123 -456 0.123 Hello, World!");
        let mut reader = StdinReader::new(cursor);

        assert_eq!(123, reader.read_space::<u32>());
        assert_eq!(-456, reader.read_space::<i32>());
        assert_eq!(0.123f64, reader.read_space());
        assert_eq!("Hello, World!".to_string(), reader.read_line::<String>());
    }
}
