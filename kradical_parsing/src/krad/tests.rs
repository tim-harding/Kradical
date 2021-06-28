use super::*;
use crate::test_constants::*;

// JIS213
// "亜 : ｜ 一 口\n"
const KANJI_LINE: &[u8] = &[
    0xB0, 0xA1, 0x20, 0x3A, 0x20, 0xA1, 0xC3, 0x20, 0xB0, 0xEC, 0x20, 0xB8, 0xFD, 0x0A,
];

// First kanji EUC-JP, radicals JIS213
// "丂 : 一 勹\n"
const KANJI_LINE2: &[u8] = &[
    0x8F, 0xB0, 0xA1, 0x20, 0x3A, 0x20, 0xB0, 0xEC, 0x20, 0xD2, 0xB1, 0x0A,
];

// JIS213
// "｜ 一 口\n"
const RADICALS: &[u8] = &[0xA1, 0xC3, 0x20, 0xB0, 0xEC, 0x20, 0xB8, 0xFD, 0x0A];

fn parsed_kanji() -> Decomposition {
    Decomposition {
        kanji: "亜".to_string(),
        radicals: vec!["｜".to_string(), "一".to_string(), "口".to_string()],
    }
}

fn parsed_kanji_2() -> Decomposition {
    Decomposition {
        kanji: "丂".to_string(),
        radicals: vec!["一".to_string(), "勹".to_string()],
    }
}

#[test]
fn parses_radical() {
    let res = radical(RADICALS);
    assert_eq!(res, Ok((&RADICALS[2..], "｜".to_string())));
}

#[test]
fn parses_radicals() {
    let res = radicals(RADICALS);
    assert_eq!(res, Ok((NEWLINE, parsed_kanji().radicals)));
}

#[test]
fn parses_kanji() {
    let res = kanji_line(KANJI_LINE);
    assert_eq!(res, Ok((NEWLINE, parsed_kanji())));
}

#[test]
fn parses_kanji_2() {
    let res = kanji_line(KANJI_LINE2);
    assert_eq!(res, Ok((NEWLINE, parsed_kanji_2())));
}

#[test]
fn parses_line_as_kanji() {
    let res = next_kanji(KANJI_LINE);
    assert_eq!(res, Ok((NEWLINE, parsed_kanji())));
}

#[test]
fn ignores_comment() {
    let line = vec![COMMENT_LINE, KANJI_LINE].join(EMPTY);
    let res = next_kanji(&line);
    assert_eq!(res, Ok((NEWLINE, parsed_kanji())));
}

#[test]
fn parses_lines() {
    let line = vec![KANJI_LINE, COMMENT_LINE, KANJI_LINE].join(EMPTY);
    let res = lines(&line);
    assert_eq!(res, Ok((NEWLINE, vec![parsed_kanji(), parsed_kanji()])));
}

#[test]
fn works_on_actual_file() {
    let res = parse_file("../assets/edrdg_files/kradfile");
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().len(), 6_355);
}

#[test]
fn works_on_actual_file_2() {
    let res = parse_file("../assets/edrdg_files/kradfile2");
    assert_eq!(res.is_ok(), true);
    assert_eq!(res.unwrap().len(), 5_801);
}
