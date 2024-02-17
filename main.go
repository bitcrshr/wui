package main

import (
	"fmt"
	"os"

	"github.com/bitcrshr/wui/api"
	openweathermap "github.com/bitcrshr/wui/api/open_weather_map"
	"github.com/bitcrshr/wui/api/radar"
	_ "github.com/joho/godotenv/autoload"
	"github.com/rotisserie/eris"
)

func main() {
	owmApiKey := os.Getenv("OWM_API_KEY")
	if owmApiKey == "" {
		err := eris.Errorf("OWM_API_KEY not set in env")
		panic(err)
	}

	radarApiKey := os.Getenv("RADAR_API_KEY")
	if radarApiKey == "" {
		err := eris.Errorf("RADAR_API_KEY not set in env")
		panic(err)
	}

	owm := openweathermap.NewOpenWeatherMapClient(owmApiKey)
	radar := radar.NewRadarClient(radarApiKey)

	coords, err := radar.GeocodeIp()
	if err != nil {
		panic(err)
	}

	weather, err := owm.GetCurrentWeather(coords.Latitude, coords.Longitude, api.TemperatureUnit_Fahrenheit)
	if err != nil {
		panic(err)
	}

	fmt.Printf("The temperature is %f and it feels like %f", weather.Temperature, weather.FeelsLike)
}
