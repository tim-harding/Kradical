use super::{Alternate, Membership, Radical};
use crate::test_constants::{COMMENT_LINE, EMPTY};

fn parsed_radical_simple() -> Radical {
    Radical {
        glyph: "一".to_string(),
        strokes: 1,
        alternate: Alternate::None,
    }
}

#[test]
fn strokes() {
    let res = super::strokes(b"12");
    assert_eq!(res, Ok((EMPTY, 12)));
}

// JIS X 0213
// $ 一 1
const IDENT_LINE_SIMPLE: &[u8] = &[0x24, 0x20, 0xB0, 0xEC, 0x20, 0x31];

#[test]
fn radical() {
    let radical_and_space = &IDENT_LINE_SIMPLE[2..];
    let res = super::radical(radical_and_space);
    assert_eq!(res, Ok((&IDENT_LINE_SIMPLE[5..], "一".to_string())))
}

#[test]
fn simple_ident_line() {
    let res = super::ident_line(IDENT_LINE_SIMPLE);
    assert_eq!(res, Ok((EMPTY, parsed_radical_simple())));
}

#[test]
fn hex() {
    let res = super::hex(b"6134");
    assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
}

#[test]
fn image() {
    let res = super::image(b"js02");
    assert_eq!(res, Ok((EMPTY, Alternate::Image("js02".to_string()))));
}

#[test]
fn alt_is_hex() {
    let res = super::alternate(b"6134");
    assert_eq!(res, Ok((EMPTY, Alternate::Glyph("辶".to_string()))));
}

#[test]
fn alt_is_image() {
    let res = super::alternate(b"js02");
    assert_eq!(res, Ok((EMPTY, Alternate::Image("js02".to_string()))));
}

#[test]
fn alt_is_none() {
    let res = super::alternate(EMPTY);
    assert_eq!(res, Ok((EMPTY, Alternate::None)));
}

#[test]
fn image_ident_line() {
    // $ Ф 2 js02
    const IDENT_LINE_FULL_IMG: &[u8] = &[
        0x24, 0x20, 0xD0, 0xA4, 0x20, 0x32, 0x20, 0x6A, 0x73, 0x30, 0x32,
    ];
    let res = super::ident_line(IDENT_LINE_FULL_IMG);
    assert_eq!(
        res,
        Ok((
            EMPTY,
            Radical {
                glyph: "个".to_string(),
                strokes: 2,
                alternate: Alternate::Image("js02".to_string()),
            }
        ))
    )
}

#[test]
fn glyph_ident_line() {
    // $ ˻ 3 3D38
    const IDENT_LINE_FULL_JIS: &[u8] = &[
        0x24, 0x20, 0xCB, 0xBB, 0x20, 0x33, 0x20, 0x33, 0x44, 0x33, 0x38,
    ];
    let res = super::ident_line(IDENT_LINE_FULL_JIS);
    assert_eq!(
        res,
        Ok((
            EMPTY,
            Radical {
                glyph: "忙".to_string(),
                strokes: 3,
                alternate: Alternate::Glyph("\u{5FC4}".to_string()),
            }
        ))
    )
}

#[test]
fn kanji_line() {
    // Radkfile line 548
    const KANJI_LINE: &[u8] = &[
        0xB0, 0xD4, 0xB1, 0xD9, 0xB2, 0xB1, 0xB2, 0xF7, 0xB2, 0xF8, 0xB2, 0xF9, 0xB2, 0xFA, 0xB2,
        0xFB, 0xB3, 0xB4, 0xB3, 0xE6, 0xB4, 0xB7, 0xB4, 0xB8, 0xB6, 0xB1, 0xB8, 0xE7, 0xB9, 0xB1,
        0xB9, 0xB2, 0xB9, 0xFB, 0xBA, 0xA8, 0xBB, 0xB4, 0xBE, 0xF0, 0xBF, 0xB5, 0xC0, 0xAD, 0xC0,
        0xCB, 0xC1, 0xFE, 0xC2, 0xC6, 0xC4, 0xF0, 0xC5, 0xE9, 0xC6, 0xB4, 0xC6, 0xD7, 0xC7, 0xBA,
        0xC9, 0xDD, 0xCA, 0xB0, 0xCB, 0xBB, 0xCB, 0xFD, 0xCC, 0xFB, 0xCE, 0xE7,
    ];
    let expected: Vec<String> = [
        "惟", "悦", "憶", "快", "怪", "悔", "恢", "懐", "慨", "恰", "慣", "憾", "怯", "悟", "恒",
        "慌", "惚", "恨", "惨", "情", "慎", "性", "惜", "憎", "惰", "悌", "悼", "憧", "惇", "悩",
        "怖", "憤", "忙", "慢", "愉", "怜",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();
    let res = super::kanji_lines(KANJI_LINE);
    assert_eq!(res, Ok((EMPTY, expected)));
}

#[test]
fn kanji_multiline() {
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
    let expected: Vec<String> = [
        "猿", "荻", "獲", "狂", "狭", "狗", "狐", "獄", "狛", "墾", "懇", "獅", "狩", "狙", "狸",
        "猪", "独", "猫", "狽", "犯", "猛", "猶", "猟", "狼", "潴", "犹", "犲", "狃", "狆", "狄",
        "狎", "狒", "狢", "狠", "狡", "狹", "狷", "猗", "猊", "猜", "猖", "猝", "猴", "猯", "猩",
        "猥", "猾", "獏", "獗", "獪", "獨", "獰", "獵", "獺", "蕕", "誑", "逖",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();
    let res = super::kanji_lines(KANJI_MULTILINE);
    assert_eq!(res, Ok((EMPTY, expected)));
}

fn inclusion_expected() -> Membership {
    let inc: Vec<String> = [
        "郁", "廓", "郭", "郷", "響", "饗", "郡", "祁", "郊", "蔀", "邪", "邸", "鄭", "都", "那",
        "部", "邦", "爺", "耶", "郵", "廊", "榔", "郎", "嚮", "娜", "揶", "擲", "梛", "椰", "槨",
        "瑯", "螂", "躑", "邨", "邯", "邱", "邵", "郢", "郤", "扈", "郛", "鄂", "鄒", "鄙", "鄲",
        "鄰",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();
    Membership {
        radical: Radical {
            glyph: "邦".to_string(),
            strokes: 3,
            alternate: Alternate::Image("kozatoR".to_string()),
        },
        kanji: inc,
    }
}

// 588 - 590
const FULL_KANJI: &[u8] = &[
    0x24, 0x20, 0xCB, 0xAE, 0x20, 0x33, 0x20, 0x6B, 0x6F, 0x7A, 0x61, 0x74, 0x6F, 0x52, 0x0A, 0xB0,
    0xEA, 0xB3, 0xC7, 0xB3, 0xD4, 0xB6, 0xBF, 0xB6, 0xC1, 0xB6, 0xC2, 0xB7, 0xB4, 0xB7, 0xB7, 0xB9,
    0xD9, 0xBC, 0xC3, 0xBC, 0xD9, 0xC5, 0xA1, 0xC5, 0xA2, 0xC5, 0xD4, 0xC6, 0xE1, 0xC9, 0xF4, 0xCB,
    0xAE, 0xCC, 0xEC, 0xCC, 0xED, 0xCD, 0xB9, 0xCF, 0xAD, 0xCF, 0xB1, 0xCF, 0xBA, 0xD3, 0xEC, 0xD5,
    0xB1, 0xD9, 0xE8, 0xDA, 0xB3, 0xDB, 0xEB, 0xDC, 0xBF, 0xDC, 0xDA, 0xE0, 0xE7, 0xEA, 0xA7, 0xED,
    0xB6, 0xEE, 0xB7, 0xEE, 0xB8, 0xEE, 0xB9, 0x0A, 0xEE, 0xBA, 0xEE, 0xBB, 0xEE, 0xBC, 0xEE, 0xBD,
    0xEE, 0xBE, 0xEE, 0xBF, 0xEE, 0xC0, 0xEE, 0xC1, 0xEE, 0xC2, 0xEE, 0xC3, 0x0A,
];

#[test]
fn inclusion() {
    let res = super::kanji(FULL_KANJI);
    assert_eq!(res, Ok((EMPTY, inclusion_expected())));
}

#[test]
fn inclusion_with_comment() {
    let lines = [COMMENT_LINE, FULL_KANJI].join("".as_bytes());
    let res = super::kanji(&lines);
    assert_eq!(res, Ok((EMPTY, inclusion_expected())));
}

#[test]
fn works_on_actual_file() {
    let res = super::parse_file("../assets/edrdg_files/radkfile");
    if let Ok(inclusions) = res {
        assert_eq!(inclusions.len(), 253);
    } else {
        println!("{:?}", res);
        assert_eq!(true, res.is_ok());
    }
}

#[test]
fn works_on_actual_file_2() {
    let res = super::parse_file("../assets/edrdg_files/radkfile2");
    if let Ok(inclusions) = res {
        assert_eq!(inclusions.len(), 253);
    } else {
        println!("{:?}", res);
        assert_eq!(true, res.is_ok());
    }
}
