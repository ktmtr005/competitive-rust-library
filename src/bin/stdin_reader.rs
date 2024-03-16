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

    fn read_space(&mut self) -> String {
        self.read_until(b' ').to_string()
    }

    fn read_line(&mut self) -> String {
        self.read_until(b'\n').to_string()
    }

    fn read_until(&mut self, delim: u8) -> &str {
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
        std::str::from_utf8(&self.buf).expect("invalid utf-8 string")
    }
}

fn main() {
    let mut reader = StdinReader::new(std::io::stdin().lock());
    let s1 = reader.read_space();
    let s2 = reader.read_line();
    let s3 = reader.read_line();
    println!("{} {}\n{}", s1, s2, s3);
}

#[cfg(test)]
mod test {
    use cli_test_dir::*;
    #[test]
    fn test_stdin_reader() {
        let testdir = TestDir::new("./stdin_reader", "");
        let output = testdir
            .cmd()
            .output_with_stdin("a b c\nd e\n")
            .expect_success();
        assert_eq!(output.stdout_str(), "a b c\nd e\n");
    }
}
