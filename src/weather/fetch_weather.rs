use std::collections::HashMap;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use std::io::{Error, ErrorKind};
use reqwest::blocking::Client;
use super::weather::Weather;
use crate::config::Units;
use crate::remove_colors::remove_ansi_colors;

enum TimeKind {
    Time,
    DateTime
}

// ========== ╭──────────────────────────╮ ==========
// ========== | Fetching Data of Weather | ==========
// ========== ╰──────────────────────────╯ ==========
impl Weather {
    pub fn fetch(&mut self) -> Result<&Weather, Error> {
        let client = Client::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("format", "j1");

        let url: String = format!(
            "https://wttr.in/{}",
            self.stuff.config.city,
        );

        let response = match client.get(&url).query(&params).send() {
            Ok(response) => response,
            Err(error) => return Err(Error::new(ErrorKind::Other, error))
        };

        if let Ok(data) = response.json::<serde_json::Value>() {
            let (temp_units, wind_units, time_format, timestamp_format) = match self.stuff.config.units {
                Units::Metric => (("C", "°C"), ("Kmph", "km/h"), "%H:%M", "%d.%m.%Y"),
                Units::Imperial => (("F", "°F"), ("Miles", "mph"), "%I:%M %p", "%Y-%m-%d")
            };

            let current_data = &data["current_condition"][0];

            let region = format!("{} — {}",
                                 remove_char(&data["nearest_area"][0]["country"][0]["value"].as_str().unwrap_or("N/A"), '"'),
                                 remove_char(&data["nearest_area"][0]["areaName"][0]["value"].as_str().unwrap_or("N/A"), '"')
            );

            let updated_time = format_time(remove_char(&current_data["localObsDateTime"]
                .as_str().unwrap_or("N/A"), '"').as_str(), "%Y-%m-%d %I:%M %p", timestamp_format, TimeKind::DateTime)?;

            // Weather
            let description = remove_char(&current_data["weatherDesc"][0]["value"].as_str().unwrap_or("N/A"), '"');
            let real_temp = remove_char(
                                 &current_data[format!("temp_{}", temp_units.0)].as_str().unwrap_or("N/A"), '"'
            ).parse().unwrap_or(-128);
            let temp_feel_like = remove_char(
                                 &current_data[format!("FeelsLike{}", temp_units.0)].as_str().unwrap_or("N/A"), '"'
            ).parse().unwrap_or(-128);
            let mut temperature = format!("{}({}) {}",
                                      colorize_temperature(real_temp, &self.stuff.config.units),
                                      colorize_temperature(temp_feel_like, &self.stuff.config.units),
                                      temp_units.1
            );
            temperature = if !self.stuff.config.use_colors {
                remove_ansi_colors(temperature)
            } else {
                temperature
            };

            // Wind
            let mut wind_dir = remove_char(current_data["winddir16Point"].as_str().unwrap_or("N/A"), '"');
            wind_dir = wind_direction_to_arrow(&wind_dir).to_string();
            let wind_speed = remove_char(&current_data[format!("windspeed{}", wind_units.0)].as_str().unwrap_or("N/A"), '"');
            let mut wind = format!("{} {} {}",
                               wind_dir,
                               colorize_wind_speed(wind_speed.parse().unwrap_or(-128), &self.stuff.config.units),
                               wind_units.1
            );
            wind = if !self.stuff.config.use_colors {
                remove_ansi_colors(wind)
            } else {
                wind
            };

            // Sun Time
            let sunrise = format_time(remove_char(&data["weather"][0]["astronomy"][0]["sunrise"]
                .as_str().unwrap_or("N/A"), '"').as_str(), "%I:%M %p", time_format, TimeKind::Time)?;
            let sunset = format_time(remove_char(&data["weather"][0]["astronomy"][0]["sunset"]
                .as_str().unwrap_or("N/A"), '"').as_str(), "%I:%M %p", time_format, TimeKind::Time)?;
            let suntime = format!("{} - {}", sunrise, sunset);

            // UV-Index
            let uv_index = remove_char(&current_data["uvIndex"].to_string(), '"');

            self.region = region;
            self.updated_time = updated_time;

            self.description = description;
            self.temperature = temperature;
            self.wind = wind;
            self.suntime = suntime;
            self.uv_index = uv_index;

            self.stuff.code_of_weather = remove_char(current_data["weatherCode"].as_str().unwrap_or("N/A"), '"');

            return Ok(self);
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid weather data"))
        }
    }
}

fn format_time(time: &str, format: &str, new_format: &str, kind: TimeKind) -> Result<String, Error> {
    match kind {
        TimeKind::DateTime => Ok(
            NaiveDateTime::parse_from_str(time, format)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
                .format(new_format)
                .to_string()
        ),
        TimeKind::Time => Ok(
            NaiveTime::parse_from_str(time, format)
                .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
                .format(new_format)
                .to_string()
        ),
    }
}

fn remove_char(string: &str, character: char) -> String{
    return string.chars().filter(|c| *c != character).collect::<String>()
}

pub fn colorize_temperature(value: i32, units: &Units) -> String {
    // Границы в градусах Цельсия и Фаренгейта
    let color = match units {
        Units::Metric => match value {
            ..-10         => "\x1b[36m",
            -10..0           => "\x1b[34m",
            0..15          => "\x1b[32m",
            15..25          => "\x1b[33m",
            25..35          => "\x1b[91m",
            _                     => "\x1b[31m",
        },
        Units::Imperial => match value {
            ..15          => "\x1b[36m",
            15..32          => "\x1b[34m",
            32..59          => "\x1b[32m",
            59..77          => "\x1b[33m",
            77..95          => "\x1b[91m",
            _                     => "\x1b[31m",
        },
    };

    return format!("{}{}\x1b[0m", color, value)
}

pub fn colorize_wind_speed(speed: i32, units: &Units) -> String {
    let color = match units {
        Units::Metric => match speed {
            0..5     => "\x1b[36m",
            5..15    => "\x1b[32m",
            15..30   => "\x1b[33m",
            30..50   => "\x1b[91m",
            _         => "\x1b[31m",
        },
        Units::Imperial => match speed {
            0..3     => "\x1b[36m",
            3..10    => "\x1b[32m",
            10..18   => "\x1b[33m",
            18..31   => "\x1b[91m",
            _         => "\x1b[31m",
        },
    };

    format!("{color}{speed}\x1b[0m")
}

pub fn wind_direction_to_arrow(dir: &str) -> &str {
    match dir {
        "N" | "NNE"   => "↑",
        "NE" | "ENE"  => "↗",
        "E" | "ESE"  => "→",
        "SE" | "SSE"  => "↘",
        "S" | "SSW"  => "↓",
        "SW" | "WSW" => "↙",
        "W" | "WNW" => "←",
        "NW" | "NNW"  => "↖",
        _     => "?",
    }
}