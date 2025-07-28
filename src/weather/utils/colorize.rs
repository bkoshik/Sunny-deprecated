use crate::config::Units;

pub fn colorize_temperature(temp: &str, units: &Units) -> String {
    let temp_int = temp.parse().unwrap_or(-128);

    let color = match units {
        Units::Metric => match temp_int {
            ..-10 => "36",
            -10..0 => "34",
            0..15 => "32",
            15..25 => "33",
            25..35 => "91",
            35.. => "31",
        },
        Units::Imperial => match temp_int {
            ..15 => "36",
            15..32 => "34",
            32..59 => "32",
            59..77 => "33",
            77..95 => "91",
            95.. => "31",
        },
    };

    return format!("\x1b[{}m{}\x1b[0m", color, temp)
}

pub fn colorize_wind_speed(speed: &str, units: &Units) -> String {
    let speed_int = speed.parse().unwrap_or(-128);

    let color = match units {
        Units::Metric => match speed_int {
            ..5 => "36",
            5..15 => "32",
            15..30 => "33",
            30..50 => "91",
            50.. => "31",
        },
        Units::Imperial => match speed_int {
            ..3 => "36",
            3..10 => "32",
            10..18 => "33",
            18..31 => "91",
            31.. => "31",
        },
    };

    return format!("\x1b[{}m{}\x1b[0m", color, speed)
}

pub fn colorize_uv_index(uvi: &str, uvi_name: &str) -> String {
    let uvi_int = uvi.parse().unwrap_or(-128);

    let uvi_color = match uvi_int {
        ..3 => "32",
        3..6 => "33",
        6..8 => "91",
        8..11 => "35",
        11.. => "47;30",
    };


    return format!("\x1b[{}m{} {}\x1b[0m", uvi_color, uvi, uvi_name);
}