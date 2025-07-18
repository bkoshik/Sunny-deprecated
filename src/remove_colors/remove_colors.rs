pub fn remove_ansi_colors(data: String) -> String {
    let re = regex::Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    return re.replace_all(&data, "").to_string()
}