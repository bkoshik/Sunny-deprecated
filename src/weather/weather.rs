use crate::config::config::Config;

// ========== Structure for Simpler Getting Data about Weather ==========
pub struct Weather {
    pub area: String,
    pub region: String,
    pub country: String,
    pub updated_time: String,

    pub description: String,
    pub temperature: String,
    pub wind: String,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index: String,
    pub humidity: String,

    pub stuff: Stuff,
}

pub struct Stuff {
    pub code_of_weather: String,
    pub config: Config,
}

impl Weather {
    pub fn new(config: Config) -> Weather {
        let stuff = Stuff {
            code_of_weather: "0".to_string(),
            config: config,
        };

        return Self {
            area: String::new(),
            region: String::new(),
            country: String::new(),
            updated_time: String::new(),

            description: String::new(),
            temperature: String::new(),
            wind: String::new(),
            sunrise: String::new(),
            sunset: String::new(),
            uv_index: String::new(),
            humidity: String::new(),

            stuff: stuff
        }
    }
}