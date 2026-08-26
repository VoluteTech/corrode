use ratatui::{Frame, layout::Rect, style::Color, symbols::Marker, widgets::canvas::Canvas};

#[derive(Clone, Copy)]
pub struct WeatherTemplate {
    sunny: bool,
    cloudy: bool,
    rainy: bool,
    snowy: bool,
    foggy: bool,
    windy: bool,
}

impl WeatherTemplate {
    pub fn from_weather(code: i32, wind_speed: f64) -> Self {
        Self {
            sunny: matches!(code, 0..=2),
            cloudy: matches!(
                code,
                1..=3 | 45 | 48 | 51..=57 | 61..=67 | 71..=77 | 80..=86 | 95..=99
            ),
            rainy: matches!(code, 51..=67 | 80..=82 | 95..=99),
            snowy: matches!(code, 71..=77 | 85 | 86),
            foggy: matches!(code, 45 | 48),
            windy: wind_speed >= 25.0,
        }
    }
}

fn filled_circle(cx: f64, cy: f64, radius: f64) -> Vec<(f64, f64)> {
    let mut points = Vec::new();
    for x in -20..=20 {
        for y in -20..=20 {
            let px = f64::from(x) / 20.0 * radius;
            let py = f64::from(y) / 20.0 * radius;
            if px * px + py * py <= radius * radius {
                points.push((cx + px, cy + py));
            }
        }
    }
    points
}

fn cloud_points() -> Vec<(f64, f64)> {
    // Half-unit sampling avoids the striped pattern produced when points line up
    // with the terminal's Braille cell grid.
    let lobes = [
        (52.0, 67.0, 10.0, 7.0),
        (63.0, 72.0, 13.0, 11.0),
        (75.0, 67.0, 11.0, 8.0),
    ];
    let mut points = Vec::new();
    for x in 80..=174 {
        for y in 116..=160 {
            let px = f64::from(x) / 2.0;
            let py = f64::from(y) / 2.0;
            if lobes.iter().any(|(cx, cy, rx, ry)| {
                let dx = (px - cx) / rx;
                let dy = (py - cy) / ry;
                dx * dx + dy * dy <= 1.0
            }) {
                points.push((px, py));
            }
        }
    }
    points
}

pub fn draw_weather(frame: &mut Frame, area: Rect, weather: WeatherTemplate, tick: u64) {
    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(move |ctx| {
            use ratatui::widgets::canvas::{Line, Points};

            // Ground and a more detailed foreground give every composition a stable scene.
            ctx.draw(&Line {
                x1: 3.0,
                y1: 8.0,
                x2: 97.0,
                y2: 8.0,
                color: Color::DarkGray,
            });
            for (x1, y1, x2, y2, color) in [
                (9.0, 8.0, 9.0, 38.0, Color::White),
                (9.0, 38.0, 27.0, 58.0, Color::White),
                (27.0, 58.0, 48.0, 38.0, Color::White),
                (48.0, 38.0, 48.0, 8.0, Color::White),
                (9.0, 8.0, 48.0, 8.0, Color::White),
                (14.0, 34.0, 27.0, 49.0, Color::DarkGray),
                (27.0, 49.0, 43.0, 34.0, Color::DarkGray),
                (32.0, 8.0, 32.0, 27.0, Color::Yellow),
                (42.0, 8.0, 42.0, 27.0, Color::Yellow),
                (32.0, 27.0, 42.0, 27.0, Color::Yellow),
                (15.0, 22.0, 25.0, 22.0, Color::LightBlue),
                (15.0, 32.0, 25.0, 32.0, Color::LightBlue),
                (15.0, 22.0, 15.0, 32.0, Color::LightBlue),
                (25.0, 22.0, 25.0, 32.0, Color::LightBlue),
                (20.0, 22.0, 20.0, 32.0, Color::DarkGray),
                (15.0, 27.0, 25.0, 27.0, Color::DarkGray),
            ] {
                ctx.draw(&Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color,
                });
            }

            if weather.sunny {
                // In mixed weather the sun sits behind the cloud's upper-right
                // edge, rather than occupying the same space as the cloud.
                let (cx, cy) = if weather.cloudy {
                    (86.0, 84.0)
                } else {
                    (75.0, 75.0)
                };
                let rotation = tick as f64 * 0.025;
                for ray in 0..12 {
                    let angle = f64::from(ray) * std::f64::consts::TAU / 12.0 + rotation;
                    ctx.draw(&Line {
                        x1: cx + angle.cos() * 11.0,
                        y1: cy + angle.sin() * 11.0,
                        x2: cx + angle.cos() * 17.0,
                        y2: cy + angle.sin() * 17.0,
                        color: Color::LightYellow,
                    });
                }
                let sun = filled_circle(cx, cy, 8.0);
                ctx.draw(&Points {
                    coords: &sun,
                    color: Color::Yellow,
                });
            }

            if weather.cloudy {
                let cloud = cloud_points();
                ctx.draw(&Points {
                    coords: &cloud,
                    color: Color::Gray,
                });
            }

            let phase = (tick % 12) as f64;
            if weather.rainy {
                for x in (47..94).step_by(7) {
                    let offset = (f64::from(x) + phase) % 12.0;
                    ctx.draw(&Line {
                        x1: f64::from(x),
                        y1: 58.0 - offset,
                        x2: f64::from(x) - 3.0,
                        y2: 50.0 - offset,
                        color: Color::LightBlue,
                    });
                    ctx.draw(&Line {
                        x1: f64::from(x),
                        y1: 42.0 - offset,
                        x2: f64::from(x) - 3.0,
                        y2: 34.0 - offset,
                        color: Color::Blue,
                    });
                }
            }

            if weather.snowy {
                let mut snow = Vec::new();
                for x in (46..95).step_by(7) {
                    for row in 0..3 {
                        let drift = ((tick + x as u64 + row * 5) % 9) as f64;
                        snow.push((
                            f64::from(x) + drift / 3.0,
                            57.0 - f64::from(row as u32) * 12.0 - drift,
                        ));
                    }
                }
                ctx.draw(&Points {
                    coords: &snow,
                    color: Color::White,
                });
            }

            if weather.foggy {
                for row in 0..5 {
                    let y = 58.0 - f64::from(row) * 8.0;
                    let fog: Vec<_> = (0..90)
                        .map(|x| {
                            let px = 8.0 + f64::from(x);
                            (px, y + (px / 8.0 + f64::from(row)).sin() * 1.5)
                        })
                        .collect();
                    ctx.draw(&Points {
                        coords: &fog,
                        color: Color::DarkGray,
                    });
                }
            }

            if weather.windy {
                for row in 0..3 {
                    let y = 31.0 - f64::from(row) * 8.0;
                    let wind: Vec<_> = (0..48)
                        .map(|x| {
                            let px = 50.0 + f64::from(x);
                            (px, y + (px / 7.0 + phase / 4.0).sin() * 2.0)
                        })
                        .collect();
                    ctx.draw(&Points {
                        coords: &wind,
                        color: Color::Cyan,
                    });
                }
            }
        });

    frame.render_widget(canvas, area);
}
