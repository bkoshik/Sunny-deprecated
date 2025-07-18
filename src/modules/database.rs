use std::collections::HashMap;

pub fn load_bases() -> (HashMap<String, String>, HashMap<String, Vec<String>>) {
    let mut wwo_code: HashMap<String, String> = HashMap::new();
    let mut weather_ascii_art: HashMap<String, Vec<String>> = HashMap::new();

    // ========== WWO Code ==========
    wwo_code.insert("116".to_string(), "PartlyCloudy".to_string());
    wwo_code.insert("119".to_string(), "Cloudy".to_string());
    wwo_code.insert("113".to_string(), "Sunny".to_string());
    wwo_code.insert("122".to_string(), "VeryCloudy".to_string());
    wwo_code.insert("143".to_string(), "Fog".to_string());
    wwo_code.insert("176".to_string(), "LightShowers".to_string());
    wwo_code.insert("179".to_string(), "LightSleetShowers".to_string());
    wwo_code.insert("182".to_string(), "LightSleet".to_string());
    wwo_code.insert("185".to_string(), "LightSleet".to_string());
    wwo_code.insert("200".to_string(), "ThunderyShowers".to_string());
    wwo_code.insert("227".to_string(), "LightSnow".to_string());
    wwo_code.insert("230".to_string(), "HeavySnow".to_string());
    wwo_code.insert("248".to_string(), "Fog".to_string());
    wwo_code.insert("260".to_string(), "Fog".to_string());
    wwo_code.insert("263".to_string(), "LightShowers".to_string());
    wwo_code.insert("266".to_string(), "LightRain".to_string());
    wwo_code.insert("281".to_string(), "LightSleet".to_string());
    wwo_code.insert("284".to_string(), "LightSleet".to_string());
    wwo_code.insert("293".to_string(), "LightRain".to_string());
    wwo_code.insert("296".to_string(), "LightRain".to_string());
    wwo_code.insert("299".to_string(), "HeavyShowers".to_string());
    wwo_code.insert("302".to_string(), "HeavyRain".to_string());
    wwo_code.insert("305".to_string(), "HeavyShowers".to_string());
    wwo_code.insert("308".to_string(), "HeavyRain".to_string());
    wwo_code.insert("311".to_string(), "LightSleet".to_string());
    wwo_code.insert("314".to_string(), "LightSleet".to_string());
    wwo_code.insert("317".to_string(), "LightSleet".to_string());
    wwo_code.insert("320".to_string(), "LightSnow".to_string());
    wwo_code.insert("323".to_string(), "LightSnowShowers".to_string());
    wwo_code.insert("326".to_string(), "LightSnowShowers".to_string());
    wwo_code.insert("329".to_string(), "HeavySnow".to_string());
    wwo_code.insert("332".to_string(), "HeavySnow".to_string());
    wwo_code.insert("335".to_string(), "HeavySnowShowers".to_string());
    wwo_code.insert("338".to_string(), "HeavySnow".to_string());
    wwo_code.insert("350".to_string(), "LightSleet".to_string());
    wwo_code.insert("353".to_string(), "LightShowers".to_string());
    wwo_code.insert("356".to_string(), "HeavyShowers".to_string());
    wwo_code.insert("359".to_string(), "HeavyRain".to_string());
    wwo_code.insert("362".to_string(), "LightSleetShowers".to_string());
    wwo_code.insert("365".to_string(), "LightSleetShowers".to_string());
    wwo_code.insert("368".to_string(), "LightSnowShowers".to_string());
    wwo_code.insert("371".to_string(), "HeavySnowShowers".to_string());
    wwo_code.insert("374".to_string(), "LightSleetShowers".to_string());
    wwo_code.insert("377".to_string(), "LightSleet".to_string());
    wwo_code.insert("386".to_string(), "ThunderyShowers".to_string());
    wwo_code.insert("389".to_string(), "ThunderyHeavyRain".to_string());
    wwo_code.insert("392".to_string(), "ThunderySnowShowers".to_string());
    wwo_code.insert("395".to_string(), "HeavySnowShowers".to_string());

    // ========== Weather ASCII Art ==========
    weather_ascii_art.insert("Unknown".to_string(), vec![
        "    .-.      ".to_string(),
        "     __)     ".to_string(),
        "    (        ".to_string(),
        "     `-’     ".to_string(),
        "      •      ".to_string(),
    ]);
    weather_ascii_art.insert("Sunny".to_string(), vec![
        "\x1b[38;5;226m    \\   /    \x1b[0m".to_string(),
        "\x1b[38;5;226m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;226m  ― (   ) ―  \x1b[0m".to_string(),
        "\x1b[38;5;226m     `-’     \x1b[0m".to_string(),
        "\x1b[38;5;226m    /   \\    \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("PartlyCloudy".to_string(), vec![
        "\x1b[38;5;226m   \\  /\x1b[0m      ".to_string(),
        "\x1b[38;5;226m _ /\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m   \\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "             ".to_string(),
    ]);
    weather_ascii_art.insert("Cloudy".to_string(), vec![
        "             ".to_string(),
        "\x1b[38;5;250m     .--.    \x1b[0m".to_string(),
        "\x1b[38;5;250m  .-(    ).  \x1b[0m".to_string(),
        "\x1b[38;5;250m (___.__)__) \x1b[0m".to_string(),
        "             ".to_string(),
    ]);
    weather_ascii_art.insert("VeryCloudy".to_string(), vec![
        "             ".to_string(),
        "\x1b[38;5;240;1m     .--.    \x1b[0m".to_string(),
        "\x1b[38;5;240;1m  .-(    ).  \x1b[0m".to_string(),
        "\x1b[38;5;240;1m (___.__)__) \x1b[0m".to_string(),
        "             ".to_string(),
    ]);
    weather_ascii_art.insert("LightShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;111m     ‘ ‘ ‘ ‘ \x1b[0m".to_string(),
        "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("HeavyShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;21;1m   ‚‘‚‘‚‘‚‘  \x1b[0m".to_string(),
        "\x1b[38;5;21;1m   ‚’‚’‚’‚’  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("LightSnowShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;255m     *  *  * \x1b[0m".to_string(),
        "\x1b[38;5;255m    *  *  *  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("HeavySnowShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;255;1m    * * * *  \x1b[0m".to_string(),
        "\x1b[38;5;255;1m   * * * *   \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("LightSleetShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;111m     ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m* \x1b[0m".to_string(),
        "\x1b[38;5;255m    *\x1b[38;5;111m ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("ThunderyShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;228;5m    ⚡\x1b[38;5;111;25m‘ ‘\x1b[38;5;228;5m⚡\x1b[38;5;111;25m‘ ‘ \x1b[0m".to_string(),
        "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("ThunderyHeavyRain".to_string(), vec![
        "\x1b[38;5;240;1m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;240;1m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;240;1m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;21;1m  ‚‘\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‘‚\x1b[38;5;228;5m⚡\x1b[38;5;21;25m‚‘ \x1b[0m".to_string(),
        "\x1b[38;5;21;1m  ‚’‚’\x1b[38;5;228;5m⚡\x1b[38;5;21;25m’‚’  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("ThunderySnowShowers".to_string(), vec![
        "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m".to_string(),
        "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m".to_string(),
        "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m".to_string(),
        "\x1b[38;5;255m     *\x1b[38;5;228;5m⚡\x1b[38;5;255;25m*\x1b[38;5;228;5m⚡\x1b[38;5;255;25m* \x1b[0m".to_string(),
        "\x1b[38;5;255m    *  *  *  \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("LightRain".to_string(), vec![
        "\x1b[38;5;250m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;250m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;250m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;111m    ‘ ‘ ‘ ‘  \x1b[0m".to_string(),
        "\x1b[38;5;111m   ‘ ‘ ‘ ‘   \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("HeavyRain".to_string(), vec![
        "\x1b[38;5;240;1m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;240;1m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;240;1m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;21;1m  ‚‘‚‘‚‘‚‘   \x1b[0m".to_string(),
        "\x1b[38;5;21;1m  ‚’‚’‚’‚’   \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("LightSnow".to_string(), vec![
        "\x1b[38;5;250m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;250m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;250m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;255m    *  *  *  \x1b[0m".to_string(),
        "\x1b[38;5;255m   *  *  *   \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("HeavySnow".to_string(), vec![
        "\x1b[38;5;240;1m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;240;1m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;240;1m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;255;1m   * * * *   \x1b[0m".to_string(),
        "\x1b[38;5;255;1m  * * * *    \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("LightSleet".to_string(), vec![
        "\x1b[38;5;250m     .-.     \x1b[0m".to_string(),
        "\x1b[38;5;250m    (   ).   \x1b[0m".to_string(),
        "\x1b[38;5;250m   (___(__)  \x1b[0m".to_string(),
        "\x1b[38;5;111m    ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘ \x1b[38;5;255m*  \x1b[0m".to_string(),
        "\x1b[38;5;255m   *\x1b[38;5;111m ‘ \x1b[38;5;255m*\x1b[38;5;111m ‘   \x1b[0m".to_string(),
    ]);
    weather_ascii_art.insert("Fog".to_string(), vec![
        "\x1b[38;5;251m _ - _ - _ - \x1b[0m".to_string(),
        "\x1b[38;5;251m _ - _ - _ - \x1b[0m".to_string(),
        "\x1b[38;5;251m _ - _ - _ - \x1b[0m".to_string(),
        "\x1b[38;5;251m  _ - _ - _  \x1b[0m".to_string(),
        "\x1b[38;5;251m _ - _ - _ - \x1b[0m".to_string(),
    ]);

    return (wwo_code, weather_ascii_art)
}