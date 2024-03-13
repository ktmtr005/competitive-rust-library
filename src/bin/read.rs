use cargo_snippet::snippet;
fn main() {
    let s = read();
    println!("{}", s);
}

#[snippet]
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

#[cfg(test)]
mod test {
    use cli_test_dir::*;
    #[test]
    fn test_read() {
        let testdir = TestDir::new("./read", "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"abc
        "#,
            )
            .expect_success();
        assert_eq!(output.stdout_str(), "abc\n");
    }
}
