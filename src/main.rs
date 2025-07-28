/*!
Main function does:
  1. Creating new thread for loading 2 hashmaps of ASCII arts into RAM
  2. Make config and weather variable
  3. Fetching data of weather from wttr.in
  4. Getting a result from second thread
  5. Getting ASCII art that chooses by wwo_code
  6. Getting formatted lines with name of data, separator and data
  7. Option<Remove colors>
  8. Printing information
!*/

use std::io::Error;
use crate::ascii_arts::load_ascii_arts;
use crate::config::ConfigDeser;
use crate::weather::Weather;

mod weather;
mod config;
mod ascii_arts;

fn main() -> Result<(), Error> {
    // Creating new thread
    let handle_load_base = std::thread::spawn(load_ascii_arts);

    let config = ConfigDeser::load_config_args()?;

    // New Weather variable
    let mut weather = Weather::new(config);

    // Fetching data of weather
    let _ = weather.fetch()?;

    // Waiting for the second thread and getting a result
    let (wwo_code, ascii_art_db) = handle_load_base.join().unwrap();

    // Getting ASCII art of weather
    let ascii_art = weather.get_ascii_art(wwo_code, ascii_art_db)?;

    // Formatting lines
    let info_lines = vec![
        format!("Description: {}", weather.description),
        format!("Temperature: {}", weather.temperature),
        format!("Wind: {}", weather.wind),
        format!("Suntime: {} - {}", weather.sunrise, weather.sunset),
        format!("UV Index: {}", weather.uv_index),
        format!("Humidity: {}", weather.humidity),
    ];

    // Printing lines
    println!("Report: {} ─ {} | {}\n", weather.country, weather.region, weather.updated_time);
    for i in 0..5.max(info_lines.len()) {
        let ascii = ascii_art.get(i).unwrap_or(&"".to_string()).to_string();
        let info = info_lines.get(i).unwrap_or(&"".to_string()).to_string();
        println!("{:<13} {}", ascii, info);
    }

    Ok(())
}