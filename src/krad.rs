mod jis213;

use anyhow::Result;
use nom::{
    bytes::{
        complete::{tag, take_until},
        streaming::is_not,
    },
    character::complete::char,
    combinator::{map, map_res, opt, value},
    multi::{separated_list0, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};
use thiserror::Error;

const SEPARATOR: &[u8] = " : ".as_bytes();

// Todo: Shouldn't need to clone this
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KanjiParts<'a> {
    kanji: &'a str,
    radicals: Vec<&'a str>,
}

#[derive(Error, Debug)]
pub enum KradError {
    #[error("Invalid JIS0208 or JIS0212 codepoint")]
    Jis,
}

pub fn lines(b: &[u8]) -> IResult<&[u8], Vec<KanjiParts>> {
    separated_list1(char('\n'), next_kanji)(b)
}

fn next_kanji(b: &[u8]) -> IResult<&[u8], KanjiParts> {
    map(
        separated_pair(comments, opt(char('\n')), kanji_line),
        |(_comments, kanji)| kanji,
    )(b)
}

fn kanji_line(b: &[u8]) -> IResult<&[u8], KanjiParts> {
    map(
        separated_pair(kanji, tag(SEPARATOR), radicals),
        |(kanji, radicals)| KanjiParts { kanji, radicals },
    )(b)
}

fn kanji(b: &[u8]) -> IResult<&[u8], &'static str> {
    map_res(take_until(SEPARATOR), decode_jis)(b)
}

fn radicals(b: &[u8]) -> IResult<&[u8], Vec<&'static str>> {
    separated_list1(char(' '), radical)(b)
}

fn radical(b: &[u8]) -> IResult<&[u8], &'static str> {
    map_res(is_not(" \n"), decode_jis)(b)
}

fn comments(b: &[u8]) -> IResult<&[u8], ()> {
    value((), separated_list0(char('\n'), comment))(b)
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(char('#'), is_not("\n")))(b)
}

pub fn decode_jis(b: &[u8]) -> Result<&'static str> {
    let code = bytes_to_u32(b);
    match b.len() {
        2 | 3 => jis213::decode(code).ok_or(KradError::Jis.into()),
        _ => Err(KradError::Jis.into()),
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
    use anyhow::Result;

    // Todo: Test cases aren't real JIS213,
    // probably because it isn't valid UTF8

    const KANJI_LINE: &[u8] = "��� : �� �� �� �� ��\n".as_bytes();
    const COMMENT_LINE: &[u8] = "# September 2007\n".as_bytes();
    const NEWLINE: &[u8] = "\n".as_bytes();

    fn parsed_kanji() -> KanjiParts<'static> {
        KanjiParts {
            kanji: "���",
            radicals: vec!["��", "��", "��", "��", "��"],
        }
    }

    #[test]
    fn is_comment() -> Result<()> {
        let (_i, o) = comment(COMMENT_LINE)?;
        assert_eq!(o, ());
        Ok(())
    }

    #[test]
    fn multiple_comment_lines() -> Result<()> {
        let line = vec![COMMENT_LINE, COMMENT_LINE].join("".as_bytes());
        let res = comments(&line);
        assert_eq!(res, Ok((NEWLINE, ())));
        Ok(())
    }

    #[test]
    fn parses_radical() -> Result<()> {
        let res = radical("�� �� ��\n".as_bytes())?;
        assert_eq!(res, (" �� ��\n".as_bytes(), "��"));
        Ok(())
    }

    #[test]
    fn parses_radicals() -> Result<()> {
        let res = radicals("�� �� ��\n".as_bytes())?;
        assert_eq!(res, (NEWLINE, vec!["��", "��", "��"]));
        Ok(())
    }

    #[test]
    fn parses_kanji() -> Result<()> {
        let res = kanji_line(KANJI_LINE)?;
        assert_eq!(res, (NEWLINE, parsed_kanji()));
        Ok(())
    }

    #[test]
    fn parses_line_as_kanji() -> Result<()> {
        let res = next_kanji(KANJI_LINE)?;
        assert_eq!(res, (NEWLINE, parsed_kanji()));
        Ok(())
    }

    #[test]
    fn ignores_comment() {
        let line = vec![COMMENT_LINE, KANJI_LINE].join("".as_bytes());
        let res = next_kanji(&line);
        assert_eq!(res, Ok((NEWLINE, parsed_kanji())));
    }

    #[test]
    fn parses_lines() {
        let line = vec![KANJI_LINE, COMMENT_LINE, KANJI_LINE].join("".as_bytes());
        let res = lines(&line);
        assert_eq!(res, Ok((NEWLINE, vec![parsed_kanji(), parsed_kanji()])));
    }
}
