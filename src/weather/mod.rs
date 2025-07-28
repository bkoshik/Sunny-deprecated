pub mod weather;
mod fetch_weather;
mod get_ascii_art;
mod utils;
mod resolve;

pub use weather::Weather;
pub mod render {
    pub use crate::weather::utils::format::render_weather;
}