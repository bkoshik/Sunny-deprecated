use crate::config::config::Config;

// ========== Structure for Simpler Getting Data about Weather ==========
pub struct Weather {
    pub region: String,
    pub updated_time: String,

    pub description: String,
    pub temperature: String,
    pub wind: String,
    pub suntime: String,
    pub uv_index: String,

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
            region: String::new(),
            updated_time: String::new(),

            description: String::new(),
            temperature: String::new(),
            wind: String::new(),
            suntime: String::new(),
            uv_index: String::new(),

            stuff: stuff
        }
    }
}