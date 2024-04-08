#![allow(dead_code)]
pub struct StdinReader<R: std::io::BufRead> {
    reader: R,
    buf: Vec<u8>,
}

impl<R: std::io::BufRead> StdinReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: Vec::new(),
        }
    }

    pub fn read_space<T: std::str::FromStr>(&mut self) -> T {
        self.read_until(b' ')
    }

    pub fn read_line<T: std::str::FromStr>(&mut self) -> T {
        self.read_until(b'\n')
    }

    fn read_until<T: std::str::FromStr>(&mut self, delim: u8) -> T {
        // set a next token to self.buf
        loop {
            self.buf.clear();
            let len = self
                .reader
                .read_until(delim, &mut self.buf)
                .expect("failed to reading bytes");
            match len {
                0 => panic!("early eof"),
                1 if self.buf[0] == delim => (), // only a delimiter, so loop again
                _ => {
                    // remove delimiter
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
