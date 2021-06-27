extern crate kanji_api;

use anyhow::Result;
use kanji_api::krad::{lines, decode_jis};
use std::fs;

fn main() -> Result<()> {
    kanji_parse_playground()
    // parse_kradfile2()
}

fn kanji_parse_playground() -> Result<()> {
    const STUFF: &[&[u8]] = &[
        &[0x8F, 0xB0, 0xA1], 
        &[0xB0, 0xEC], 
        &[0xD2, 0xB1],
    ];
    for thing in STUFF {
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
                nom::Err::Error(err) => println!("Error: {:?}", err.code),
                nom::Err::Failure(err) => println!("Failure: {:?}", err.code),
            },
        },
        Err(err) => println!("{}", err),
    }
    Ok(())
}