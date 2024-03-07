#![allow(dead_code)]

use cargo_snippet::snippet;

#[snippet("read")]
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

#[snippet("read_all")]
fn read_all() -> String {
    use std::io::Read;
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).ok();
    String::from_utf8(buf).unwrap()
}

#[snippet("parse_all")]
fn parse_all<T: std::str::FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split_whitespace()
        .map(|x| x.parse().expect("failed to parse."))
        .collect::<Vec<T>>()
}

#[snippet("get_lines")]
fn get_lines(s: String) -> Vec<String> {
    s.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_all() {
        let input = String::from("0 1 2 3 4");
        assert_eq!(parse_all::<u64>(input), vec![0, 1, 2, 3, 4]);
        let input2 = String::from("-1 -2 -3 -4 -5");
        assert_eq!(parse_all::<i32>(input2), vec![-1, -2, -3, -4, -5]);
    }

    #[test]
    fn test_get_lines() {
        let input = String::from("Foo Bar\nFooBar\n");
        let res = vec!["Foo Bar".to_string(), "FooBar".to_string()];
        assert_eq!(get_lines(input), res);
    }
}
