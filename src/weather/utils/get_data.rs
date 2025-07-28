pub fn get_arrow_of_wind_direction(dir: &str) -> char {
    return match dir {
        "N" | "NNE" => '↑',
        "NE" | "ENE" => '↗',
        "E" | "ESE" => '→',
        "SE" | "SSE" => '↘',
        "S" | "SSW" => '↓',
        "SW" | "WSW" => '↙',
        "W" | "WNW" => '←',
        "NW" | "NNW" => '↖',
        _ => '?',
    }
}

pub fn get_name_of_uvi(uvi: &str) -> String {
    let uvi_int = uvi.parse().unwrap_or(-128);

    let uvi_name =  match uvi_int {
        ..3 => "Low",
        3..6 => "Moderate",
        6..8 => "High",
        8..11 => "Extremely",
        11.. => "Deathly",
    };

    return uvi_name.to_string();
}