use std::error::Error;
use serde::Deserialize;
use chrono::{DateTime, FixedOffset, Local, Utc};

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
    pub forecast: Forecast
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub lat: f32,
    pub lon: f32,
    pub localtime_epoch: i64,
    pub localtime: String
}

#[derive(Debug, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>
}

#[derive(Debug, Deserialize)]
pub struct Forecastday {
    pub hour: Vec<Forecastdayhour>,
    pub date: String
}

#[derive(Debug, Deserialize)]
pub struct Forecastdayhour {
    pub time_epoch: i64,
    pub time: String,
    pub temp_c: f32,
    pub condition: Condition,
    pub wind_kph: f32,
    pub precip_mm: f32,
    pub chance_of_rain: i16

}

#[derive(Debug, Deserialize)]
pub struct Condition {
    pub text: String
}

#[derive(Deserialize, Debug)]
struct Weathers {
    weathers: Vec<Weather>
}

pub fn get_weather(request: &str) -> Result<Weather, NewsApiError> {
    let response = ureq::get(request)
        .call()
        .map_err( |e | NewsApiError::RequestFailed(e))?
        .into_string()
        .map_err( |e | NewsApiError::FailedResponseToString(e))?;

    let weather: Weather = serde_json::from_str(&response)
        .map_err( |e | NewsApiError::WeatherParseFailed(e))?;

    Ok(weather)
}
