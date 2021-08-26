use crate::opts::OutputFormat;
use kradical_parsing::krad::{self, Decomposition, KradError};

pub fn parse(inputs: &[String], format: OutputFormat) -> Result<String, KradError> {
    let parsed: Result<Vec<_>, _> = inputs.iter().map(krad::parse_file).collect();
    let parsed: Vec<_> = parsed?
        .into_iter()
        .flat_map(|file| file.into_iter())
        .collect();
    Ok(formatter(format)(&parsed))
}

fn formatter(format: OutputFormat) -> fn(&[Decomposition]) -> String {
    match format {
        OutputFormat::Unicode => to_unicode,
        OutputFormat::Rust => to_rust,
        OutputFormat::Json => to_json,
    }
}

fn to_json(decompositions: &[Decomposition]) -> String {
    let mut lines = vec![];
    lines.push("[".to_owned());
    for (i, decomposition) in decompositions.iter().enumerate() {
        lines.push("\t{".to_owned());
        lines.push(format!("\t\t\"kanji\": \"{}\",", decomposition.kanji));
        lines.push("\t\t\"radicals\": [".to_owned());
        for (i, radical) in decomposition.radicals.iter().enumerate() {
            if i == decomposition.radicals.len() - 1 {
                lines.push(format!("\t\t\t\"{}\"", radical));
            } else {
                lines.push(format!("\t\t\t\"{}\",", radical));
            }
        }
        lines.push("\t\t]".to_owned());
        if i == decompositions.len() - 1 {
            lines.push("\t}".to_owned());
        } else {
            lines.push("\t},".to_owned());
        }
    }
    lines.push("]".to_owned());
    lines.join("\n")
}

fn to_unicode(decompositions: &[Decomposition]) -> String {
    let lines: Vec<String> = decompositions
        .iter()
        .map(|decomposition| {
            let radicals = decomposition.radicals.join(" ");
            format!("{} : {}", decomposition.kanji, &radicals)
        })
        .collect();
    lines.join("\n")
}

fn to_rust(decompositions: &[Decomposition]) -> String {
    let mut lines = vec![
        "use super::Decomposition;".to_string(),
        "".to_string(),
        "/// The list of radical decompositions from the `kradfile`".to_string(),
        "pub const DECOMPOSITIONS: &[Decomposition] = &[".to_string(),
    ];
    for decomposition in decompositions {
        lines.push("\t Decomposition {".to_string());
        lines.push(format!("\t\tkanji: \'{}\',", decomposition.kanji));
        lines.push("\t\tradicals: &[".to_string());
        for radical in decomposition.radicals.iter() {
            lines.push(format!("\t\t\t\'{}\',", radical));
        }
        lines.push("\t\t],".to_string());
        lines.push("\t},".to_string());
    }
    lines.push("];".to_string());
    lines.join("\n")
}
