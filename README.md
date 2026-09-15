<div align="center">

# ☀️ Corrode

**A small, animated weather forecast for your terminal.**

Corrode searches for a city, fetches its current conditions, and turns them into a responsive Braille weather scene—without leaving the command line.

[Installation](#installation) · [Usage](#usage) · [Keybindings](#keybindings) · [Development](#development)

</div>

---

## Features

- Fast keyboard-driven city filtering
- Current temperature, apparent temperature, humidity, precipitation, pressure, and wind details
- Animated Braille scenes rendered directly in the terminal
- Composable sun, cloud, rain, snow, fog, and wind effects
- Responsive interface powered by [Ratatui](https://ratatui.rs/)
- Weather data provided by [Open-Meteo](https://open-meteo.com/)

## Installation

### Requirements

- A terminal with Unicode and color support
- An internet connection for weather requests
- [Rust](https://www.rust-lang.org/tools/install) 1.85 or newer when building from source

### Build from source

```sh
git clone https://github.com/VoluteTech/corrode.git
cd corrode
cargo build --release
```

The optimized executable will be available at:

```text
target/release/corrode
```

Run it directly:

```sh
./target/release/corrode
```

Or install it into Cargo's binary directory, which is usually `~/.cargo/bin`:

```sh
cargo install --path .
corrode
```

> Ensure `~/.cargo/bin` is included in your `PATH` when using `cargo install`.

## Usage

Start Corrode:

```sh
corrode
```

1. Begin typing a city name in the **Filter City** panel.
2. Use <kbd>↑</kbd> and <kbd>↓</kbd> to highlight a matching city.
3. Press <kbd>Enter</kbd> to autocomplete the city and request its weather.
4. Read the current conditions in the **Info** panel and view the matching animated scene in the **Weather** panel.
5. Press <kbd>Ctrl</kbd>+<kbd>D</kbd> when you are ready to quit.

Corrode currently searches its built-in list of supported cities. An invalid or incomplete selection is reported in the Info panel.

## Keybindings

| Key | Action |
| --- | --- |
| <kbd>↑</kbd> / <kbd>↓</kbd> | Select the previous or next matching city |
| <kbd>Ctrl</kbd>+<kbd>K</kbd> / <kbd>Ctrl</kbd>+<kbd>J</kbd> | Select the previous or next city |
| <kbd>Ctrl</kbd>+<kbd>P</kbd> / <kbd>Ctrl</kbd>+<kbd>N</kbd> | Select the previous or next city |
| <kbd>Enter</kbd> | Choose the highlighted city and fetch its weather |
| <kbd>Backspace</kbd> | Remove the last filter character |
| <kbd>Esc</kbd> | Clear the filter and selection |
| <kbd>Ctrl</kbd>+<kbd>D</kbd> | Quit Corrode |

## Weather scenes

Corrode translates Open-Meteo weather codes into visual layers. Conditions can be combined, so a forecast may display sun behind clouds, rain beneath cloud cover, or wind passing through a snowy scene.

The wind layer appears when the current wind speed reaches **25 km/h**.

## Development

Run the application in development mode:

```sh
cargo run
```

Before submitting a change, run the complete check chain:

```sh
cargo fmt --check
cargo check
cargo clippy -- -D warnings
cargo test
```

### Technology

- Rust
- Ratatui and Crossterm
- Tokio
- Reqwest and Serde
- Open-Meteo API

## Privacy and network access

Corrode sends the selected city's latitude and longitude to the Open-Meteo forecast API. It does not require an API key and does not store personal information.

## License

No license has been published for this project yet. Unless a license is added, all rights are reserved by the project owner.
