package radar

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
	baseGeocodeEndpoint   string = "https://api.radar.io/v1/search/autocomplete"
	baseGeocodeIpEndpoint string = "https://api.radar.io/v1/geocode/ip"
)

type RadarClient struct {
	apiKey string
}

func NewRadarClient(apiKey string) *RadarClient {
	if apiKey == "" {
		err := eris.Errorf("apiKey cannot be empty")
		panic(err)
	}

	return &RadarClient{
		apiKey: apiKey,
	}
}

func (c *RadarClient) Geocode(query string) (*api.Coordinates, error) {
	if query == "" {
		err := eris.Errorf("query cannot be empty")
		return nil, err
	}

	params := url.Values{}
	params.Add("query", query)
	params.Add("limit", "1")

	endpoint, err := url.Parse(
		fmt.Sprintf("%s/?%s", baseGeocodeEndpoint, params.Encode()),
	)
	if err != nil {
		err = eris.Wrap(err, "failed to parse request url")
		return nil, err
	}

	res, err := http.Get(endpoint.String())
	if err != nil {
		err = eris.Wrap(err, "failed to make request")
		return nil, err
	}

	if res.StatusCode != http.StatusOK {
		bodyBytes, err := io.ReadAll(res.Body)
		if err != nil {
			err = eris.Wrapf(err, "response had status code %d and could not parse body", res.StatusCode)
			return nil, err
		}

		err = eris.Wrapf(err, "response had status %d and had body %s", res.StatusCode, string(bodyBytes))
		return nil, err
	}

	bodyBytes, err := io.ReadAll(res.Body)
	if err != nil {
		err = eris.Wrap(err, "failed to read request body")
		return nil, err
	}

	resJson := &GeocodeResponse{}
	if err := json.Unmarshal(bodyBytes, resJson); err != nil {
		err = eris.Wrap(err, "failed to unmarshal body")
		return nil, err
	}

	if len(resJson.Addresses) == 0 {
		err = eris.Wrap(err, "response.Addresses had 0 length")
		return nil, err
	}

	coords := &api.Coordinates{
		Latitude:  resJson.Addresses[0].Latitude,
		Longitude: resJson.Addresses[0].Longitude,
	}

	return coords, nil
}

func (c *RadarClient) GeocodeIp() (*api.Coordinates, error) {
	req, err := http.NewRequest(http.MethodGet, baseGeocodeIpEndpoint, nil)
	if err != nil {
		err = eris.Wrap(err, "failed to build request")
		return nil, err
	}

	req.Header.Add("Authorization", c.apiKey)

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		err = eris.Errorf("failed to make request")
		return nil, err
	}

	if res.StatusCode != http.StatusOK {
		bodyBytes, err := io.ReadAll(res.Body)
		if err != nil {
			err = eris.Errorf("response had status code %d and could not parse body", res.StatusCode)
			return nil, err
		}

		err = eris.Errorf("response had status %d and had body %s", res.StatusCode, string(bodyBytes))
		return nil, err
	}

	bodyBytes, err := io.ReadAll(res.Body)
	if err != nil {
		err = eris.Wrap(err, "failed to read request body")
		return nil, err
	}

	resJson := &GeocodeIpResponse{}
	if err := json.Unmarshal(bodyBytes, resJson); err != nil {
		err = eris.Wrap(err, "failed to unmarshal body")
		return nil, err
	}

	if resJson.Address == nil {
		err = eris.Errorf("response.Address was nil")
		return nil, err
	}

	coords := &api.Coordinates{
		Latitude:  resJson.Address.Latitude,
		Longitude: resJson.Address.Longitude,
	}

	return coords, nil
}

type GeocodeResponse struct {
	Addresses []*struct {
		Latitude  float64 `json:"latitude"`
		Longitude float64 `json:"longitude"`
	} `json:"addresses"`

	// TODO: much more, but eh?
}

type GeocodeIpResponse struct {
	Address *struct {
		Latitude  float64 `json:"latitude"`
		Longitude float64 `json:"longitude"`
	} `json:"address"`

	// TODO: much more, but eh?
}
