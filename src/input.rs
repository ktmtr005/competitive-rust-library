#![allow(dead_code)]

use cargo_snippet::snippet;

#[snippet("read")]
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line.");
    buf
}

#[snippet(include = "read")]
#[snippet("get_string")]
fn get_string(s: String) -> String {
    s.trim().to_string()
}

#[snippet(include = "read")]
#[snippet(include = "get_string")]
#[snippet("string_to_vec")]
fn string_to_vec<T: std::str::FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_string(s)
        .split_whitespace()
        .map(|x| x.parse().expect("failed to parse."))
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_string() {
        let input = String::from("Foo\n");
        assert_eq!(get_string(input), String::from("Foo"));
    }

    #[test]
    fn test_string_to_vec() {
        let input = String::from("0 1 2 3 4");
        assert_eq!(string_to_vec::<u64>(input), vec![0, 1, 2, 3, 4]);
        let input2 = String::from("-1 -2 -3 -4 -5");
        assert_eq!(string_to_vec::<i32>(input2), vec![-1, -2, -3, -4, -5]);
    }
}
