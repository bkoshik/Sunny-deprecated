use std::io::Error;
use crate::modules::config::ConfigDeser;
use crate::modules::weather::Weather;

mod modules;

fn main() -> Result<(), Error> {
    let mut weather = Weather::new(ConfigDeser::load_config_args()?);
    let _ = weather.fetch()?;
    let ascii_art = weather.get_ascii_art()?;
    let lines = weather.fmt_lines();

    for (line_of_art, data) in ascii_art.iter().zip(&lines) {
        println!("{} {}", line_of_art, data);
    }
    println!("\n{} | {}", weather.region, weather.updated_time);

    Ok(())
}