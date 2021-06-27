extern crate kanji_api;

use anyhow::Result;
use kanji_api::krad::{lines, decode_jis};
use std::fs;

fn main() -> Result<()> {
    let stuff = [
        [0xB0u8, 0xA1u8],
        [0xA1u8, 0xC3u8],
        [0xB0u8, 0xECu8],
    ];
    for thing in stuff {
        let res = decode_jis(&thing);
        println!("{:?}", res);
    }
    Ok(())
}

fn parse_kradfile2() -> Result<()> {
    match fs::read("./dictionary-files/downloads/kradfile") {
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