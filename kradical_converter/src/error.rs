use kradical_parsing::{krad::KradError, radk::RadkError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConvertError {
    #[error("Error during krad parsing")]
    Krad(#[from] KradError),

    #[error("Error during radk parsing")]
    Radk(#[from] RadkError),

    #[error("IO error")]
    Io(#[from] std::io::Error),
}
