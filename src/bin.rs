extern crate kanji_api;

use anyhow::Result;
use nom::{
    bytes::complete::{take, take_until},
    combinator::eof,
    multi::many_till,
    sequence::pair,
    IResult,
};
use std::fs;

const NEWLINE: &[u8] = &[0x0A];

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

fn lines(b: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    let (i, o) = many_till(line, eof)(b)?;
    let (lines, _end) = o;
    Ok((i, lines))
}

fn line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, o) = pair(take_until(NEWLINE), take(1u8))(b)?;
    let (line, _newline) = o;
    Ok((i, line))
}
