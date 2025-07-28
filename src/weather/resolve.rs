use super::Weather;

impl Weather {
    pub(crate) fn resolve(&self, key: &str) -> Option<&str> {
        match key {
            "area" => Some(&self.area),
            "region" => Some(&self.region),
            "country" => Some(&self.country),
            "updated_time" => Some(&self.updated_time),
            
            "description" => Some(&self.description),
            "temperature" => Some(&self.temperature),
            "wind" => Some(&self.wind),
            "sunrise" => Some(&self.sunrise),
            "sunset" => Some(&self.sunset),
            "uv_index" => Some(&self.uv_index),
            "humidity" => Some(&self.humidity),
            _ => None,
        }
    }
}