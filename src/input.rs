#![allow(dead_code)]

use cargo_snippet::snippet;

#[snippet("read")]
fn read() -> String {
    let mut buf = String::new();
    std::io::stdin()
        .read_line(&mut buf)
        .expect("failed to read line.");
    buf.trim().to_string()
}

#[snippet(include = "read")]
#[snippet("read_vec")]
fn read_vec<T: std::str::FromStr>() -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    read()
        .split_whitespace()
        .map(|x| x.parse().expect("failed to parse."))
        .collect::<Vec<T>>()
}
