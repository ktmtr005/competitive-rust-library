#![allow(dead_code)]
use cargo_snippet::snippet;

#[snippet]
fn fastout(s: &str) {
    use std::io::{stdout, BufWriter, Write};
    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", s).expect("failed to write data.");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_write() {
        fastout(&format!("{} {}", "Hello, ", "World!"));
        fastout("2 7 1 8 2 8");
    }
}
