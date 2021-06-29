use std::path::Path;

use crate::shared::{comments, decode_jis};
use nom::{
    bytes::{
        complete::{tag, take_until},
        streaming::is_not,
    },
    character::complete::char,
    combinator::{map, map_res, opt},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Enumerates the modules's possible errors
#[derive(Error, Debug)]
pub enum KradError {
    /// Error while parsing kradfile
    #[error("Error while parsing kradfile")]
    Parse,

    /// Error while reading kradfile
    #[error("Error while reading kradfile")]
    Io(#[from] std::io::Error),
}

const SEPARATOR: &[u8] = " : ".as_bytes();

/// A decomposition of a kanji into its constituent radicals
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decomposition {
    /// The kanji character
    pub kanji: String,

    /// A list of characters representing the radicals in the kanji
    pub radicals: Vec<String>,
}

type KradResult = Result<Vec<Decomposition>, KradError>;

/// Parses a kradfile or kradfile2 and returns
/// the list of kanji radical decompositions
///
/// # Arguments
///
/// * `path` - A path to the kradfile
pub fn parse_file<P: AsRef<Path>>(path: P) -> KradResult {
    parse_file_implementation(path.as_ref())
}

// Monomorphisation bloat avoidal splitting
fn parse_file_implementation(path: &Path) -> KradResult {
    std::fs::read(path)
        .map_err(|err| err.into())
        .and_then(|b| parse_bytes(&b))
}

/// Parses the contents of a kradfile or kradfile2 and returns
/// the list of kanji radical decompositions
///
/// # Arguments
///
/// * `b` - The bytes to parse
pub fn parse_bytes(b: &[u8]) -> KradResult {
    lines(b).map(|(_i, o)| o).map_err(|_err| KradError::Parse)
}

fn lines(b: &[u8]) -> IResult<&[u8], Vec<Decomposition>> {
    separated_list1(char('\n'), next_kanji)(b)
}

fn next_kanji(b: &[u8]) -> IResult<&[u8], Decomposition> {
    map(
        separated_pair(comments, opt(char('\n')), kanji_line),
        |(_comments, kanji)| kanji,
    )(b)
}

fn kanji_line(b: &[u8]) -> IResult<&[u8], Decomposition> {
    map(
        separated_pair(kanji, tag(SEPARATOR), radicals),
        |(kanji, radicals)| Decomposition { kanji, radicals },
    )(b)
}

fn kanji(b: &[u8]) -> IResult<&[u8], String> {
    map_res(take_until(" "), decode_jis)(b)
}

fn radicals(b: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list1(char(' '), radical)(b)
}

fn radical(b: &[u8]) -> IResult<&[u8], String> {
    map_res(is_not(" \n"), decode_jis)(b)
}
