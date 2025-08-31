use clap::{Parser, ValueEnum};

#[derive(Parser, Clone, PartialEq, Eq, Debug)]
#[command(about = "Convert EDRDG radical decomposition files to various formats")]
pub struct Opts {
    /// Input format (krad or radk)
    pub input_format: InputFormat,

    /// Output format (unicode, rust, or json)
    pub output_format: OutputFormat,

    /// Input files to process
    #[arg(short, long, required = true)]
    pub inputs: Vec<String>,

    /// Output file path
    #[arg(short, long)]
    pub output: String,
}

#[derive(PartialEq, Eq, Clone, Copy, ValueEnum, Debug)]
pub enum InputFormat {
    Radk,
    Krad,
}

#[derive(PartialEq, Eq, Clone, Copy, ValueEnum, Debug)]
pub enum OutputFormat {
    Unicode,
    Rust,
    Json,
}
