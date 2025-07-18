use std::collections::HashMap;
use std::io::Error;
use super::weather::Weather;

// ========== ╭──────────────────────────────╮ ==========
// ========== | Getting ASCII Art of Weather | ==========
// ========== ╰──────────────────────────────╯ ==========
impl Weather {
    pub fn get_ascii_art(&self) -> Result<Vec<&str>, Error> {
        let mut wwo_code: HashMap<&str, &str> = HashMap::new();
        let mut weather_ascii_art: HashMap<&str, Vec<&str>> = HashMap::new();

        // ========== WWO Code ==========
        wwo_code.insert("113", "Sunny");
        wwo_code.insert("116", "PartlyCloudy");
        wwo_code.insert("119", "Cloudy");
        wwo_code.insert("122", "VeryCloudy");
        wwo_code.insert("143", "Fog");
        wwo_code.insert("176", "LightShowers");
        wwo_code.insert("179", "LightSleetShowers");
        wwo_code.insert("182", "LightSleet");
        wwo_code.insert("185", "LightSleet");
        wwo_code.insert("200", "ThunderyShowers");
        wwo_code.insert("227", "LightSnow");
        wwo_code.insert("230", "HeavySnow");
        wwo_code.insert("248", "Fog");
        wwo_code.insert("260", "Fog");
        wwo_code.insert("263", "LightShowers");
        wwo_code.insert("266", "LightRain");
        wwo_code.insert("281", "LightSleet");
        wwo_code.insert("284", "LightSleet");
        wwo_code.insert("293", "LightRain");
        wwo_code.insert("296", "LightRain");
        wwo_code.insert("299", "HeavyShowers");
        wwo_code.insert("302", "HeavyRain");
        wwo_code.insert("305", "HeavyShowers");
        wwo_code.insert("308", "HeavyRain");
        wwo_code.insert("311", "LightSleet");
        wwo_code.insert("314", "LightSleet");
        wwo_code.insert("317", "LightSleet");
        wwo_code.insert("320", "LightSnow");
        wwo_code.insert("323", "LightSnowShowers");
        wwo_code.insert("326", "LightSnowShowers");
        wwo_code.insert("329", "HeavySnow");
        wwo_code.insert("332", "HeavySnow");
        wwo_code.insert("335", "HeavySnowShowers");
        wwo_code.insert("338", "HeavySnow");
        wwo_code.insert("350", "LightSleet");
        wwo_code.insert("353", "LightShowers");
        wwo_code.insert("356", "HeavyShowers");
        wwo_code.insert("359", "HeavyRain");
        wwo_code.insert("362", "LightSleetShowers");
        wwo_code.insert("365", "LightSleetShowers");
        wwo_code.insert("368", "LightSnowShowers");
        wwo_code.insert("371", "HeavySnowShowers");
        wwo_code.insert("374", "LightSleetShowers");
        wwo_code.insert("377", "LightSleet");
        wwo_code.insert("386", "ThunderyShowers");
        wwo_code.insert("389", "ThunderyHeavyRain");
        wwo_code.insert("392", "ThunderySnowShowers");
        wwo_code.insert("395", "HeavySnowShowers");

        // ========== Weather ASCII Art ==========
        weather_ascii_art.insert("Unknown", vec![
            "    .-.      ",
            "     __)     ",
            "    (        ",
            "     `-’     ",
            "      •      ",
        ]);
        weather_ascii_art.insert("Sunny", vec![
            "\x1b[38;5;226m    \\   /    \x1b[0m",
            "\x1b[38;5;226m     .-.     \x1b[0m",
            "\x1b[38;5;226m  ― (   ) ―  \x1b[0m",
            "\x1b[38;5;226m     `-’     \x1b[0m",
            "\x1b[38;5;226m    /   \\    \x1b[0m",
        ]);
        weather_ascii_art.insert("PartlyCloudy", vec![
            "\x1b[38;5;226m   \\  /\x1b[0m      ",
            "\x1b[38;5;226m _ /\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m   \\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "             "
        ]);
        weather_ascii_art.insert("Cloudy", vec![
            "             ",
            "\x1b[38;5;250m     .--.    \x1b[0m",
            "\x1b[38;5;250m  .-(    ).  \x1b[0m",
            "\x1b[38;5;250m (___.__)__) \x1b[0m",
            "             ",
        ]);
        weather_ascii_art.insert("VeryCloudy", vec![
            "             ",
            "\x1b[38;5;240;1m     .--.    \x1b[0m",
            "\x1b[38;5;240;1m  .-(    ).  \x1b[0m",
            "\x1b[38;5;240;1m (___.__)__) \x1b[0m",
            "             ",
        ]);
        weather_ascii_art.insert("LightShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;111m     ‘ ‘ ‘ ‘ \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
        ]);
        weather_ascii_art.insert("HeavyShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
            "\x1b[38;5;21;1m   ‚‘‚‘‚‘‚‘  \x1b[0m",
            "\x1b[38;5;21;1m   ‚’‚’‚’‚’  \x1b[0m",
        ]);
        weather_ascii_art.insert("LightSnowShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;255m     *  *  * \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
        ]);
        weather_ascii_art.insert("HeavySnowShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
            "\x1b[38;5;255;1m    * * * *  \x1b[0m",
            "\x1b[38;5;255;1m   * * * *   \x1b[0m",
        ]);
        weather_ascii_art.insert("LightSleetShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;111m     ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m* \x1b[0m",
            "\x1b[38;5;255m    *\x1b[38;5;111m ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘  \x1b[0m",
        ]);
        weather_ascii_art.insert("ThunderyShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;228;5m    ⚡\x1b[38;5;111;25m‘ ‘\x1b[38;5;228;5m⚡\x1b[38;5;111;25m‘ ‘ \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
        ]);
        weather_ascii_art.insert("ThunderyHeavyRain", vec![
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;21;1m  ‚‘\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‘‚\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‚‘ \x1b[0m",
            "\x1b[38;5;21;1m  ‚’‚’\x1b[38;5;228;5m⚡\x1b[38;5;21;25m’‚’  \x1b[0m",
        ]);
        weather_ascii_art.insert("ThunderySnowShowers", vec![
            "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
            "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
            "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
            "\x1b[38;5;255m     *\x1b[38;5;228;5m⚡\x1b[38;5;255;25m*\x1b[38;5;228;5m⚡\x1b[38;5;255;25m* \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
        ]);
        weather_ascii_art.insert("LightRain", vec![
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m",
            "\x1b[38;5;111m   ‘ ‘ ‘ ‘   \x1b[0m",
        ]);
        weather_ascii_art.insert("HeavyRain", vec![
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;21;1m  ‚‘‚‘‚‘‚‘   \x1b[0m",
            "\x1b[38;5;21;1m  ‚’‚’‚’‚’   \x1b[0m",
        ]);
        weather_ascii_art.insert("LightSnow", vec![
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;255m    *  *  *  \x1b[0m",
            "\x1b[38;5;255m   *  *  *   \x1b[0m",
        ]);
        weather_ascii_art.insert("HeavySnow", vec![
            "\x1b[38;5;240;1m     .-.     \x1b[0m",
            "\x1b[38;5;240;1m    (   ).   \x1b[0m",
            "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
            "\x1b[38;5;255;1m   * * * *   \x1b[0m",
            "\x1b[38;5;255;1m  * * * *    \x1b[0m",
        ]);
        weather_ascii_art.insert("LightSleet", vec![
            "\x1b[38;5;250m     .-.     \x1b[0m",
            "\x1b[38;5;250m    (   ).   \x1b[0m",
            "\x1b[38;5;250m   (___(__)  \x1b[0m",
            "\x1b[38;5;111m    ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m*  \x1b[0m",
            "\x1b[38;5;255m   *\x1b[38;5;111m ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘   \x1b[0m",
        ]);
        weather_ascii_art.insert("Fog", vec![
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
            "\x1b[38;5;251m  _ - _ - _  \x1b[0m",
            "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
        ]);

        let weather_code = self.stuff.code_of_weather.as_str();
        let name_of_art = wwo_code
            .get(weather_code)
            .copied()
            .unwrap_or("Unknown");

        let unknown = vec!["[ no art found ]"];
        let ascii_art: &Vec<&str> = weather_ascii_art
            .get(name_of_art)
            .unwrap_or(&unknown);

        return Ok(ascii_art.clone());
    }
}