use serde::Deserialize;

const API_BASE: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Deserialize, Clone, Debug)]
pub struct WeatherResponse {
    pub current: CurrentWeather,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CurrentWeather {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: f64,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: f64,
    pub wind_gusts_10m: f64,
    pub precipitation: f64,
    pub surface_pressure: f64,
    pub weather_code: i32,
    #[allow(dead_code)]
    pub is_day: i32,
}

pub async fn fetch_weather(lat: f64, lon: f64) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "{}?latitude={}&longitude={}&current=temperature_2m,apparent_temperature,relative_humidity_2m,wind_speed_10m,wind_direction_10m,wind_gusts_10m,precipitation,surface_pressure,weather_code,is_day",
        API_BASE, lat, lon
    );
    reqwest::get(url).await?.json::<WeatherResponse>().await
}
