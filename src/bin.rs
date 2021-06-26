extern crate kanji_api;

use anyhow::Result;
use kanji_api::krad::lines;
use std::fs;

fn main() -> Result<()> {
    match fs::read("./dictionary-files/downloads/kradfile2") {
        Ok(text) => match lines(&text) {
            Ok(parsed) => {
                let (_, results) = parsed;
                println!("{:?}", results.len());
            }
            Err(err) => match err {
                nom::Err::Incomplete(needed) => println!("Incomplete: {:?}", needed),
                nom::Err::Error(err) => println!("Error: {:?}", err.input.len()),
                nom::Err::Failure(err) => println!("Failure: {:?}", err.code),
            },
        },
        Err(err) => println!("{}", err),
    }
    Ok(())
}