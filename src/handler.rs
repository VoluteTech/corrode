use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tokio::sync::mpsc;

use crate::{
    api,
    app::{App, AppState},
};

pub fn handle_key_events(
    key_event: KeyEvent,
    app: &mut App,
    tx: mpsc::Sender<api::WeatherResponse>,
) {
    let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);

    match key_event.code {
        KeyCode::Down => {
            app.next();
        }
        KeyCode::Up => {
            app.previous();
        }
        KeyCode::Char('j') if ctrl => {
            app.next();
        }
        KeyCode::Char('k') if ctrl => {
            app.previous();
        }
        KeyCode::Char('n') if ctrl => {
            app.next();
        }
        KeyCode::Char('p') if ctrl => {
            app.previous();
        }
        KeyCode::Char('y') if ctrl => {
            if let Some(city) = app.get_selected_city() {
                app.input_text = city.name.clone();
                app.filter_cities();
                app.filtered_cities = vec![];
            }
        }
        KeyCode::Enter => {
            let city = app
                .get_selected_city()
                .cloned()
                .or_else(|| app.find_city_by_name(&app.input_text));

            if let Some(city) = city {
                app.input_text = city.name;
                app.filtered_cities.clear();
                app.selection_index = 0;
                app.state = AppState::Loading;

                let tx_clone = tx.clone();
                let (lat, lon) = (city.lat, city.lon);

                tokio::spawn(async move {
                    if let Ok(data) = api::fetch_weather(lat, lon).await {
                        let _ = tx_clone.send(data).await;
                    }
                });
            } else {
                app.state = AppState::Error("Please select a city".into());
            }
        }
        KeyCode::Backspace => {
            app.input_text.pop();
            app.filter_cities();
        }
        KeyCode::Char(c) => {
            app.input_text.push(c);
            app.filter_cities();
        }
        KeyCode::Esc => {
            app.input_text.clear();
            app.filtered_cities.clear();
            app.selection_index = 0;
        }
        _ => {}
    }
}
