use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use reqwest::blocking::Client;
use crate::config::Units;
use super::weather::Weather;
use super::utils::*;

// ========== ╭──────────────────────────╮ ==========
// ========== | Fetching Data of Weather | ==========
// ========== ╰──────────────────────────╯ ==========
impl Weather {
    pub fn fetch(&mut self) -> Result<&Weather, Error> {
        let client = Client::new();

        let mut params: HashMap<&str, &str> = HashMap::new();
        params.insert("format", "j1");

        let url: String = format!("https://wttr.in/{}",
                                  self.stuff.config.location);

        let response = match client.get(&url).query(&params).send() {
            Ok(response) => response,
            Err(error) => return Err(Error::new(ErrorKind::Other, error))
        };

        if let Ok(data) = response.json::<serde_json::Value>() {
            // ((for wttr.in, for format!), (for wttr.in, for format!), for suntime, for updated_time)
            let (temp_units, wind_units, time_format, timestamp_format) = match self.stuff.config.units {
                Units::Metric => (("C", "°C"), ("Kmph", "km/h"), "%H:%M", "%d.%m.%Y %H:%H"),
                Units::Imperial => (("F", "°F"), ("Miles", "mph"), "%I:%M %p", "%Y-%m-%d %I:%M %p"),
            };

            let current_data = &data["current_condition"][0];

            let region = format!("{} — {}",
                                 format::remove_char(&data["nearest_area"][0]["country"][0]["value"]),
                                 format::remove_char(&data["nearest_area"][0]["areaName"][0]["value"]));

            let updated_time = format::format_time(
                &format::remove_char(&current_data["localObsDateTime"]), "%Y-%m-%d %I:%M %p", timestamp_format, format::TimeKind::DateTime)?;

            // Description
            let description = format::remove_char(&current_data["weatherDesc"][0]["value"]);

            // Temperature
            let temp = format::remove_char(&current_data[format!("temp_{}", temp_units.0)]);
            let temp_feels = format::remove_char(&current_data[format!("FeelsLike{}", temp_units.0)]);
            let temp_display = format!("{}({}) {}",
                                       colorize::colorize_temperature(&temp, &self.stuff.config.units),
                                       colorize::colorize_temperature(&temp_feels, &self.stuff.config.units),
                                       temp_units.1
            );
            let temperature_output = if !self.stuff.config.use_colors {
                format::remove_colors(temp_display)
            } else {
                temp_display
            };

            // Wind
            let wind_dir_code = format::remove_char(&current_data["winddir16Point"]);
            let wind_dir_arrow = get_data::get_arrow_of_wind_direction(&wind_dir_code).to_string();
            let wind_speed_raw = format::remove_char(&current_data[format!("windspeed{}", wind_units.0)]);
            let wind_display = format!("{} {} {}",
                                       wind_dir_arrow,
                                       colorize::colorize_wind_speed(&wind_speed_raw, &self.stuff.config.units),
                                       wind_units.1
            );
            let wind_output = if !self.stuff.config.use_colors {
                format::remove_colors(wind_display)
            } else {
                wind_display
            };

            // Sun Time
            let sunrise = format::format_time(format::remove_char(&data["weather"][0]["astronomy"][0]["sunrise"])
                                          .as_str(), "%I:%M %p", time_format, format::TimeKind::Time)?;
            let sunset = format::format_time(format::remove_char(&data["weather"][0]["astronomy"][0]["sunset"])
                                         .as_str(), "%I:%M %p", time_format, format::TimeKind::Time)?;

            // UV-Index
            let uv_index = format::remove_char(&current_data["uvIndex"]);
            let uv_index_name = get_data::get_name_of_uvi(&uv_index);
            let uv_index_display = colorize::colorize_uv_index(&uv_index, &uv_index_name);
            let uv_index_output = if !self.stuff.config.use_colors {
                format::remove_colors(uv_index)
            } else {
                uv_index_display
            };

            // Humidity
            let humidity = format!("{}%", format::remove_char(&current_data["humidity"]));

            // Code of Weather
            let code_of_weather = format::remove_char(&current_data["weatherCode"]);

            self.region = region;
            self.updated_time = updated_time;

            self.description = description;
            self.temperature = temperature_output;
            self.wind = wind_output;
            self.sunrise = sunrise;
            self.sunset = sunset;
            self.uv_index = uv_index_output;
            self.humidity = humidity;

            self.stuff.code_of_weather = code_of_weather;

            return Ok(self);
        } else {
            return Err(Error::new(ErrorKind::InvalidData, "Invalid weather data"))
        }
    }
}