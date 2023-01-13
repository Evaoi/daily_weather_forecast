mod theme;

use std::error::Error;

use dotenv::dotenv;

use newsapi::{get_weather, Weather};

fn render_weather(weather: &Weather) {
    let theme = theme::default();
    theme.print_text("# Top en-têtes\n\n");
    theme.print_text(&format!("`{}`", weather.location.name));
    theme.print_text(&format!(">*{}*", weather.location.country));
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