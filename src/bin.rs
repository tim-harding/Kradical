extern crate kanji_api;

use anyhow::Result;
use nom::{IResult, bytes::complete::{take, take_until}, combinator::{eof}, dbg_dmp, multi::{many_till}};
use std::fs;

const NEWLINE: &[u8] = &[0x0A];

fn main() -> Result<()> {
    match fs::read("./dictionary-files/downloads/kradfile2") {
        Ok(text) => {
            match lines(&text) {
                Ok(parsed) => {
                    let (_, results) = parsed;
                    println!("{:?}", results.0.len());
                }
                Err(err) => match err {
                    nom::Err::Incomplete(needed) => println!("Incomplete: {:?}", needed),
                    nom::Err::Error(err) => println!("Error: {:?}", err.input.len()),
                    nom::Err::Failure(err) => println!("Failure: {:?}", err.code),
                },
            }
            let mut count = 0;
            for byte in text {
                if byte == 0x0A {
                    count += 1
                }
            }
            println!("{}", count)
        }
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn lines(b: &[u8]) -> IResult<&[u8], (Vec<&[u8]>, &[u8])> {
    many_till(line, eof)(b)
}

fn line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let line = dbg_dmp(take_until(NEWLINE), "line")(b)?;
    println!("{}, {}", line.0.len(), line.1.len());
    // println!("{}", String::from_utf8_lossy(line.0));
    // Ok(line)
    take(line.1.len() + 1)(b)
}
