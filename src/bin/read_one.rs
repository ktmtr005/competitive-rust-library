fn main() {
    let s1 = read_one();
    let s2 = read_one();
    let s3 = read_one();
    println!("{} {} {}", s1, s2, s3);
}

fn read_one() -> String {
    use std::io::{stdin, Read};
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token
}

#[cfg(test)]
mod test {
    use cli_test_dir::*;
    #[test]
    fn test_read_one() {
        let testdir = TestDir::new("./read_one", "");
        let output = testdir
            .cmd()
            .output_with_stdin(
                r#"a b c
        "#,
            )
            .expect_success();
        assert_eq!(output.stdout_str(), "a b c\n");
    }
}
