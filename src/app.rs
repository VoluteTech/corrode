use crate::api::WeatherResponse;

pub enum AppState {
    Idle,
    Loading,
    Error(String),
}

#[derive(Clone)]
pub struct City {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

pub struct App {
    pub cities: Vec<City>,
    pub filtered_cities: Vec<City>,
    pub selection_index: usize,
    pub input_text: String,
    pub weather: Option<WeatherResponse>,
    pub state: AppState,
    pub frame: u64,
}

impl App {
    pub fn new() -> Self {
        let cities = vec![
            City {
                name: "Paris".into(),
                lat: 48.85,
                lon: 2.35,
            },
            City {
                name: "Tokyo".into(),
                lat: 35.68,
                lon: 139.69,
            },
            City {
                name: "New York".into(),
                lat: 40.71,
                lon: -74.00,
            },
            City {
                name: "London".into(),
                lat: 51.50,
                lon: -0.12,
            },
            City {
                name: "Berlin".into(),
                lat: 52.52,
                lon: 13.40,
            },
            City {
                name: "Sydney".into(),
                lat: -33.86,
                lon: 151.20,
            },
            City {
                name: "Dubai".into(),
                lat: 25.20,
                lon: 55.27,
            },
            City {
                name: "Singapore".into(),
                lat: 1.35,
                lon: 103.81,
            },
            City {
                name: "Los Angeles".into(),
                lat: 34.05,
                lon: -118.24,
            },
            City {
                name: "Rome".into(),
                lat: 41.90,
                lon: 12.49,
            },
            City {
                name: "Madrid".into(),
                lat: 40.41,
                lon: -3.70,
            },
            City {
                name: "Amsterdam".into(),
                lat: 52.36,
                lon: 4.90,
            },
            City {
                name: "Toronto".into(),
                lat: 43.65,
                lon: -79.38,
            },
            City {
                name: "Seoul".into(),
                lat: 37.56,
                lon: 126.97,
            },
            City {
                name: "Mumbai".into(),
                lat: 19.07,
                lon: 72.87,
            },
            City {
                name: "Cairo".into(),
                lat: 30.04,
                lon: 31.23,
            },
            City {
                name: "Moscow".into(),
                lat: 55.75,
                lon: 37.61,
            },
            City {
                name: "Rio de Janeiro".into(),
                lat: -22.90,
                lon: -43.17,
            },
            City {
                name: "Beijing".into(),
                lat: 39.90,
                lon: 116.40,
            },
            City {
                name: "Bangkok".into(),
                lat: 13.75,
                lon: 100.51,
            },
            City {
                name: "Stockholm".into(),
                lat: 59.32,
                lon: 18.06,
            },
            City {
                name: "Oslo".into(),
                lat: 59.91,
                lon: 10.75,
            },
            City {
                name: "Copenhagen".into(),
                lat: 55.67,
                lon: 12.56,
            },
            City {
                name: "Helsinki".into(),
                lat: 60.16,
                lon: 24.93,
            },
            City {
                name: "Vienna".into(),
                lat: 48.20,
                lon: 16.37,
            },
        ];

        Self {
            cities: cities.clone(),
            filtered_cities: cities,
            selection_index: 0,
            input_text: String::new(),
            weather: None,
            state: AppState::Idle,
            frame: 0,
        }
    }

    pub fn next(&mut self) {
        let len = self.filtered_cities.len();
        if len == 0 {
            return;
        }
        if self.selection_index >= len - 1 {
            self.selection_index = 0;
        } else {
            self.selection_index += 1;
        }
    }

    pub fn previous(&mut self) {
        let len = self.filtered_cities.len();
        if len == 0 {
            return;
        }
        if self.selection_index == 0 {
            self.selection_index = len - 1;
        } else {
            self.selection_index -= 1;
        }
    }

    pub fn filter_cities(&mut self) {
        if self.input_text.is_empty() {
            self.filtered_cities = vec![];
            self.selection_index = 0;
            return;
        }

        let query_lower = self.input_text.to_lowercase();
        self.filtered_cities = self
            .cities
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        self.selection_index = 0;
    }

    pub fn get_selected_city(&self) -> Option<&City> {
        if self.selection_index < self.filtered_cities.len() {
            Some(&self.filtered_cities[self.selection_index])
        } else {
            None
        }
    }

    pub fn find_city_by_name(&self, name: &str) -> Option<City> {
        self.cities
            .iter()
            .find(|c| c.name.to_lowercase() == name.to_lowercase())
            .cloned()
    }
}
