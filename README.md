# Sunny
### Table of Contents
- **[Installation](#installation)**
- **[Configuration](#configuration)**
- **[Command Line Args](#command-line-args)**
- **[Architecture](#architecture)**
- **[Changelog](CHANGELOG.md)**

### Example:
[![example](img/example.png)](img/example.png)

### Features
- **Get current weather from `wttr.in`**
- **ASCII-based weather art**
- **Configurable with `config.toml`**
- **Fast, built with Rust**
- **CLI support with args**

---
## Installation
### From source
1. Clone the repository:
```shell
git clone https://github.com/bkoshik/Sunny.git
cd Sunny
```

2. Build the project
```shell
cargo build --release
```

---
## Configuration
### Place:
- **Linux:** `~/.config/sunny/config.toml`
- **macOS:** `~/Library/Application\ Support/sunny/config.toml`
- **Windows:** `C:\Users\[User]\AppData\Roaming\sunny\config.toml`

### Options:
- `city`: The city for fetching weather data
- `units`: Units for temperature and wind speed
    - metric
    - imperial
- `use_colors`: Flag that removes colors

### Example:
```toml
city = "Saryagash"
units = "metric"
use_colors = true
```

---
## Command Line Args
### Options:
- `--city [city_name]`: The city for fetching weather data
- `--units {metric/imperial}`: Units for temperature and wind speed
  - metric
  - imperial
- `--use-colors {true/false}`: Flag that removes colors

### Example:
```shell
sunny --city Saryagash --units metric --use-colors true
```

---
## Architecture
### System
- **OS:** `Apple macOS`
- **IDE:** `JetBrains RustRover`
- **Language:** `Rust`
- **Build System:** `Cargo`

### File System
```text
Sunny
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── CHANGELOG.md
├── img
│   └── example.png
└── src
    ├── ascii_arts
    │   ├── mod.rs
    │   └── ascii_arts.rs
    ├── config
    │   ├── mod.rs
    │   ├── config.rs
    │   └── units.rs
    ├── main.rs
    └── weather
        ├── utils
        │   ├── mod.rs 
        │   ├── format.rs
        │   ├── get_data.rs
        │   └── colorize.rs
        ├── mod.rs
        ├── get_ascii_art.rs
        ├── fetch_weather.rs
        └── weather.rs
```