use cargo_snippet::snippet;

fn main() {
    let s = String::from("Yes");
    write_line(&s);
}

#[snippet]
fn write_line(s: &str) {
    use std::io::{stdout, BufWriter, Write};
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    writeln!(out, "{}", s).unwrap();
}

#[cfg(test)]
mod test {
    use cli_test_dir::*;
    #[test]
    fn test_write() {
        let testdir = TestDir::new("./write", "test_name");
        let output = testdir.cmd().expect_success();
        assert_eq!(output.stdout_str(), "Yes\n");
    }
}
