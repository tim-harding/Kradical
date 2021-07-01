use std::collections::{HashMap, HashSet};

use crate::opts::OutputFormat;
use kradical_parsing::radk::{self, Membership, Radical, RadkError};

pub fn parse(inputs: &[String], format: OutputFormat) -> Result<String, RadkError> {
    let parsed: Result<Vec<_>, _> = inputs.iter().map(radk::parse_file).collect();
    let parsed: Vec<_> = parsed?
        .into_iter()
        .flat_map(|file| file.into_iter())
        .collect();
    let parsed = consolidate(parsed);
    Ok(formatter(format)(&parsed))
}

fn formatter(format: OutputFormat) -> fn(&[Membership]) -> String {
    match format {
        OutputFormat::Unicode => to_unicode,
        OutputFormat::Rust => to_rust,
    }
}

fn to_unicode(expansions: &[Membership]) -> String {
    let lines: Vec<_> = expansions
        .iter()
        .map(|expansion| {
            let kanji = expansion.kanji.join(" ");
            let radical = &expansion.radical;
            format!("{} {} : {}", radical.glyph, radical.strokes, kanji)
        })
        .collect();
    lines.join("\n")
}

fn to_rust(expansions: &[Membership]) -> String {
    let mut lines = vec![
        "use super::Membership;".to_string(),
        "".to_string(),
        "/// For each radical, a list of which kanji contain it from the `radkfile`".to_string(),
        "pub const MEMBERSHIPS: &[Membership] = &[".to_string(),
    ];
    for expansion in expansions {
        lines.push("\tMembership {".to_string());
        let radical = &expansion.radical;
        lines.push(format!("\t\tradical: \"{}\",", radical.glyph));
        lines.push(format!("\t\tstrokes: {},", radical.strokes));
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

fn consolidate(expansions: Vec<Membership>) -> Vec<Membership> {
    let mut consolidation: HashMap<Radical, HashSet<String>> = HashMap::new();
    for expansion in expansions.into_iter() {
        match consolidation.entry(expansion.radical) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().extend(expansion.kanji);
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(expansion.kanji.into_iter().collect());
            }
        }
    }

    let mut consolidation: Vec<_> = consolidation
        .into_iter()
        .map(|(radical, kanji)| {
            let kanji: Vec<_> = kanji.into_iter().collect();
            Membership { radical, kanji }
        })
        .collect();

    for membership in consolidation.iter_mut() {
        membership.kanji.sort();
    }

    consolidation.sort_by(|l, r| {
        if l.radical.strokes != r.radical.strokes {
            l.radical.strokes.cmp(&r.radical.strokes)
        } else if l.kanji.len() != r.kanji.len() {
            l.kanji.len().cmp(&r.kanji.len()).reverse()
        } else {
            l.radical.glyph.cmp(&r.radical.glyph)
        }
    });

    consolidation
}
