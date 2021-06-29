use crate::opts::OutputFormat;
use kradical_parsing::radk::{self, Expansion, RadkError};

pub fn parse(inputs: &[String], format: OutputFormat) -> Result<String, RadkError> {
    let parsed: Result<Vec<_>, _> = inputs.iter().map(|input| radk::parse_file(input)).collect();
    let parsed: Vec<_> = parsed?
        .into_iter()
        .flat_map(|file| file.into_iter())
        .collect();
    Ok(formatter(format)(&parsed))
}

fn formatter(format: OutputFormat) -> fn(&[Expansion]) -> String {
    match format {
        OutputFormat::Unicode => to_unicode,
        OutputFormat::Rust => to_rust,
    }
}

fn to_unicode(expansions: &[Expansion]) -> String {
    todo!()
}

fn to_rust(expansions: &[Expansion]) -> String {
    todo!()
}
