package api

type TemperatureUnit string

const (
	TemperatureUnit_Unspecified TemperatureUnit = ""
	TemperatureUnit_Kelvin      TemperatureUnit = "kelvin"
	TemperatureUnit_Celsius     TemperatureUnit = "celsius"
	TemperatureUnit_Fahrenheit  TemperatureUnit = "fahrenheit"
)

type WeatherSource interface {
	GetCurrentWeather(lat float64, lon float64, units TemperatureUnit) (*CurrentWeatherData, error)
}

type CurrentWeatherData struct {
	Units       TemperatureUnit
	Temperature float32
	FeelsLike   float32
	Pressure    int
	Humidity    int
}

type Geocoder interface {
	Geocode(query string) (*Coordinates, error)
	GeocodeIp() (*Coordinates, error)
}

type Coordinates struct {
	Latitude  float64
	Longitude float64
}
