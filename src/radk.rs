use std::{borrow::Cow, string::FromUtf8Error};

use crate::{jis212::jis212_to_utf8, shared::decode_jis};
use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use nom::{
    bitvec::view::AsBits,
    branch::alt,
    bytes::complete::{tag, take_until, take_while, take_while1, take_while_m_n},
    character::{is_alphanumeric, is_digit},
    combinator::{map, map_opt, map_res, opt, success, value},
    sequence::{pair, separated_pair},
    IResult,
};
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

fn kanji_line(b: &[u8]) -> IResult<&[u8], String> {
    map_res(take_until("\n"), from_kanji_line)(b)
}

fn from_kanji_line(b: &[u8]) -> Result<String, Cow<'static, str>> {
    EUCJPEncoding.decode(b, DecoderTrap::Replace)
}

fn ident_line(b: &[u8]) -> IResult<&[u8], Ident> {
    map(
        pair(token_radical_strokes, alternate),
        |((radical, strokes), alternate)| Ident {
            radical,
            strokes,
            alternate,
        },
    )(b)
}

fn alternate(b: &[u8]) -> IResult<&[u8], Alternate> {
    alt((alternate_some, success(Alternate::None)))(b)
}

fn alternate_some(b: &[u8]) -> IResult<&[u8], Alternate> {
    map(
        pair(tag(" ".as_bytes()), alt((hex, image))),
        |(_, alternate)| alternate,
    )(b)
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
    const IDENT_LINE_SIMPLE: &[u8] = &[0x24, 0x20, 0xB0, 0xEC, 0x20, 0x31];

    // $ Ф 2 js02
    const IDENT_LINE_FULL_IMG: &[u8] = &[
        0x24, 0x20, 0xD0, 0xA4, 0x20, 0x32, 0x20, 0x6A, 0x73, 0x30, 0x32,
    ];

    // $ ˻ 3 3D38
    const IDENT_LINE_FULL_JIS: &[u8] = &[
        0x24, 0x20, 0xCB, 0xBB, 0x20, 0x33, 0x20, 0x33, 0x44, 0x33, 0x38,
    ];

    // Radkfile line 548
    const KANJI_LINE: &[u8] = &[
        0xB0, 0xD4, 0xB1, 0xD9, 0xB2, 0xB1, 0xB2, 0xF7, 0xB2, 0xF8, 0xB2, 0xF9, 0xB2, 0xFA, 0xB2,
        0xFB, 0xB3, 0xB4, 0xB3, 0xE6, 0xB4, 0xB7, 0xB4, 0xB8, 0xB6, 0xB1, 0xB8, 0xE7, 0xB9, 0xB1,
        0xB9, 0xB2, 0xB9, 0xFB, 0xBA, 0xA8, 0xBB, 0xB4, 0xBE, 0xF0, 0xBF, 0xB5, 0xC0, 0xAD, 0xC0,
        0xCB, 0xC1, 0xFE, 0xC2, 0xC6, 0xC4, 0xF0, 0xC5, 0xE9, 0xC6, 0xB4, 0xC6, 0xD7, 0xC7, 0xBA,
        0xC9, 0xDD, 0xCA, 0xB0, 0xCB, 0xBB, 0xCB, 0xFD, 0xCC, 0xFB, 0xCE, 0xE7, 0x0A,
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
        assert_eq!(res, Ok((EMPTY, (k.radical, k.strokes))));
    }

    #[test]
    fn radk_simple_ident_line() {
        let res = ident_line(IDENT_LINE_SIMPLE);
        assert_eq!(res, Ok((EMPTY, parsed_radical_simple())));
    }

    #[test]
    fn radk_hex() {
        let res = hex("6134".as_bytes());
        assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
    }

    #[test]
    fn radk_image() {
        let res = image("js02".as_bytes());
        assert_eq!(res, Ok((EMPTY, Alternate::Image("js02".to_string()))));
    }

    #[test]
    fn radk_alt_is_hex() {
        let res = alternate(" 6134".as_bytes());
        assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
    }

    #[test]
    fn radk_alt_is_image() {
        let res = alternate(" js02".as_bytes());
        assert_eq!(res, Ok((EMPTY, Alternate::Image("js02".to_string()))));
    }

    #[test]
    fn radk_alt_is_none() {
        let res = alternate(EMPTY);
        assert_eq!(res, Ok((EMPTY, Alternate::None)));
    }

    #[test]
    fn radk_image_ident_line() {
        let res = ident_line(IDENT_LINE_FULL_IMG);
        assert_eq!(
            res,
            Ok((
                EMPTY,
                Ident {
                    radical: "个".to_string(),
                    strokes: 2,
                    alternate: Alternate::Image("js02".to_string()),
                }
            ))
        )
    }

    #[test]
    fn radk_glyph_ident_line() {
        let res = ident_line(IDENT_LINE_FULL_JIS);
        assert_eq!(
            res,
            Ok((
                EMPTY,
                Ident {
                    radical: "忙".to_string(),
                    strokes: 3,
                    alternate: Alternate::Glyph("\u{5FC4}".to_string()),
                }
            ))
        )
    }

    #[test]
    fn radk_kanji_line() {
        let expected = "惟悦憶快怪悔恢懐慨恰慣憾怯悟恒慌惚恨惨情慎性惜憎惰悌悼憧惇悩怖憤忙慢愉怜";
        let res = kanji_line(KANJI_LINE);
        assert_eq!(res, Ok((NEWLINE, expected.to_string())));
    }

    /*
    #[test]
    fn radk_ident_line() {
        let res = ident_line(IDENT_LINE_FULL);
        assert_eq!(res, Ok((EMPTY, parsed_radical())));
    }
    */
}
