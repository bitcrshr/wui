pub trait Geocoder {
    type Error;
    async fn geocode(query: &str) -> Result<(i64, i64), Self::Error>;

    /// Optional implementation, automatically will get the ip from the
    /// request and geocode it
    async fn geocode_ip() -> Option<Result<(i64, i64), Self::Error>> {
        None
    }
}

pub mod radar {
    use serde::Deserialize;
    use urlencoding::encode;

    pub struct RadarGeocoder {
        api_key: String,
    }
    impl RadarGeocoder {
        pub fn new(api_key: &str) -> Self {
            Self {
                api_key: String::from(api_key),
            }
        }
    }
    impl super::Geocoder for RadarGeocoder {
        type Error = reqwest::Error;

        async fn geocode(query: &str) -> Result<(i64, i64), Self::Error> {
            let url_base = "https://api.radar.io/v1/geocode/forward";
            let url = format!("{}?query={}", url_base, encode(query),);

            let res: RadarGeocodeResponse = reqwest::get(url).await?.json().await?;

            Ok((res.addresses[0].latitude, res.addresses[0].longitude))
        }

        async fn geocode_ip() -> Option<Result<(i64, i64), Self::Error>> {
            let url = "https://api.radar.io/v1/geocode/ip";

            let res: RadarGeocodeResponse = match reqwest::get(url).await {
                Ok(res) => match res.json::<RadarGeocodeResponse>().await {
                    Ok(res_json) => res_json,
                    Err(e) => return Some(Err(e)),
                },

                Err(e) => return Some(Err(e)),
            };

            Some(Ok((res.addresses[0].latitude, res.addresses[0].longitude)))
        }
    }

    #[derive(Debug, Clone, Deserialize)]
    struct RadarGeocodeResponse {
        pub addresses: Vec<RadarGeocodeAddress>,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    struct RadarGeocodeAddress {
        latitude: i64,
        longitude: i64,
    }
}
