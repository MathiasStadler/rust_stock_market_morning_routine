// FROM HERE
// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

use std::error::Error;
// use anyhow::{bail, Result};
// type BoxResult<T> = Result<T>;
type BoxResult<T> = Result<T, Box<dyn Error>>;

fn run(s: &str) -> BoxResult<i32> {
    if s.len() == 0 {
        panic!("empty string");
    }
    Ok(s.trim().parse()?)
}

fn main() {
    println!("{:?}", run("23"));
    println!("{:?}", run("2x"));
    println!("{:?}", run(""));
}

//expected output
// Ok(23)
// Err(ParseIntError { kind: InvalidDigit })
// Err(StringError("empty string"))