extern crate kanji_api;

use std::fs;
use anyhow::Result;
use nom::IResult;

fn main() -> Result<()> {
    match fs::read("./dictionary-files/downloads/kradfile2") {
        Ok(text) => {
            match lines(&text) {
                Ok(parsed) => println!("{:?}", parsed),
                Err(err) => println!("{:?}", err)
            }
        }
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn lines<'a>(b: &'a[u8]) -> IResult<&'a[u8], &'a[u8]> {
    nom::bytes::complete::tag(&[0x0A])(b)
}