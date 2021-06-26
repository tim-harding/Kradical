use nom::{IResult, branch::alt, bytes::{complete::{tag, take, take_till, take_until}, streaming::is_not}, character::{complete::{char, one_of}, is_newline}, combinator::{complete, eof, map, value}, multi::{many_till, separated_list1}, sequence::{pair, separated_pair}};

// Note: requires newline before eof
// Otherwise silently ignores the last line

const SEPARATOR: &[u8] = " : ".as_bytes();

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
    separated_list1(char(' '), radical)(b)
}

fn radical(b: &[u8]) -> IResult<&[u8], &[u8]> {
    is_not(" \n")(b)
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    let (i, _o) = pair(char('#'), end_of_line)(b)?;
    Ok((i, ()))
}

fn end_of_line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((is_not("\n"), eof))(b)
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

    #[test]
    fn is_not_comment() {
        let res = comment("��� : �� �� ��\n".as_bytes());
        match res {
            // Todo: I'm sure there's a better way of writing this
            Ok(_) => assert_eq!(true, false),
            Err(_) => {},
        }
    }

    #[test]
    fn takes_entire_line() -> Result<()> {
        let res = end_of_line("# September 2007\n".as_bytes())?;
        assert_eq!(res, ("\n".as_bytes(), "# September 2007".as_bytes()));
        Ok(())
    }

    #[test]
    fn parses_radical() -> Result<()> {
        let res = radical("�� �� ��\n".as_bytes())?;
        assert_eq!(res, (" �� ��\n".as_bytes(), "��".as_bytes()));
        Ok(())
    }
}
