// FROM HERE
// https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html

// without question mark as error handling

use std::{error::Error, string::ParseError};
// use anyhow::{bail, Result};
// type BoxResult<T> = Result<T>;
type BoxResult<T> = Result<T, Box<dyn Error>>;


fn run(s: &str) -> BoxResult<i32> {
    if s.len() == 0 {
        panic!("Err(Empty string)");
    }
    // Ok(s.trim().parse() )

    let number:i32 = match s.trim().parse() {
        Ok(number)  => number,
        Err(e)  => return Err(Box::new(e)),
    };
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

// RUST_BACKTRACE=full cargo run --package rust_stock_market_morning_routine --example main_run_error_log_handling
// RUST_BACKTRACE=1 cargo run --package rust_stock_market_morning_routine --example main_run_error_log_handling