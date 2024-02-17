package openweathermap

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"

	"github.com/bitcrshr/wui/api"
	"github.com/rotisserie/eris"
)

const (
	currentWeatherBaseUrl string = "https://api.openweathermap.org/data/2.5/weather"
)

type OpenWeatherMapClient struct {
	apiKey string
}

func NewOpenWeatherMapClient(apiKey string) *OpenWeatherMapClient {
	if apiKey == "" {
		panic(eris.New("api key cannot be empty"))
	}

	return &OpenWeatherMapClient{
		apiKey: apiKey,
	}
}

func (c *OpenWeatherMapClient) GetCurrentWeather(lat float64, lon float64, units api.TemperatureUnit) (*api.CurrentWeatherData, error) {
	params := url.Values{}
	params.Add("lat", fmt.Sprint(lat))
	params.Add("lon", fmt.Sprint(lon))
	params.Add("appid", c.apiKey)

	owmUnits := "standard"
	switch units {
	case api.TemperatureUnit_Celsius:
		owmUnits = "metric"
	case api.TemperatureUnit_Fahrenheit:
		owmUnits = "imperial"
	case api.TemperatureUnit_Kelvin:
		owmUnits = "standard"
	}

	params.Add("units", owmUnits)

	endpoint, err := url.Parse(
		fmt.Sprintf("%s?%s", currentWeatherBaseUrl, params.Encode()),
	)
	if err != nil {
		err = eris.Wrap(err, "failed to parse current weather endpoint url")
		return nil, err
	}

	fmt.Printf("openweathermap url: %s\n", endpoint.String())

	res, err := http.Get(endpoint.String())
	if err != nil {
		err = eris.Wrap(err, "failed to do current weather http request")
		return nil, err
	}

	if res.StatusCode != http.StatusOK {
		bodyBytes, err := io.ReadAll(res.Body)
		if err != nil {
			err = eris.Wrapf(err, "request failed with status code %d and was also unable to ready body bytes", res.StatusCode)
			return nil, err
		}

		err = eris.Errorf("current weather request failed with status code %d and had body %s", res.StatusCode, string(bodyBytes))
		return nil, err
	}

	bodyBytes, err := io.ReadAll(res.Body)
	if err != nil {
		err = eris.Wrapf(err, "current weather request returned OK, but unable to read body bytes")
		return nil, err
	}

	fmt.Printf("raw body:\n%s\n", string(bodyBytes))

	resJson := &GetCurrentWeatherResponse{}
	if err := json.Unmarshal(bodyBytes, resJson); err != nil {
		err = eris.Wrap(err, "current weather request was OK but failed to unmarshal body")
		return nil, err
	}

	if resJson.Main == nil {
		err = eris.Errorf("current weather response did not have a `main` field")
		return nil, err
	}

	return &api.CurrentWeatherData{
		Units:       units,
		Temperature: resJson.Main.Temp,
		FeelsLike:   resJson.Main.FeelsLike,
		Pressure:    resJson.Main.Pressure,
		Humidity:    resJson.Main.Humidity,
	}, nil
}

type GetCurrentWeatherResponse struct {
	Coord *struct {
		Latitude  float64 `json:"lat"`
		Longitude float64 `json:"lon"`
	} `json:"coord"`

	Weather []*struct {
		Id          int    `json:"id"`
		Main        string `json:"main"`
		Description string `json:"description"`
		Icon        string `json:"icon"`
	} `json:"weather"`

	Main *struct {
		Temp        float32 `json:"temp"`
		FeelsLike   float32 `json:"feels_like"`
		TempMin     float32 `json:"temp_min"`
		TempMax     float32 `json:"temp_max"`
		Pressure    int     `json:"pressure"`
		Humidity    int     `json:"humidity"`
		SeaLevel    int     `json:"sea_level"`
		GroundLevel int     `json:"grnd_level"`
	} `json:"main"`

	Visibility int `json:"visibility"`

	Wind *struct {
		Speed   float32 `json:"speed"`
		Degrees int     `json:"deg"`
		Gust    float32 `json:"gust"`
	} `json:"wind"`

	// TODO: more!
}
