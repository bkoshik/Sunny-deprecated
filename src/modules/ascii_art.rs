use std::collections::HashMap;
use std::io::Error;
use super::weather::Weather;

// ========== ╭──────────────────────────────╮ ==========
// ========== | Getting ASCII Art of Weather | ==========
// ========== ╰──────────────────────────────╯ ==========
impl Weather {
    pub fn get_ascii_art(&self, wwo_code: HashMap<String, String>, weather_ascii_art: HashMap<String, Vec<String>>) -> Result<Vec<String>, Error> {
        let unknown = "Unknown".to_string();
        let weather_code = self.stuff.code_of_weather.as_str();
        let name_of_art = wwo_code
            .get(weather_code)
            .unwrap_or(&unknown);

        let unknown = vec!["[ no art found ]".to_string()];
        let ascii_art: &Vec<String> = weather_ascii_art
            .get(name_of_art)
            .unwrap_or(&unknown);

        return Ok(ascii_art.clone());
    }
}