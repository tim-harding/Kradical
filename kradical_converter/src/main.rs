use clap::{AppSettings, Clap};
use kradical_parsing::krad::{self, Decomposition, KradError};
use std::{fs::File, io::Write};
use thiserror::Error;

#[derive(Clap, Clone, PartialEq, Eq, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(arg_enum)]
    input_format: InputFormat,

    #[clap(arg_enum)]
    output_format: OutputFormat,

    #[clap(short, long, required = true)]
    inputs: Vec<String>,

    #[clap(short, long)]
    output: String,
}

#[derive(PartialEq, Eq, Clone, Copy, Clap, Debug)]
enum InputFormat {
    Radk,
    Krad,
}

#[derive(PartialEq, Eq, Clone, Copy, Clap, Debug)]
enum OutputFormat {
    Unicode,
    Rust,
}

#[derive(Debug, Error)]
enum ConvertError {
    #[error("Error during krad parsing")]
    Krad(#[from] KradError),

    #[error("IO error")]
    Io(#[from] std::io::Error),
}

fn main() -> Result<(), ConvertError> {
    let opts = Opts::parse();

    let text = match opts.input_format {
        InputFormat::Radk => todo!(),
        InputFormat::Krad => parse_krad(&opts.inputs, opts.output_format)?,
    };

    File::create(opts.output).and_then(|mut file| file.write(text.as_bytes()))?;

    Ok(())
}

fn parse_krad(inputs: &[String], format: OutputFormat) -> Result<String, KradError> {
    let formatter = krad_formatter(format);
    let parsed: Result<Vec<_>, _> = inputs.iter().map(|input| krad::parse_file(input)).collect();
    let decompositions: Vec<String> = parsed?
        .iter()
        .flat_map(|file| file.into_iter().map(formatter))
        .collect();
    Ok(decompositions.join("\n"))
}

fn krad_formatter(format: OutputFormat) -> fn(&Decomposition) -> String {
    match format {
        OutputFormat::Unicode => krad_to_unicode,
        OutputFormat::Rust => todo!(),
    }
}

fn krad_to_unicode(decomposition: &Decomposition) -> String {
    let radicals = decomposition.radicals.join(" ");
    format!("{} : {}", decomposition.kanji, &radicals)
}
