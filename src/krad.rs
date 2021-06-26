use nom::{
    bytes::{
        complete::{tag, take_until},
        streaming::is_not,
    },
    character::complete::char,
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::{pair, separated_pair},
    IResult,
};

// Note: requires newline before eof

const SEPARATOR: &[u8] = " : ".as_bytes();

// Todo: Shouldn't need to clone this
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KanjiParts<'a> {
    kanji: &'a [u8],
    radicals: Vec<&'a [u8]>,
}

pub fn lines(b: &[u8]) -> IResult<&[u8], Vec<KanjiParts>> {
    separated_list1(char('\n'), next_kanji)(b)
}

fn next_kanji(b: &[u8]) -> IResult<&[u8], KanjiParts> {
    let (i, o) = separated_pair(comments, opt(char('\n')), kanji_line)(b)?;
    let (_comments, kanji) = o;
    Ok((i, kanji))
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

fn comments(b: &[u8]) -> IResult<&[u8], ()> {
    let (i, _o) = separated_list0(char('\n'), comment)(b)?;
    Ok((i, ()))
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    let (i, _o) = pair(char('#'), is_not("\n"))(b)?;
    Ok((i, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    const KANJI_LINE: &[u8] = "��� : �� �� �� �� ��\n".as_bytes();
    const COMMENT_LINE: &[u8] = "# September 2007\n".as_bytes();
    const NEWLINE: &[u8] = "\n".as_bytes();

    fn parsed_kanji() -> KanjiParts<'static> {
        KanjiParts {
            kanji: "���".as_bytes(),
            radicals: vec![
                "��".as_bytes(),
                "��".as_bytes(),
                "��".as_bytes(),
                "��".as_bytes(),
                "��".as_bytes(),
            ],
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
        assert_eq!(res, (" �� ��\n".as_bytes(), "��".as_bytes()));
        Ok(())
    }

    #[test]
    fn parses_radicals() -> Result<()> {
        let res = radicals("�� �� ��\n".as_bytes())?;
        assert_eq!(
            res,
            (
                NEWLINE,
                vec!["��".as_bytes(), "��".as_bytes(), "��".as_bytes()]
            )
        );
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
