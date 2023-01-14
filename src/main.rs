mod theme;

use std::error::Error;

use chrono::{offset::TimeZone, DateTime, Datelike, Local, NaiveDateTime};

use dotenv::dotenv;

use newsapi::{get_weather, Weather};

fn render_weather(weather: &Weather) {
    let theme = theme::default();
    theme.print_text("# Prévisions métérologiques\n\n");
    theme.print_text(&format!("`{}` (`{}`, `{}`)", weather.location.name, weather.location.lat, weather.location.lon));
    theme.print_text(&format!("*{} {}*", weather.location.country, weather.location.localtime));
    theme.print_text("---");
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url: &str = 
        "http://api.weatherapi.com/v1/forecast.json?q=Braine-Alleud&lang=fr&days=1&key=";
;
    let url = format!("{}{}", url, api_key);

    let weather = get_weather(&url)?;

    render_weather(&weather);

    Ok(())
}