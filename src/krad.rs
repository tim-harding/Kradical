use nom::{IResult, bytes::complete::{tag, take, take_until}, combinator::eof, multi::many_till, sequence::{pair, separated_pair, terminated}};

const NEWLINE: &[u8] = &[0x0A];
const SEPARATOR: &[u8] = &[0x20, 0x3A, 0x20];
const POUND: &[u8] = &[0x23];
const SPACE: &[u8] = &[0x20];

pub fn lines(b: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    let (i, o) = many_till(line, eof)(b)?;
    let (lines, _end) = o;
    Ok((i, lines))
}

fn line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    let (i, o) = pair(take_until(NEWLINE), take(1u8))(b)?;
    let (line, _newline) = o;
    Ok((i, line))
}

struct KanjiParts<'a> {
    kanji: &'a[u8],
    parts: Vec<&'a[u8]>,
}

// fn kanji_line(b: &[u8]) -> IResult<&[u8], &[u8]> {
    // let (i, o) = separated_pair(take_until(SEPARATOR), tag(SEPARATOR), )
// }

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}