use crate::shared::decode_jis;
use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::is_digit,
    combinator::{map, map_res, value},
    sequence::{pair, separated_pair},
    IResult,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RadkError {
    #[error("Could not parse stroke order as u8")]
    StrokeOrder,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ident {
    radical: String,
    strokes: u8,
}

fn ident_line(b: &[u8]) -> IResult<&[u8], Ident> {
    map(pair(token_radical_strokes, tag("\n")), |(ident, _)| ident)(b)
}

fn token_radical_strokes(b: &[u8]) -> IResult<&[u8], Ident> {
    map(
        separated_pair(token_radical, tag(" "), strokes),
        |(radical, strokes)| Ident { radical, strokes },
    )(b)
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

    const SIMPLE_IDENT_LINE: &[u8] = &[0x24, 0x20, 0xB0, 0xEC, 0x20, 0x31, 0x0A];

    fn parsed_radical() -> Ident {
        Ident {
            radical: "一".to_string(),
            strokes: 1,
        }
    }

    #[test]
    fn radk_strokes() {
        let res = strokes("12".as_bytes());
        assert_eq!(res, Ok((EMPTY, 12)));
    }

    #[test]
    fn radk_radical() {
        let radical_and_space = &SIMPLE_IDENT_LINE[2..];
        let res = radical(radical_and_space);
        assert_eq!(res, Ok((&SIMPLE_IDENT_LINE[4..], "一".to_string())))
    }

    #[test]
    fn radk_ident_radical() {
        let res = token_radical(SIMPLE_IDENT_LINE);
        assert_eq!(res, Ok((&SIMPLE_IDENT_LINE[4..], "一".to_string())));
    }

    #[test]
    fn radk_ident_radical_strokes() {
        let radical_and_space = &SIMPLE_IDENT_LINE;
        let res = token_radical_strokes(radical_and_space);
        assert_eq!(res, Ok((NEWLINE, parsed_radical())));
    }

    #[test]
    fn radk_simple_ident_line() {
        let res = ident_line(SIMPLE_IDENT_LINE);
        assert_eq!(res, Ok((EMPTY, parsed_radical())));
    }
}
