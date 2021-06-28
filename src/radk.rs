use crate::{jis212::jis212_to_utf8, shared::decode_jis};
use nom::{IResult, bytes::complete::{tag, take_until, take_while, take_while_m_n}, character::{is_alphanumeric, is_digit}, combinator::{map, map_res, value}, sequence::{pair, separated_pair}};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RadkError {
    #[error("Could not parse stroke order as u8")]
    StrokeOrder,

    #[error("Could not parse alternate representation as a glyph")]
    NotGlyph,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ident {
    radical: String,
    strokes: u8,
    alternate: Alternate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Alternate {
    Image(String),
    Glyph(String),
    None,
}

fn ident_line(b: &[u8]) -> IResult<&[u8], Ident> {
    map(pair(token_radical_strokes, tag("\n")), |(ident, _)| Ident {
        radical: "".to_string(),
        strokes: 0,
        alternate: Alternate::None,
    })(b)
}

fn image(b: &[u8]) -> IResult<&[u8], &str> {
    map_res(take_while(is_alphanumeric), std::str::from_utf8)(b)
}

fn hex(b: &[u8]) -> IResult<&[u8], String> {
    map_res(take_while_m_n(4, 4, is_hex_digit), from_hex)(b)
}

fn from_hex(b: &[u8]) -> Result<String, RadkError> {
    let s = std::str::from_utf8(b).map_err(|_| RadkError::NotGlyph)?;
    let code = u16::from_str_radix(&s, 16).map_err(|_| RadkError::NotGlyph)?;
    jis212_to_utf8(code)
        .ok_or(RadkError::NotGlyph)
        .map(|s| s.to_string())
}

fn is_hex_digit(b: u8) -> bool {
    let c = b as char;
    (c.is_ascii_uppercase() || c.is_ascii_digit()) && c.is_digit(16)
}

fn token_radical_strokes(b: &[u8]) -> IResult<&[u8], (String, u8)> {
    separated_pair(token_radical, tag(" "), strokes)(b)
}

fn token_radical(b: &[u8]) -> IResult<&[u8], String> {
    map(
        separated_pair(ident_line_token, tag(" "), radical),
        |(_, radical)| radical,
    )(b)
}

fn ident_line_token(b: &[u8]) -> IResult<&[u8], ()> {
    value((), tag("$"))(b)
}

fn radical(b: &[u8]) -> IResult<&[u8], String> {
    map_res(take_until(" "), decode_jis)(b)
}

fn strokes(b: &[u8]) -> IResult<&[u8], u8> {
    map_res(take_while(is_digit), parse_number)(b)
}

fn parse_number(b: &[u8]) -> Result<u8, RadkError> {
    String::from_utf8(b.into())
        .map_err(|_err| RadkError::StrokeOrder)?
        .parse()
        .map_err(|_err| RadkError::StrokeOrder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::{EMPTY, NEWLINE};

    // JIS X 0213
    // $ 一 1
    const IDENT_LINE_SIMPLE: &[u8] = &[0x24, 0x20, 0xB0, 0xEC, 0x20, 0x31, 0x0A];

    // $ Ф 2 js02
    const IDENT_LINE_FULL_IMG: &[u8] = &[
        0x24, 0x20, 0xD0, 0xA4, 0x20, 0x32, 0x20, 0x6A, 0x73, 0x30, 0x32, 0x0A,
    ];

    // $ �� 3 6134
    const IDENT_LINE_FULL_JIS: &[u8] = &[
        0x24, 0x20, 0xB9, 0xFE, 0x20, 0x33, 0x20, 0x36, 0x31, 0x33, 0x34, 0x0A,
    ];

    fn parsed_radical_simple() -> Ident {
        Ident {
            radical: "一".to_string(),
            strokes: 1,
            alternate: Alternate::None,
        }
    }

    #[test]
    fn radk_strokes() {
        let res = strokes("12".as_bytes());
        assert_eq!(res, Ok((EMPTY, 12)));
    }

    #[test]
    fn radk_radical() {
        let radical_and_space = &IDENT_LINE_SIMPLE[2..];
        let res = radical(radical_and_space);
        assert_eq!(res, Ok((&IDENT_LINE_SIMPLE[4..], "一".to_string())))
    }

    #[test]
    fn radk_ident_radical() {
        let res = token_radical(IDENT_LINE_SIMPLE);
        assert_eq!(res, Ok((&IDENT_LINE_SIMPLE[4..], "一".to_string())));
    }

    #[test]
    fn radk_ident_radical_strokes() {
        let radical_and_space = &IDENT_LINE_SIMPLE;
        let res = token_radical_strokes(radical_and_space);
        let k = parsed_radical_simple();
        assert_eq!(res, Ok((NEWLINE, (k.radical, k.strokes))));
    }

    #[test]
    fn radk_simple_ident_line() {
        let res = ident_line(IDENT_LINE_SIMPLE);
        assert_eq!(res, Ok((EMPTY, parsed_radical_simple())));
    }
    
    #[test]
    fn radk_hex() {
        let res = hex("6134".as_bytes());
        assert_eq!(res, Ok((EMPTY, "辶".to_string())));
    }
    
    #[test]
    fn radk_image() {
        let res = image("js02".as_bytes());
        assert_eq!(res, Ok((EMPTY, "js02")));
    }

    /*
    #[test]
    fn radk_ident_line() {
        let res = ident_line(IDENT_LINE_FULL);
        assert_eq!(res, Ok((EMPTY, parsed_radical())));
    }
    */
}
