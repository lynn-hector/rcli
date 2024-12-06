use std::io::Read;
use anyhow::Result;

pub fn get_reader(input: &str) -> Result<Box<dyn Read>>{
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::io::stdin())
    };
    Ok(reader)
}