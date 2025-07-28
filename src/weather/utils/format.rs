use std::io::{Error, ErrorKind};
use chrono::{NaiveDateTime, NaiveTime};

pub enum TimeKind {
    Time,
    DateTime
}

pub fn format_time(time: &str, format: &str, new_format: &str, kind: TimeKind) -> Result<String, Error> {
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

pub fn remove_char(value: &serde_json::value::Value) -> String {
    return value.as_str().unwrap_or("N/A").chars().filter(|c| *c != '"').collect::<String>()
}

pub fn remove_colors(data: String) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    return re.replace_all(&data, "").to_string()
}