mod api;
mod app;
mod braille;
mod handler;
mod ui;

use crate::app::{App, AppState};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use std::{error::Error, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    ratatui::run(|terminal| {
        let mut app = App::new();
        let (tx, mut rx) = mpsc::channel(10);

        loop {
            app.frame += 1;
            terminal.draw(|f| ui::render(f, &mut app))?;

            if let Ok(data) = rx.try_recv() {
                app.weather = Some(data);
                app.state = AppState::Idle;
            }

            if event::poll(Duration::from_millis(16))?
                && let Event::Key(key) = event::read()?
            {
                if key.code == KeyCode::Char('d') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }

                handler::handle_key_events(key, &mut app, tx.clone());
            }
        }
        Ok(())
    })
}
