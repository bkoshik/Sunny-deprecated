/*!
Main function does:
  1. Creating new thread for loading 2 hashmaps of ASCII arts into RAM
  2. Make config and weather variable
  3. Fetching data of weather from wttr.in
  4. Getting a result from second thread
  5. Getting ASCII art that chooses by wwo_code
  6. Getting formatted lines with name of data, separator and data
  7. Printing information
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
    let handle_load_base = std::thread::spawn(move || {
        return load_ascii_arts();
    });

    // New Weather variable
    let mut weather = Weather::new(ConfigDeser::load_config_args()?);

    // Fetching data of weather
    let _ = weather.fetch()?;

    // Waiting for the second thread and getting a result
    let (wwo_code, ascii_art_db) = handle_load_base.join().unwrap();

    // Getting ASCII art of weather
    let ascii_art = weather.get_ascii_art(wwo_code, ascii_art_db)?;

    // Formatting lines -- Name: Data
    let lines = weather.fmt_lines();

    // Printing lines
    for (line_of_art, data) in ascii_art.iter().zip(&lines) {
        println!("{} {}", line_of_art, data);
    }
    println!("\n{} | {}", weather.region, weather.updated_time);

    Ok(())
}