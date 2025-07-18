use super::weather::Weather;

impl Weather {
    pub fn fmt_lines(&self) -> Vec<String> {
        vec![
            format!("Weather: \x1b[1m{}\x1b[0m", self.description),
            format!("Temp: \x1b[1m{}\x1b[0m", self.temperature),
            format!("Wind: \x1b[1m{}\x1b[0m", self.wind),
            format!("Suntime: \x1b[1m{}\x1b[0m", self.suntime),
            format!("UV Index: \x1b[1m{}\x1b[0m", self.uv_index),
        ]
    }
}