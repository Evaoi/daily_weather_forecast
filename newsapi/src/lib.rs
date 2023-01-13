use std::error::Error;
use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum NewsApiError {
    #[error("Erreur de récupération des prévisions métérologiques")]
    RequestFailed(ureq::Error),
    #[error("Erreur de conversion en chaine de caractères")]
    FailedResponseToString(std::io::Error),
    #[error("Erreur de formatage des prévisons métérologiques")]
    WeatherParseFailed(serde_json::Error)
}

#[derive(Debug, Deserialize)]
pub struct Weather {
    pub location: Location,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    //lat: f32,
    //lon: f32,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String
}

#[derive(Deserialize, Debug)]
struct Weathers {
    weathers: Vec<Weather>
}

pub fn get_weather(url: &str) -> Result<Weather, NewsApiError> {
    let response = ureq::get(url)
        .call()
        .map_err( |e | NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err( |e | NewsApiError::FailedResponseToString(e))?;

    let weather: Weather = serde_json::from_str(&response)
        .map_err( |e | NewsApiError::WeatherParseFailed(e))?;

    Ok(weather)
}
