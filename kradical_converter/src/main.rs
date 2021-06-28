use clap::{AppSettings, Clap};
use kradical_parsing::krad::{parse_file, Decomposition, KradError};
use std::{fs::File, io::Write};

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

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

// Todo: Take files to parse and output location as arguments
fn krad_stuff() -> Result<(), KradError> {
    let mut decompositions = vec![];
    decompositions.extend(parse_kradfile()?);
    decompositions.extend(parse_kradfile2()?);
    File::create("./assets/outputs/krad_utf8.txt").and_then(|mut file| {
        for decomposition in decompositions {
            let radicals = decomposition.radicals.join(" ");
            let kanji = decomposition.kanji;
            let s = format!("{} : {}\n", kanji, &radicals);
            file.write(s.as_bytes())?;
        }
        Ok(())
    })?;
    Ok(())
}

fn parse_kradfile() -> Result<Vec<Decomposition>, KradError> {
    parse_file("./assets/edrdg_files/kradfile")
}

fn parse_kradfile2() -> Result<Vec<Decomposition>, KradError> {
    parse_file("./assets/edrdg_files/kradfile2")
}
