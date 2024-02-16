use serde::Serialize;
use strum::Display;

pub mod geocoder;
pub mod open_weather_map;

pub trait WeatherSource {
    async fn get_weather_data(
        &self,
        lat: f64,
        lon: f64,
        units: TemperatureUnit,
    ) -> Result<WeatherData, reqwest::Error>;
}

#[derive(Debug, Display, Copy, Clone)]
pub enum TemperatureUnit {
    #[strum(serialize = "standard")]
    Kelvin,

    #[strum(serialize = "metric")]
    Celsius,

    #[strum(serialize = "imperial")]
    Fahrenheit,
}

#[derive(Debug)]
pub struct WeatherData {
    pub units: TemperatureUnit,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: i32,
    pub humidity: i32,
}
