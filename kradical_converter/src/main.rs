use clap::Clap;
use error::ConvertError;
use std::{fs::OpenOptions, io::Write};

use crate::opts::{InputFormat, Opts};

mod error;
mod krad;
mod opts;
mod radk;

fn main() -> Result<(), ConvertError> {
    let opts = Opts::parse();
    let text = match opts.input_format {
        InputFormat::Radk => radk::parse(&opts.inputs, opts.output_format)?,
        InputFormat::Krad => krad::parse(&opts.inputs, opts.output_format)?,
    };
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(opts.output)
        .and_then(|mut file| file.write_all(text.as_bytes()))?;
    Ok(())
}
