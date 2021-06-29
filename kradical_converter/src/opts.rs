use clap::{AppSettings, Clap};

#[derive(Clap, Clone, PartialEq, Eq, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(arg_enum)]
    pub input_format: InputFormat,

    #[clap(arg_enum)]
    pub output_format: OutputFormat,

    #[clap(short, long, required = true)]
    pub inputs: Vec<String>,

    #[clap(short, long)]
    pub output: String,
}

#[derive(PartialEq, Eq, Clone, Copy, Clap, Debug)]
pub enum InputFormat {
    Radk,
    Krad,
}

#[derive(PartialEq, Eq, Clone, Copy, Clap, Debug)]
pub enum OutputFormat {
    Unicode,
    Rust,
}
