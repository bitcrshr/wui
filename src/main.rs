use api::WeatherSource;

mod api;

#[tokio::main]
async fn main() {
    let source =
        api::open_weather_map::OpenWeatherMapSource::new("31660b6485a0f1cda89d91b9d392b4d5");

    let res = source
        .get_weather_data(39.501824, -84.7642624, api::TemperatureUnit::Fahrenheit)
        .await
        .expect("huh?");

    println!("Got weather data!\n{:?}", res);
}
