use std::io::{Error, ErrorKind};
use clap::ValueEnum;

// ========== Units (Metrical or Imperial) ==========
#[derive(Debug, Clone, ValueEnum)]
pub enum Units {
    Metric,
    Imperial,
}

impl Units {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "metric" => Ok(Self::Metric),
            "imperial" => Ok(Self::Imperial),
            _ => Err(Error::new(ErrorKind::InvalidInput, format!("Invalid units: {}", s))),
        }
    }
}