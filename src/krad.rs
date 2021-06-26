use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    combinator::{eof, map, value},
    multi::many_till,
    sequence::{pair, separated_pair},
    IResult,
};

// Note: requires newline before eof

const NEWLINE: &[u8] = &[0x0A];
const SEPARATOR: &[u8] = &[0x20, 0x3A, 0x20];
const POUND: &[u8] = &[0x23];
const SPACE: &[u8] = &[0x20];

// Todo: Shouldn't need to clone this
#[derive(Debug, Clone)]
pub struct KanjiParts<'a> {
    kanji: &'a [u8],
    radicals: Vec<&'a [u8]>,
}

pub fn lines(b: &[u8]) -> IResult<&[u8], Vec<Option<KanjiParts>>> {
    let (i, o) = many_till(line, eof)(b)?;
    let (lines, _end) = o;
    Ok((i, lines))
}

fn line(b: &[u8]) -> IResult<&[u8], Option<KanjiParts>> {
    alt((value(None, comment), map(kanji_line, |k| Some(k))))(b)
}

fn kanji_line(b: &[u8]) -> IResult<&[u8], KanjiParts> {
    let (i, o) = separated_pair(take_until(SEPARATOR), tag(SEPARATOR), radicals)(b)?;
    let (kanji, radicals) = o;
    let parts = KanjiParts { kanji, radicals };
    Ok((i, parts))
}

fn radicals(b: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    let (i, o) = pair(many_till(radical, tag(NEWLINE)), take(1u8))(b)?;
    let ((radicals, _), _newline) = o;
    Ok((i, radicals))
}

fn radical(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, o) = pair(take_until(SPACE), take(1u8))(b)?;
    let (radical, _space) = o;
    Ok((i, radical))
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    let (i, _o) = pair(tag(POUND), end_of_line)(b)?;
    Ok((i, ()))
}

fn end_of_line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, o) = pair(take_until(NEWLINE), take(1u8))(b)?;
    let (line, _newline) = o;
    Ok((i, line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn is_comment() -> Result<()> {
        let (_i, o) = comment("# September 2007\n".as_bytes())?;
        assert_eq!(o, ());
        Ok(())
    }
}
