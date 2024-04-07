#![allow(dead_code)]
pub fn fastout(s: &str) {
    use std::io::{stdout, BufWriter, Write};
    let mut out = BufWriter::new(stdout().lock());
    writeln!(out, "{}", s).expect("failed to write data.");
}
