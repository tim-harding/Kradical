use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use crate::opts::OutputFormat;
use kradical_parsing::radk::{self, Expansion, Radical, RadkError};

pub fn parse(inputs: &[String], format: OutputFormat) -> Result<String, RadkError> {
    let parsed: Result<Vec<_>, _> = inputs.iter().map(|input| radk::parse_file(input)).collect();
    let parsed: Vec<_> = parsed?
        .into_iter()
        .flat_map(|file| file.into_iter())
        .collect();
    let parsed = consolidate(parsed);
    Ok(formatter(format)(&parsed))
}

fn formatter(format: OutputFormat) -> fn(&[Expansion]) -> String {
    match format {
        OutputFormat::Unicode => to_unicode,
        OutputFormat::Rust => to_rust,
    }
}

fn to_unicode(expansions: &[Expansion]) -> String {
    let lines: Vec<_> = expansions
        .iter()
        .map(|expansion| {
            let kanji = expansion.kanji.join(" ");
            let radical = &expansion.radical;
            let alt = match &radical.alternate {
                radk::Alternate::Image(image) => format!(" alt_image({})", image),
                radk::Alternate::Glyph(glyph) => format!(" alt_glyph({})", glyph),
                radk::Alternate::None => "".to_string(),
            };
            format!("{} {}{} : {}", radical.glyph, radical.strokes, alt, kanji)
        })
        .collect();
    lines.join("\n")
}

fn to_rust(expansions: &[Expansion]) -> String {
    let mut lines = vec![
        "use super::{Expansion, Alternate};".to_string(),
        "".to_string(),
        "pub const EXPANSIONS: &[Expansion] = &[".to_string(),
    ];
    for expansion in expansions {
        lines.push("\tExpansion {".to_string());
        let radical = &expansion.radical;
        let alt = match &radical.alternate {
            radk::Alternate::Image(image) => format!("Image(\"{}\")", image),
            radk::Alternate::Glyph(glyph) => format!("Glyph(\"{}\")", glyph),
            radk::Alternate::None => "None".to_string(),
        };
        lines.push(format!("\t\tradical: \"{}\",", radical.glyph));
        lines.push(format!("\t\tstrokes: {},", radical.strokes));
        lines.push(format!("\t\talternate: Alternate::{},", alt));
        lines.push("\t\tkanji: &[".to_string());
        for glyph in &expansion.kanji {
            lines.push(format!("\t\t\t\"{}\",", glyph));
        }
        lines.push("\t\t],".to_string());
        lines.push("\t},".to_string());
    }
    lines.push("];".to_string());
    lines.join("\n")
}

fn consolidate(expansions: Vec<Expansion>) -> Vec<Expansion> {
    let mut consolidation: HashMap<Radical, HashSet<String>> = HashMap::new();
    for expansion in expansions.into_iter() {
        match consolidation.entry(expansion.radical) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().extend(expansion.kanji);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(HashSet::from_iter(expansion.kanji.into_iter()));
            }
        }
    }
    let mut consolidation: Vec<_> = consolidation
        .into_iter()
        .map(|(radical, kanji)| {
            let kanji: Vec<_> = kanji.into_iter().collect();
            Expansion { radical, kanji }
        })
        .collect();

    consolidation.sort_unstable_by(|l, r| l.radical.strokes.cmp(&r.radical.strokes));

    consolidation
}
