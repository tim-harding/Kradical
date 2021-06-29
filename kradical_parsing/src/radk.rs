use crate::shared::{comments, decode_jis};
use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use jis::jis212_to_utf8;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while, take_while1, take_while_m_n},
    character::{complete::space0, is_alphanumeric, is_digit},
    combinator::{eof, map, map_res, success, value},
    multi::many_till,
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use std::{path::Path, string::FromUtf8Error};
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum RadkError {
    #[error("Could not parse stroke order as u8")]
    StrokeOrder,

    #[error("Could not parse alternate representation as a glyph")]
    NotGlyph,

    #[error("Invalid kanji line")]
    EucJp,

    #[error("Error while parsing kradfile")]
    Parse,

    #[error("Error while reading kradfile")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Radical {
    pub glyph: String,
    pub strokes: u8,
    pub alternate: Alternate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expansion {
    pub radical: Radical,
    pub kanji: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Alternate {
    Image(String),
    Glyph(String),
    None,
}

type RadkResult = Result<Vec<Expansion>, RadkError>;

/// Parses a kradfile or kradfile2 and returns
/// the list of kanji radical decompositions
///
/// # Arguments
///
/// * `path` - A path to the kradfile
pub fn parse_file<P: AsRef<Path>>(path: P) -> RadkResult {
    parse_file_implementation(path.as_ref())
}

// Monomorphisation bloat avoidal splitting
fn parse_file_implementation(path: &Path) -> RadkResult {
    std::fs::read(path)
        .map_err(|err| err.into())
        .and_then(|b| parse_bytes(&b))
}

/// Parses the contents of a kradfile or kradfile2 and returns
/// the list of kanji radical decompositions
///
/// # Arguments
///
/// * `path` - A path to the kradfile
pub fn parse_bytes(b: &[u8]) -> RadkResult {
    lines(b).map(|(_i, o)| o).map_err(|_err| RadkError::Parse)
}

fn lines(b: &[u8]) -> IResult<&[u8], Vec<Expansion>> {
    map(many_till(kanji, eof), |(kanji, _)| kanji)(b)
}

fn kanji(b: &[u8]) -> IResult<&[u8], Expansion> {
    map(
        pair(comments, separated_pair(ident_line, tag("\n"), kanji_lines)),
        |(_, (ident, kanji))| Expansion { radical: ident, kanji },
    )(b)
}

fn kanji_lines(b: &[u8]) -> IResult<&[u8], Vec<String>> {
    map_res(take_while(is_eucjp_or_space), from_kanji_line)(b)
}

fn is_eucjp_or_space(b: u8) -> bool {
    b.is_ascii_whitespace() || !b.is_ascii()
}

fn from_kanji_line(b: &[u8]) -> Result<Vec<String>, RadkError> {
    Ok(EUCJPEncoding
        .decode(b, DecoderTrap::Replace)
        .map_err(|_err| RadkError::EucJp)?
        .graphemes(true)
        .filter_map(|s| {
            if s.chars().take(1).any(|c| c.is_ascii_whitespace()) && s.chars().count() == 1 {
                None
            } else {
                Some(s.into())
            }
        })
        .collect())
}

fn ident_line(b: &[u8]) -> IResult<&[u8], Radical> {
    map(
        tuple((ident_line_token, radical, strokes, alternate)),
        |(_, radical, strokes, alternate)| Radical {
            glyph: radical,
            strokes,
            alternate,
        },
    )(b)
}

fn alternate(b: &[u8]) -> IResult<&[u8], Alternate> {
    alt((hex, image, success(Alternate::None)))(b)
}

fn image(b: &[u8]) -> IResult<&[u8], Alternate> {
    map_res(take_while1(is_alphanumeric), from_image)(b)
}

fn from_image(b: &[u8]) -> Result<Alternate, FromUtf8Error> {
    String::from_utf8(b.into()).map(|s| Alternate::Image(s))
}

fn hex(b: &[u8]) -> IResult<&[u8], Alternate> {
    map_res(take_while_m_n(4, 4, is_hex_digit), from_hex)(b)
}

fn from_hex(b: &[u8]) -> Result<Alternate, RadkError> {
    let s = std::str::from_utf8(b).map_err(|_| RadkError::NotGlyph)?;
    let code = u16::from_str_radix(&s, 16).map_err(|_| RadkError::NotGlyph)?;
    jis212_to_utf8(code)
        .ok_or(RadkError::NotGlyph)
        .map(|s| Alternate::Glyph(s.to_string()))
}

fn is_hex_digit(b: u8) -> bool {
    let c = b as char;
    (c.is_ascii_uppercase() || c.is_ascii_digit()) && c.is_digit(16)
}

fn ident_line_token(b: &[u8]) -> IResult<&[u8], ()> {
    terminated(value((), tag("$")), space0)(b)
}

fn radical(b: &[u8]) -> IResult<&[u8], String> {
    terminated(map_res(take(2u8), decode_jis), space0)(b)
}

fn strokes(b: &[u8]) -> IResult<&[u8], u8> {
    terminated(map_res(take_while(is_digit), parse_number), space0)(b)
}

fn parse_number(b: &[u8]) -> Result<u8, RadkError> {
    String::from_utf8(b.into())
        .map_err(|_err| RadkError::StrokeOrder)?
        .parse()
        .map_err(|_err| RadkError::StrokeOrder)
}
