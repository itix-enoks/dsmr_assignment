use std::str::Lines;

use crate::error::MainError;

pub fn parse(input: &str) -> Result<(), MainError> {
    let lines = input.lines();
    for l in lines {
        println!("info: line: {}", l);
   }

    Ok(())
}
