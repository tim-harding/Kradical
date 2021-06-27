extern crate kanji_api;

use anyhow::Result;
use kanji_api::krad::{lines, KanjiParts};
use std::{fs, io::Write};
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("Error while parsing kradfile")]
    Parse,
    #[error("Error while reading kradfile")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<()> {
    let mut kanji = vec![];
    append_file("./dictionary-files/downloads/kradfile", &mut kanji)?;
    append_file("./dictionary-files/downloads/kradfile2", &mut kanji)?;
    std::fs::File::create("./dictionary-files/outputs/kradfile_unicode.txt").and_then(
        |mut file| {
            for kanji in kanji {
                let s = format!("{} : {}\n", kanji.kanji, kanji.radicals.join(" "));
                file.write(s.as_bytes())?;
            }
            Ok(())
        },
    )?;
    Ok(())
}

fn append_file(filename: &str, kanji: &mut Vec<KanjiParts>) -> Result<()> {
    fs::read(filename)
        .map_err(|err| AppError::Io(err))
        .and_then(|text| {
            lines(&text)
                .map(|(_, mut results)| {
                    kanji.append(&mut results);
                })
                .map_err(|_err| AppError::Parse)
        })
        .map_err(|err| err.into())
}
