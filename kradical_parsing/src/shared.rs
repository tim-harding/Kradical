use encoding::{codec::japanese::EUCJPEncoding, DecoderTrap, Encoding};
use kradical_jis::jis213_to_utf8;
use nom::{
    bytes::complete::take_until,
    character::complete::{char, multispace0},
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, pair},
    IResult,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedError {
    /// Invalid JIS X 0213 codepoint
    #[error("Invalid JIS X 0213 codepoint")]
    Jis,

    /// Invalid EUC-JP codepoint
    #[error("Invalid EUC-JP codepoint")]
    EucJp,

    /// Incorrect number of bytes for JIS or EUC-JP
    #[error("Incorrect number of bytes for JIS or EUC-JP")]
    Unknown,
}

pub fn comments(b: &[u8]) -> IResult<&[u8], ()> {
    value(
        (),
        delimited(
            multispace0,
            separated_list0(char('\n'), comment),
            multispace0,
        ),
    )(b)
}

fn comment(b: &[u8]) -> IResult<&[u8], ()> {
    value((), pair(char('#'), take_until("\n")))(b)
}

// Sources for Unicode radical glyphs:
// https://unicode-table.com/en/blocks/kangxi-radicals/
// https://unicode-table.com/en/blocks/cjk-radicals-supplement/
// https://unicode-table.com/en/blocks/cjk-unified-ideographs/
// https://shapecatcher.com/

// Notes on image remappings
//
// Kept the left part
// Better alternative: ⺅
// 化 -> http://nihongo.monash.edu/gif212/js01.png
//
// Kept the top part
// https://www.wanikani.com/radicals/hat
// Possible alternatives: ^ へ ヘ ㅅ 𠆢
// 个 -> http://nihongo.monash.edu/gif212/js02.png
//
// Kept the top part
// https://www.wanikani.com/radicals/gun
// Possible alternatives: ⟝ 𠂉
// 乞 -> http://nihongo.monash.edu/gif212/js10.png
//
// Kept the horns
// https://www.wanikani.com/radicals/horns
// Better alternative: 丷
// 并 -> http://nihongo.monash.edu/gif212/js07.png
//
// Kept the left part
// https://www.wanikani.com/radicals/building
// Better alternative: ⻖
// 阡 -> http://nihongo.monash.edu/gif212/kozatoL.png
//
// Kept the top part
// https://www.wanikani.com/radicals/flowers
// Better alternative: 艹
// 艾 -> http://nihongo.monash.edu/gif212/js03.png
//
// Kept the right part
// https://www.wanikani.com/radicals/building
// Better alternative: ⻏
// 邦 -> http://nihongo.monash.edu/gif212/kozatoR.png
//
// Kept the horns at the top
// https://www.wanikani.com/radicals/triceratops
// Better alternative: ⺌
// 尚 -> http://nihongo.monash.edu/gif212/js04.png
//
// Kept the swoosh and above
// https://www.wanikani.com/radicals/coffin
// Better alternative: 耂
// 老 -> http://nihongo.monash.edu/gif212/js05.png

// These are different characters:
// ⻖ left  (2ED6)
// ⻏ right (2ECF)

// Todo: These should ONLY be applied to radicals, _not_ kanji.
pub fn remap_radical(code: u32) -> Option<&'static str> {
    // Remappings taken from kradfile lines 45-65
    match code {
        // 化 -> ⺅
        0xB2BD => Some("\u{2E85}"),

        // # D0 A4  2F09
        // 个 -> ⼉
        // Ignoring this one because it makes zero sense.
        // Maybe the authors had a typo.
        // 0xD0A4 => Some("\u{2F09}"),
        // This is the replacement used by Jisho.
        // 个 -> 𠆢
        0xD0A4 => Some("\u{201A2}"),

        // # D6 F5  none available - upside-down A5 CF
        // D6F5 -> 并
        // A5CF -> ハ
        // 0xD6F5 => Some("\u{30CF}"),
        // The authors suggest a vertically-flipped ハ
        // like the Wanikani horns radical
        // https://www.wanikani.com/radicals/horns
        // I found an alternate glyph that isn't
        // semantically a Japanese radical
        // (it's a kwukyel ideograph)
        // but it looks correct.
        // 并 -> 丷
        0xD6F5 => Some("\u{4E37}"),

        // 刈 -> ⺉
        0xB4A2 => Some("\u{2E89}"),

        // 込 -> ⻌
        0xB9FE => Some("\u{2ECC}"),

        // 尚 -> ⺌
        0xBEB0 => Some("\u{2E8C}"),

        // 忙 -> ⺖
        0xCBBB => Some("\u{2E96}"),

        // The suggested replacement is not correct.
        // 扎 -> ⺗
        // 0xD9A9 => Some("\u{2E97}"),
        // This is what appears on the WWWJDIC server
        // 扎 -> 扌
        0xD9A9 => Some("\u{624C}"),

        // 汁 -> ⺡
        0xBDC1 => Some("\u{2EA1}"),

        // 犯 -> ⺨
        0xC8C8 => Some("\u{2EA8}"),

        // 艾 -> ⺾
        0xE7E8 => Some("\u{2EBE}"),

        // 邦 -> ⻏
        0xCBAE => Some("\u{2ECF}"),

        // 阡 -> ⻙
        // 0xEFF4 => Some("\u{2ED9}"),
        // The above must have been another
        // mistake because there's a way better choice.
        // 阡 -> ⻖
        0xEFF4 => Some("\u{2ED6}"),

        // 老 -> ⺹
        0xCFB7 => Some("\u{2EB9}"),

        // 杰 -> ⺣
        0xDBBF => Some("\u{2EA3}"),

        // 礼 -> ⺭
        0xCEE9 => Some("\u{2EAD}"),

        // 疔 -> ⽧
        0xE1CB => Some("\u{2F67}"),

        // 禹 -> ⽱
        0xE3BB => Some("\u{2F71}"),

        // 初 -> ⻂
        0xBDE9 => Some("\u{2EC2}"),

        // 買 -> ⺲
        0xC7E3 => Some("\u{2EB2}"),

        // 滴 -> 啇
        0xC5A9 => Some("\u{5547}"),

        // Adding another of my own not from the
        // kradfile suggestions. This is the replacement
        // used by Jisho.
        // 乞 -> 𠂉
        0xB8F0 => Some("\u{20089}"),

        _ => None,
    }
}

pub fn decode_jis_radical(b: &[u8]) -> Result<String, SharedError> {
    match b.len() {
        2 => {
            let code = bytes_to_u32(b);
            remap_radical(code)
                .or_else(|| jis213_to_utf8(code))
                .map(|unicode| unicode.to_string())
                .ok_or(SharedError::Jis)
        }
        3 => EUCJPEncoding
            .decode(b, DecoderTrap::Strict)
            .map_err(|_| SharedError::EucJp),
        _ => Err(SharedError::Unknown),
    }
}

pub fn decode_jis_kanji(b: &[u8]) -> Result<String, SharedError> {
    match b.len() {
        2 => {
            let code = bytes_to_u32(b);
            jis213_to_utf8(code)
                .map(|unicode| unicode.to_string())
                .ok_or(SharedError::Jis)
        }
        3 => EUCJPEncoding
            .decode(b, DecoderTrap::Strict)
            .map_err(|_| SharedError::EucJp),
        _ => Err(SharedError::Unknown),
    }
}

fn bytes_to_u32(b: &[u8]) -> u32 {
    let mut out = 0u32;
    for (i, byte) in b.iter().rev().enumerate() {
        let byte = *byte as u32;
        out += byte << (8 * i);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_constants::*;

    #[test]
    fn is_comment() {
        let res = comment(COMMENT_LINE);
        assert_eq!(res, Ok((NEWLINE, ())));
    }

    #[test]
    fn is_comment_short() {
        let res = comment("#\n".as_bytes());
        assert_eq!(res, Ok((NEWLINE, ())));
    }

    #[test]
    fn multiple_comment_lines() {
        let line = vec![COMMENT_LINE, COMMENT_LINE].join("".as_bytes());
        let res = comments(&line);
        assert_eq!(res, Ok((EMPTY, ())));
    }
}
