mod jis213;
#[cfg(test)]
mod kradfile2_hex;
#[cfg(test)]
mod kradfile_hex;

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
    value((), pair(char('#'), take_until("\n")))(b)
}

pub fn decode_jis(b: &[u8]) -> Result<&'static str> {
    println!("{:?}", b);
    match b.len() {
        2 | 3 => {
            let code = bytes_to_u32(b);
            jis213::decode(code).ok_or(KradError::Jis.into())
        }
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
    // use super::kradfile2_hex::KRADFILE2_HEX;
    use super::kradfile_hex::KRADFILE_HEX;
    use super::*;
    use anyhow::Result;

    // JIS213
    // "亜 : ｜ 一 口\n"
    const KANJI_LINE: &[u8] = &[
        0xB0, 0xA1, 0x20, 0x3A, 0x20, 0xA1, 0xC3, 0x20, 0xB0, 0xEC, 0x20, 0xB8, 0xFD, 0x0A,
    ];

    const KANJI_3B_AND_SEP: &[u8] = &[
        0x8F, 0xB0, 0xA1, 0x20, 0x3A, 0x20, 0xB0, 0xEC, 0x20, 0xD2, 0xB1, 0x0A,
    ];

    const KANJI_LINE2: &[u8] = &[
        0xEF, 0xBF, 0xBD, 0xEF, 0xBF, 0xBD, 0xEF, 0xBF, 0xBD, 0x20, 0x3A, 0x20, 0xEF, 0xBF, 0xBD,
        0xEF, 0xBF, 0xBD, 0x20, 0xD2, 0xB1, 0x0A,
    ];

    // JIS213
    // "｜ 一 口\n"
    const RADICALS: &[u8] = &[0xA1, 0xC3, 0x20, 0xB0, 0xEC, 0x20, 0xB8, 0xFD, 0x0A];

    const COMMENT_LINE: &[u8] = "# September 2007\n".as_bytes();
    const NEWLINE: &[u8] = "\n".as_bytes();

    fn parsed_kanji() -> KanjiParts<'static> {
        KanjiParts {
            kanji: "亜",
            radicals: vec!["｜", "一", "口"],
        }
    }

    #[test]
    fn is_comment() -> Result<()> {
        let (_i, o) = comment(COMMENT_LINE)?;
        assert_eq!(o, ());
        Ok(())
    }

    #[test]
    fn is_comment_short() -> Result<()> {
        let (_i, o) = comment("#\n".as_bytes())?;
        assert_eq!(o, ());
        Ok(())
    }

    #[test]
    fn multiple_comment_lines() -> Result<()> {
        let line = vec![COMMENT_LINE, COMMENT_LINE].join("".as_bytes());
        let (_i, o) = comments(&line)?;
        assert_eq!(o, ());
        Ok(())
    }

    #[test]
    fn parses_radical() -> Result<()> {
        let res = radical(RADICALS)?;
        assert_eq!(res, (&RADICALS[2..], "｜"));
        Ok(())
    }

    #[test]
    fn parses_radicals() -> Result<()> {
        let res = radicals(RADICALS)?;
        assert_eq!(res, (NEWLINE, parsed_kanji().radicals));
        Ok(())
    }

    #[test]
    fn parses_kanji() -> Result<()> {
        let res = kanji_line(KANJI_LINE)?;
        assert_eq!(res, (NEWLINE, parsed_kanji()));
        Ok(())
    }

    #[test]
    fn parses_kanji_3b() -> Result<()> {
        let (_i, o) = kanji(KANJI_3B_AND_SEP)?;
        assert_eq!(o, parsed_kanji());
        Ok(())
    }

    #[test]
    fn parses_line_as_kanji() -> Result<()> {
        let res = next_kanji(KANJI_LINE)?;
        assert_eq!(res, (NEWLINE, parsed_kanji()));
        Ok(())
    }

    #[test]
    fn ignores_comment() -> Result<()> {
        let line = vec![COMMENT_LINE, KANJI_LINE].join("".as_bytes());
        let (_i, o) = next_kanji(&line)?;
        assert_eq!(o, parsed_kanji());
        Ok(())
    }

    #[test]
    fn parses_lines() -> Result<()> {
        let line = vec![KANJI_LINE, COMMENT_LINE, KANJI_LINE].join("".as_bytes());
        let (_i, o) = lines(&line)?;
        assert_eq!(o, vec![parsed_kanji(), parsed_kanji()]);
        Ok(())
    }

    #[test]
    fn works_on_actual_file() -> Result<()> {
        let (i, o) = lines(KRADFILE_HEX)?;
        assert_eq!(i, NEWLINE);
        Ok(())
    }

    /*
    #[test]
    fn works_on_actual_file_2() -> Result<()> {
        let (i, o) = lines(KRADFILE2_HEX)?;
        assert_eq!(i, NEWLINE);
        Ok(())
    }
    */
}
