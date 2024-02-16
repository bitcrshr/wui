use super::{TemperatureUnit, WeatherData, WeatherSource};
use serde::{Deserialize, Serialize};

pub const OPEN_WEATHER_MAP_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/weather";

pub struct OpenWeatherMapSource {
    api_key: String,
}
impl OpenWeatherMapSource {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: String::from(api_key),
        }
    }
}
impl WeatherSource for OpenWeatherMapSource {
    async fn get_weather_data(
        &self,
        lat: f64,
        lon: f64,
        units: TemperatureUnit,
    ) -> Result<WeatherData, reqwest::Error> {
        let mut req = OpenWeatherMapRequest::new(lat, lon, self.api_key.to_string());
        req = req.units(units);

        let url = format!(
            "{}?{}",
            OPEN_WEATHER_MAP_ENDPOINT,
            serde_url_params::to_string(&req).expect("must work!!!"),
        );

        println!("request:\n{:?}", url.clone());

        let mut res_raw = reqwest::get(url.clone()).await?;
        let res_text = res_raw.text().await?;
        println!("got owm response: {:?}", res_text);
        res_raw = reqwest::get(url).await?;
        let res: OpenWeatherMapResponse = res_raw.json().await?;

        Ok(WeatherData {
            units,
            temp: res.main.temp,
            feels_like: res.main.feels_like,
            pressure: res.main.pressure,
            humidity: res.main.humidity,
        })
    }
}

/// Represents a request to the OpenWeatherMap API.
#[derive(Debug, Serialize)]
pub struct OpenWeatherMapRequest {
    /// Latitude of the location.
    pub lat: f64,
    /// Longitude of the location.
    pub lon: f64,
    /// Your unique API key.
    pub appid: String,
    /// Response format. Possible values are xml and html.
    /// JSON is used by default if this parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Units of measurement. Possible values are standard, metric, and imperial.
    /// Standard units are used by default if this parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units: Option<String>,
    /// Language for the output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
impl OpenWeatherMapRequest {
    /// Constructs a new `OpenWeatherMapRequest`.
    pub fn new(lat: f64, lon: f64, appid: String) -> Self {
        OpenWeatherMapRequest {
            lat,
            lon,
            appid,
            mode: None,
            units: None,
            lang: None,
        }
    }

    /// Sets the response format.
    pub fn mode(mut self, mode: String) -> Self {
        self.mode = Some(mode);
        self
    }

    /// Sets the units of measurement.
    pub fn units(mut self, units: TemperatureUnit) -> Self {
        self.units = Some(units.to_string());
        self
    }

    /// Sets the output language.
    pub fn lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }
}

/// Represents the entire response from the OpenWeatherMap API.
#[derive(Debug, Deserialize)]
pub struct OpenWeatherMapResponse {
    /// Geographic coordinates of the location.
    pub coord: Coord,
    /// Information about the current weather conditions.
    pub weather: Vec<Weather>,
    /// Internal parameter.
    pub base: String,
    /// Main weather parameters.
    pub main: Main,
    /// Visibility distance in meters. The maximum value is 10 km.
    pub visibility: i32,
    /// Wind information.
    pub wind: Wind,
    /// Cloudiness information.
    pub clouds: Clouds,
    /// Rain volume for the last 1 and 3 hours, if available.
    pub rain: Option<Rain>,
    /// Snow volume for the last 1 and 3 hours, if available.
    pub snow: Option<Snow>,
    /// Time of data calculation, unix, UTC.
    pub dt: u64,
    /// System parameters.
    pub sys: Sys,
    /// Shift in seconds from UTC.
    pub timezone: i32,
    /// City ID.
    pub id: i32,
    /// City name.
    pub name: String,
    /// Internal parameter.
    pub cod: i32,
}

/// Geographic coordinates.
#[derive(Debug, Deserialize)]
pub struct Coord {
    /// Longitude of the location.
    pub lon: f64,
    /// Latitude of the location.
    pub lat: f64,
}

/// Weather condition information.
#[derive(Debug, Deserialize)]
pub struct Weather {
    /// Weather condition ID.
    pub id: i32,
    /// Group of weather parameters (Rain, Snow, Clouds, etc.).
    pub main: String,
    /// Weather condition within the group.
    pub description: String,
    /// Weather icon ID.
    pub icon: String,
}

/// Main weather parameters.
#[derive(Debug, Deserialize)]
pub struct Main {
    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f32,
    /// Temperature accounting for human perception of weather.
    pub feels_like: f32,
    /// Atmospheric pressure on the sea level, hPa.
    pub pressure: i32,
    /// Humidity, %.
    pub humidity: i32,
    /// Minimum temperature at the moment.
    pub temp_min: f64,
    /// Maximum temperature at the moment.
    pub temp_max: f64,
    /// Atmospheric pressure on the sea level, hPa (optional).
    pub sea_level: Option<i32>,
    /// Atmospheric pressure on the ground level, hPa (optional).
    pub grnd_level: Option<i32>,
}

/// Wind information.
#[derive(Debug, Deserialize)]
pub struct Wind {
    /// Wind speed. Unit Default: meter/sec.
    pub speed: f64,
    /// Wind direction, degrees (meteorological).
    pub deg: i32,
    /// Wind gust. Unit Default: meter/sec (optional).
    pub gust: Option<f64>,
}

/// Cloudiness information.
#[derive(Debug, Deserialize)]
pub struct Clouds {
    /// Cloudiness, %.
    pub all: i32,
}

/// Rain volume information.
#[derive(Debug, Deserialize)]
pub struct Rain {
    /// Rain volume for the last 1 hour, mm (optional).
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
    /// Rain volume for the last 3 hours, mm (optional).
    #[serde(rename = "3h")]
    pub three_hours: Option<f64>,
}

/// Snow volume information.
#[derive(Debug, Deserialize)]
pub struct Snow {
    /// Snow volume for the last 1 hour, mm (optional).
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
    /// Snow volume for the last 3 hours, mm (optional).
    #[serde(rename = "3h")]
    pub three_hours: Option<f64>,
}

/// System parameters.
#[derive(Debug, Deserialize)]
pub struct Sys {
    /// Internal parameter (optional).
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    /// Internal parameter (optional).
    pub id: Option<i32>,
    /// Internal parameter (optional).
    pub message: Option<f64>,
    /// Country code (GB, JP, etc.).
    pub country: String,
    /// Sunrise time, unix, UTC.
    pub sunrise: u64,
    /// Sunset time, unix, UTC.
    pub sunset: u64,
}
