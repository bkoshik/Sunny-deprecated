use std::io::{Error, ErrorKind};
use clap::Parser;
use serde::Deserialize;
use dirs;
use super::units::Units;

// ========== Main Config ==========
#[derive(Debug)]
pub struct Config {
    pub location: String,
    pub units: Units,
    pub use_colors: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConfigDeser {
    pub location: String,
    pub units: String,
    pub use_colors: bool,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub location: Option<String>,
    #[arg(short, long)]
    pub units: Option<Units>,
    #[arg(long)]
    pub use_colors: Option<bool>,
}

// ========== JSON Deserializing for https://ipinfo.io ==========
#[derive(Debug, Deserialize)]
struct IPInfo {
    city: String,
}

// ========== Default Config ==========
impl ConfigDeser {
    pub fn default() -> Result<Config, Error> {
        let ip_info: IPInfo = reqwest::blocking::get("https://ipinfo.io")
            .map_err(|e| Error::new(ErrorKind::Other, e))?
            .json()
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        Ok(Config {
            location: ip_info.city,
            units: Units::Metric,
            use_colors: true,
        })
    }

    pub fn load_config_file() -> Result<Config, Error> {
        let config_path = dirs::config_dir()
            .ok_or_else(|| Error::new(ErrorKind::NotFound, "Config directory not found"))?
            .join("sunny/config.toml");

        match std::fs::read_to_string(&config_path) {
            Ok(config_str) => {
                let config_toml: ConfigDeser = toml::from_str(&config_str)
                    .map_err(|e| Error::new(ErrorKind::InvalidData, format!("TOML parse error: {}", e)))?;

                Ok(Config {
                    location: config_toml.location,
                    units: Units::from_str(&config_toml.units)?,
                    use_colors: config_toml.use_colors,
                })
            }
            Err(_) => {
                Self::default()
            }
        }
    }

    pub fn load_config_args() -> Result<Config, Error> {
        let args = Args::parse();

        // Один раз читаем конфиг, если нужно
        let fallback = if args.location.is_none() || args.units.is_none() {
            Some(Self::load_config_file()?)
        } else {
            None
        };

        let location = args.location.clone().or_else(|| fallback.as_ref().map(|c| c.location.clone())).unwrap();
        let units = args.units.clone().or_else(|| fallback.as_ref().map(|c| c.units.clone())).unwrap();
        let use_color = args.use_colors.clone().or_else(|| fallback.as_ref().map(|c| c.use_colors.clone())).unwrap();

        return Ok(Config {
            location: location,
            units: units,
            use_colors: use_color
        })
    }
}