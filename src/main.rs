use std::error::Error;

use serde::{Deserialize};
//use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Weather {
    location: Location,
}

#[derive(Debug, Deserialize)]
struct Location {
    name: String,
    country: String,
    //lat: f32,
    //lon: f32,
    //tz_id: String,
    //localtime_epoch: i64,
    localtime: String
}

#[derive(Deserialize, Debug)]
struct Weathers {
    weathers: Vec<Weather>
}

fn get_weather(url: &str) -> Result<Weather, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let weather: Weather = serde_json::from_str(&response)?;

    Ok(weather)
}
fn main() {
    let url: &str = "http://api.weatherapi.com/v1/forecast.json?key=bd238e9b8e7944358d6190707231201&q=Braine-Alleud&lang=fr&days=1";
    let weather = get_weather(url);

    dbg!(weather);
}