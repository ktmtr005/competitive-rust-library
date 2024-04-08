#[macro_export]
macro_rules! bufwrite {
            ($($arg:tt)*) => {{
                use ::std::io::{BufWriter, Write};
                let mut out = BufWriter::new(::std::io::stdout().lock());
                ::std::write!(out, $($arg)*).unwrap();
            }};
        }

#[macro_export]
macro_rules! bufwriteln {
            ($($arg:tt)*) => {{
                use ::std::io::{BufWriter, Write};
                let mut out = BufWriter::new(::std::io::stdout().lock());
                ::std::writeln!(out, $($arg)*).unwrap();
            }};
        }

#[cfg(test)]
mod tests {
    #[test]
    fn test_write() {
        bufwrite!("{} {} {}\n", 1, 2, 3);
        bufwriteln!("{:.10}", std::f64::consts::E);
    }
}
