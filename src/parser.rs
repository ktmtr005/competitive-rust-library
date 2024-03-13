#![allow(dead_code)]

use cargo_snippet::snippet;

#[snippet]
fn parse_all<T: std::str::FromStr>(s: String) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split_whitespace()
        .map(|x| x.parse().expect("failed to parse."))
        .collect::<Vec<T>>()
}

#[snippet]
fn vectors(s: String) -> Vec<String> {
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
    fn test_vectors() {
        let input = String::from("Foo Bar\nFooBar\n");
        let res = vec!["Foo Bar".to_string(), "FooBar".to_string()];
        assert_eq!(vectors(input), res);
    }
}
