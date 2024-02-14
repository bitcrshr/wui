pub enum WeatherApi {
    OpenWeatherMap,
}

pub trait WeatherSource {
    async fn get_current_weather(&self, lat: i64, lon: i64) -> WeatherData;
}

pub struct WeatherData {
    pub temp: i32,
    pub feels_like: i32,
    pub pressure: i32,
    pub humidity: i32,
}

pub mod OpenWeatherMap {
    use super::{WeatherData, WeatherSource};

    pub struct Source {
        api_key: String,
    }
    impl WeatherSource for Source {
        async fn get_current_weather(&self, lat: i64, lon: i64) -> WeatherData {
            let res = reqwest::get(
                format!(
                    "https://api.openweathermap.org/data/2.5/weather?lat={:.2}&lon={:.2}&appid={}",
                    lat, lon, self.api_key
                )
                .as_str(),
            )
            .await
            .expect("");
        }
    }

    struct OpenWeatherMapResponse {
        pub coord: OwpCoord,
        pub weather: Ow,
    }
}
