# Changelog

## [0.1.4]
### Add
- Good accessibility for weather data

## [0.1.3]
### Added
- Display of Country, Region, Area names

### Changed
- Further refactored [`fetch_weather`](src/weather/fetch_weather.rs) for readability and structure
- Renamed several variables for clarity
- Minor bugfix in parsing logic

## [0.1.2]
### Added
- Display of UV Index color
- Label for UV Index value

### Changed
- Major refactor of [`fetch_weather`](src/weather/fetch_weather.rs)
- Moved helper functions from `Weather` struct to [`utils`](src/weather/utils)
- Renamed internal variables for better naming consistency
- Minor code improvements

## [0.1.1]
### Added
- Display of humidity percentage

### Changed
- Improved info output and ASCII art to match Neofetch style