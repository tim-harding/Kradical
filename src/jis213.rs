mod jis_to_unicode;

use anyhow::Result;
pub use jis_to_unicode::jis_codepoint_to_unicode;
use nom::{
    bytes::complete::{take, take_while},
    combinator::map_opt,
    multi::many_till,
    IResult,
};

// pub fn decode(b: &[u8]) -> IResult<&[u8], String> {
// many_till(codepoint(b), eof)(b)
// }

// fn codepoint(b: &[u8]) -> IResult<&[u8], &'static str> {
//
// }

fn ascii(b: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(|byte| byte < 0x9Fu8)(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn parses_ascii() {
        const TEXT: &str = "#	Copyright 2001/2007 Michael Raine, 
            James Breen and the Electronic\n
            #   Dictionary Research & Development Group";
        let res = ascii(TEXT.as_bytes());
        assert_eq!(res, Ok(("".as_bytes(), TEXT.as_bytes())));
    }
}
