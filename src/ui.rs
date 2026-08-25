use crate::{
    app::{App, AppState},
    braille,
};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, HorizontalAlignment, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

fn get_weather_description(code: i32) -> &'static str {
    match code {
        0 => "Clear sky",
        1..=3 => "Partly cloudy",
        45 | 48 => "Foggy",
        51 | 53 | 55 => "Drizzle",
        56 | 57 => "Freezing drizzle",
        61 | 63 | 65 => "Rain",
        66 | 67 => "Freezing rain",
        71 | 73 | 75 => "Snow",
        77 => "Snow grains",
        80..=82 => "Rain showers",
        85 | 86 => "Snow showers",
        95 => "Thunderstorm",
        96 | 99 => "Thunderstorm with hail",
        _ => "Unknown",
    }
}

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    let show_cities = !app.input_text.is_empty();
    let city_count = if show_cities {
        app.filtered_cities.len()
    } else {
        0
    };
    let city_block_height = if city_count > 0 {
        city_count as u16 + 2
    } else {
        0
    };

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(city_block_height),
            Constraint::Min(0),
        ])
        .split(chunks[0]);

    let input_block = Block::default()
        .title(" Filter City ")
        .borders(Borders::ALL);

    let input_text = format!("> {}", app.input_text);

    let input = Paragraph::new(input_text)
        .block(input_block)
        .style(Style::default().fg(Color::Blue));

    f.render_widget(input, left_chunks[0]);

    if show_cities && city_count > 0 {
        let mut city_lines: Vec<Line> = Vec::new();
        for (i, city) in app.filtered_cities.iter().enumerate() {
            if i == app.selection_index {
                city_lines.push(Line::from(vec![
                    Span::raw("> "),
                    Span::styled(city.name.clone(), Style::default().bold()),
                ]));
            } else {
                city_lines.push(Line::from(format!("  {}", city.name)));
            }
        }

        let city_block = Block::default().title(" Cities ").borders(Borders::ALL);

        let city_text = Paragraph::new(city_lines)
            .block(city_block)
            .style(Style::default().fg(Color::Blue));

        f.render_widget(city_text, left_chunks[1]);
    }

    let info_content = match &app.state {
        AppState::Loading => Paragraph::new("Loading..."),
        AppState::Error(e) => Paragraph::new(e.clone()),
        AppState::Idle => {
            if let Some(w) = &app.weather {
                let weather_desc = get_weather_description(w.current.weather_code);
                Paragraph::new(format!(
                    "Temperature: {:.1}°C\nFeels Like: {:.1}°C\nHumidity: {:.0}%\n\nWind Speed: {:.1} km/h\nWind Direction: {:.0}°\nWind Gusts: {:.1} km/h\n\nPrecipitation: {:.1} mm\nPressure: {:.0} hPa\nCondition: {}",
                    w.current.temperature_2m,
                    w.current.apparent_temperature,
                    w.current.relative_humidity_2m,
                    w.current.wind_speed_10m,
                    w.current.wind_direction_10m,
                    w.current.wind_gusts_10m,
                    w.current.precipitation,
                    w.current.surface_pressure,
                    weather_desc
                ))
                    .style(Style::default().fg(Color::Green))
            } else {
                Paragraph::new("Select a city and press Enter\nto load weather")
            }
        }
    };

    let info_block = Block::default().title(" Info ").borders(Borders::ALL);

    f.render_widget(info_content.block(info_block), left_chunks[2]);

    let right_block = Block::default()
        .title_alignment(HorizontalAlignment::Center)
        .title(" Weather ")
        .borders(Borders::ALL);

    let right_inner = right_block.inner(chunks[1]);
    f.render_widget(right_block, chunks[1]);
    braille::draw_house(f, right_inner);

    let sun_w: u16 = 20;
    let sun_h: u16 = 12;
    let sun_x = right_inner.x + right_inner.width - sun_w - 2;
    let sun_y = right_inner.y + 1;
    let sun_area = Rect::new(sun_x, sun_y, sun_w, sun_h);
    braille::draw_sun(f, sun_area, app.frame);
}
