use super::jis213::jis_to_utf8;

use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use nom::{
    bytes::complete::take_until, character::complete::char, combinator::value,
    multi::separated_list0, sequence::pair, IResult,
};
use thiserror::Error;

/// Enumerates the modules's possible errors
#[derive(Error, Debug)]
pub enum KradError {
    /// Invalid JIS X 0213 codepoint
    #[error("Invalid JIS X 0213 codepoint")]
    Jis,

    /// Invalid EUC-JP codepoint
    #[error("Invalid EUC-JP codepoint")]
    EucJp,

    /// Error while parsing kradfile
    #[error("Error while parsing kradfile")]
    Parse,

    /// Error while reading kradfile
    #[error("Error while reading kradfile")]
    Io(#[from] std::io::Error),
}

pub fn comments(b: &[u8]) -> IResult<&[u8], ()> {
    value((), separated_list0(char('\n'), comment))(b)
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(char('#'), take_until("\n")))(b)
}

// Todo: Write tests
pub fn decode_jis(b: &[u8]) -> Result<String, KradError> {
    match b.len() {
        2 => {
            let code = bytes_to_u32(b);
            jis_to_utf8(code)
                .map(|unicode| unicode.to_string())
                .ok_or(KradError::Jis.into())
        }
        3 => EUCJPEncoding
            .decode(b, DecoderTrap::Strict)
            .map_err(|_| KradError::EucJp.into()),
        _ => Err(KradError::Jis.into()),
    }
}

// Todo: Handle overflow
pub fn bytes_to_u32(b: &[u8]) -> u32 {
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
        assert_eq!(res, Ok((NEWLINE, ())));
    }
}
