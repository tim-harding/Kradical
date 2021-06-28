use crate::{
    jis212::jis212_to_utf8,
    shared::{comments, decode_jis},
};
use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while, take_while1, take_while_m_n},
    character::{complete::space0, is_alphanumeric, is_digit},
    combinator::{map, map_res, success, value},
    sequence::{pair, separated_pair, terminated, tuple},
    IResult,
};
use std::{borrow::Cow, string::FromUtf8Error};
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

// Todo: Grapheme clusters
// https://crates.io/crates/unicode-segmentation

#[derive(Debug, Error)]
pub enum RadkError {
    #[error("Could not parse stroke order as u8")]
    StrokeOrder,

    #[error("Could not parse alternate representation as a glyph")]
    NotGlyph,

    #[error("Invalid kanji line")]
    EucJp,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ident {
    radical: String,
    strokes: u8,
    alternate: Alternate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Inclusion {
    ident: Ident,
    kanji: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Alternate {
    Image(String),
    Glyph(String),
    None,
}

fn kanji(b: &[u8]) -> IResult<&[u8], Inclusion> {
    map(
        pair(comments, separated_pair(ident_line, tag("\n"), kanji_lines)),
        |(_, (ident, kanji))| Inclusion { ident, kanji },
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

fn ident_line(b: &[u8]) -> IResult<&[u8], Ident> {
    map(
        tuple((ident_line_token, radical, strokes, alternate)),
        |(_, radical, strokes, alternate)| Ident {
            radical,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::{COMMENT_LINE, EMPTY, NEWLINE};

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
        0xC9, 0xDD, 0xCA, 0xB0, 0xCB, 0xBB, 0xCB, 0xFD, 0xCC, 0xFB, 0xCE, 0xE7,
    ];

    // 573 - 574
    const KANJI_MULTILINE: &[u8] = &[
        0x0A, 0xB1, 0xEE, 0xB2, 0xAE, 0xB3, 0xCD, 0xB6, 0xB8, 0xB6, 0xB9, 0xB6, 0xE9, 0xB8, 0xD1,
        0xB9, 0xF6, 0xB9, 0xFD, 0xBA, 0xA6, 0xBA, 0xA9, 0xBB, 0xE2, 0xBC, 0xED, 0xC1, 0xC0, 0xC3,
        0xAC, 0xC3, 0xF6, 0xC6, 0xC8, 0xC7, 0xAD, 0xC7, 0xE2, 0xC8, 0xC8, 0xCC, 0xD4, 0xCD, 0xB1,
        0xCE, 0xC4, 0xCF, 0xB5, 0xDF, 0xCC, 0xE0, 0xBB, 0xE0, 0xBC, 0xE0, 0xBD, 0xE0, 0xBE, 0xE0,
        0xBF, 0xE0, 0xC0, 0xE0, 0xC1, 0xE0, 0xC2, 0xE0, 0xC3, 0xE0, 0xC4, 0xE0, 0xC5, 0x0A, 0xE0,
        0xC6, 0xE0, 0xC8, 0xE0, 0xC9, 0xE0, 0xCA, 0xE0, 0xCB, 0xE0, 0xCC, 0xE0, 0xCD, 0xE0, 0xCE,
        0xE0, 0xCF, 0xE0, 0xD0, 0xE0, 0xD1, 0xE0, 0xD3, 0xE0, 0xD5, 0xE0, 0xD6, 0xE0, 0xD7, 0xE0,
        0xD8, 0xE0, 0xDA, 0xE0, 0xDC, 0xE9, 0xA4, 0xEB, 0xD4, 0xED, 0xF8, 0x0A,
    ];

    // 588 - 590
    const FULL_KANJI: &[u8] = &[
        0x24, 0x20, 0xCB, 0xAE, 0x20, 0x33, 0x20, 0x6B, 0x6F, 0x7A, 0x61, 0x74, 0x6F, 0x52, 0x0A,
        0xB0, 0xEA, 0xB3, 0xC7, 0xB3, 0xD4, 0xB6, 0xBF, 0xB6, 0xC1, 0xB6, 0xC2, 0xB7, 0xB4, 0xB7,
        0xB7, 0xB9, 0xD9, 0xBC, 0xC3, 0xBC, 0xD9, 0xC5, 0xA1, 0xC5, 0xA2, 0xC5, 0xD4, 0xC6, 0xE1,
        0xC9, 0xF4, 0xCB, 0xAE, 0xCC, 0xEC, 0xCC, 0xED, 0xCD, 0xB9, 0xCF, 0xAD, 0xCF, 0xB1, 0xCF,
        0xBA, 0xD3, 0xEC, 0xD5, 0xB1, 0xD9, 0xE8, 0xDA, 0xB3, 0xDB, 0xEB, 0xDC, 0xBF, 0xDC, 0xDA,
        0xE0, 0xE7, 0xEA, 0xA7, 0xED, 0xB6, 0xEE, 0xB7, 0xEE, 0xB8, 0xEE, 0xB9, 0x0A, 0xEE, 0xBA,
        0xEE, 0xBB, 0xEE, 0xBC, 0xEE, 0xBD, 0xEE, 0xBE, 0xEE, 0xBF, 0xEE, 0xC0, 0xEE, 0xC1, 0xEE,
        0xC2, 0xEE, 0xC3, 0x0A,
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
        let res = strokes(b"12");
        assert_eq!(res, Ok((EMPTY, 12)));
    }

    #[test]
    fn radk_radical() {
        let radical_and_space = &IDENT_LINE_SIMPLE[2..];
        let res = radical(radical_and_space);
        assert_eq!(res, Ok((&IDENT_LINE_SIMPLE[5..], "一".to_string())))
    }

    #[test]
    fn radk_simple_ident_line() {
        let res = ident_line(IDENT_LINE_SIMPLE);
        assert_eq!(res, Ok((EMPTY, parsed_radical_simple())));
    }

    #[test]
    fn radk_hex() {
        let res = hex(b"6134");
        assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
    }

    #[test]
    fn radk_image() {
        let res = image(b"js02");
        assert_eq!(res, Ok((EMPTY, Alternate::Image("js02".to_string()))));
    }

    #[test]
    fn radk_alt_is_hex() {
        let res = alternate(b"6134");
        assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
    }

    #[test]
    fn radk_alt_is_image() {
        let res = alternate(b"js02");
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
        let expected: Vec<String> = [
            "惟", "悦", "憶", "快", "怪", "悔", "恢", "懐", "慨", "恰", "慣", "憾", "怯", "悟",
            "恒", "慌", "惚", "恨", "惨", "情", "慎", "性", "惜", "憎", "惰", "悌", "悼", "憧",
            "惇", "悩", "怖", "憤", "忙", "慢", "愉", "怜",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        let res = kanji_lines(KANJI_LINE);
        assert_eq!(res, Ok((EMPTY, expected)));
    }

    #[test]
    fn radk_kanji_multiline() {
        let expected: Vec<String> = [
            "猿", "荻", "獲", "狂", "狭", "狗", "狐", "獄", "狛", "墾", "懇", "獅", "狩", "狙",
            "狸", "猪", "独", "猫", "狽", "犯", "猛", "猶", "猟", "狼", "潴", "犹", "犲", "狃",
            "狆", "狄", "狎", "狒", "狢", "狠", "狡", "狹", "狷", "猗", "猊", "猜", "猖", "猝",
            "猴", "猯", "猩", "猥", "猾", "獏", "獗", "獪", "獨", "獰", "獵", "獺", "蕕", "誑",
            "逖",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        let res = kanji_lines(KANJI_MULTILINE);
        assert_eq!(res, Ok((EMPTY, expected)));
    }

    fn inclusion_expected() -> Inclusion {
        let inc: Vec<String> = [
            "郁", "廓", "郭", "郷", "響", "饗", "郡", "祁", "郊", "蔀", "邪", "邸", "鄭", "都",
            "那", "部", "邦", "爺", "耶", "郵", "廊", "榔", "郎", "嚮", "娜", "揶", "擲", "梛",
            "椰", "槨", "瑯", "螂", "躑", "邨", "邯", "邱", "邵", "郢", "郤", "扈", "郛", "鄂",
            "鄒", "鄙", "鄲", "鄰",
        ]
        .iter()
        .map(|&s| s.into())
        .collect();
        Inclusion {
            ident: Ident {
                radical: "邦".to_string(),
                strokes: 3,
                alternate: Alternate::Image("kozatoR".to_string()),
            },
            kanji: inc,
        }
    }

    #[test]
    fn radk_inclusion() {
        let res = kanji(FULL_KANJI);
        assert_eq!(res, Ok((EMPTY, inclusion_expected())));
    }

    #[test]
    fn radk_inclusion_with_comment() {
        let lines = [COMMENT_LINE, FULL_KANJI].join("".as_bytes());
        let res = kanji(&lines);
        assert_eq!(res, Ok((EMPTY, inclusion_expected())));
    }
}
