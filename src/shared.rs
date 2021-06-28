use super::jis213::jis213_to_utf8;
use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use nom::{
    bytes::complete::take_until,
    character::complete::{char, multispace0},
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, pair},
    IResult,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    /// Invalid JIS X 0213 codepoint
    #[error("Invalid JIS X 0213 codepoint")]
    Jis,

    /// Invalid EUC-JP codepoint
    #[error("Invalid EUC-JP codepoint")]
    EucJp,

    /// Incorrect number of bytes for JIS or EUC-JP
    #[error("Incorrect number of bytes for JIS or EUC-JP")]
    Unknown,
}

pub fn comments(b: &[u8]) -> IResult<&[u8], ()> {
    value(
        (),
        delimited(
            multispace0,
            separated_list0(char('\n'), comment),
            multispace0,
        ),
    )(b)
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(char('#'), take_until("\n")))(b)
}

pub fn decode_jis(b: &[u8]) -> Result<String, SharedError> {
    match b.len() {
        2 => {
            let code = bytes_to_u32(b);
            jis213_to_utf8(code)
                .map(|unicode| unicode.to_string())
                .ok_or(SharedError::Jis)
        }
        3 => EUCJPEncoding
            .decode(b, DecoderTrap::Strict)
            .map_err(|_| SharedError::EucJp),
        _ => Err(SharedError::Unknown),
    }
}

fn bytes_to_u32(b: &[u8]) -> u32 {
    let mut out = 0u32;
    for (i, byte) in b.iter().rev().enumerate() {
        out += (*byte as u32) << 8u32 * (i as u32);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;

    #[test]
    fn is_comment() {
        let res = comment(COMMENT_LINE);
        assert_eq!(res, Ok((NEWLINE, ())));
    }

    #[test]
    fn is_comment_short() {
        let res = comment("#\n".as_bytes());
        assert_eq!(res, Ok((NEWLINE, ())));
    }

    #[test]
    fn multiple_comment_lines() {
        let line = vec![COMMENT_LINE, COMMENT_LINE].join("".as_bytes());
        let res = comments(&line);
        assert_eq!(res, Ok((EMPTY, ())));
    }
}
