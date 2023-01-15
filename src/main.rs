mod theme;

use std::error::Error;
use std::io;
use chrono::{offset::TimeZone, DateTime, Datelike, Local, NaiveDateTime};
use dotenv::dotenv;
use newsapi::{get_weather, Weather, Forecastday};
use termimad::{print_text, MadSkin};

pub fn request_weather(city: &str, days: i16) -> Result<Weather, Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url: &str = "http://api.weatherapi.com/v1/forecast.json?lang=fr&key=";

    let request = format!("{}{}&q={}&days={}", url, api_key, city, days);

    let weather = get_weather(&request)?;

    Ok(weather)
}

pub fn get_weathers(mut weathers: Vec<Weather>) -> Vec<Weather> {

    let weather = request_weather("Braine-Alleud", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Brussels", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Namur", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Waterloo(Belgium)", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Lillois", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Anvers", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Liege", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Ophain", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Arlon", 1).unwrap();
    weathers.push(weather);
    let weather = request_weather("Boendael", 1).unwrap();
    weathers.push(weather);

    weathers
}

fn render_weather(weather: &Weather, theme: &MadSkin) {
    theme.print_text(&format!("`{}` (`{}`, `{}`)", weather.location.name, weather.location.lat, weather.location.lon));
    theme.print_text(&format!("*{} {}*", weather.location.country, weather.location.localtime));
    for forecastday in &weather.forecast.forecastday {
        theme.print_text(&format!("`{}`", forecastday.date));
        for forecastdayhour in &forecastday.hour {
            theme.print_text(&format!("{}", forecastdayhour.time));
            theme.print_text(&format!("*Température : {}°C*", forecastdayhour.temp_c));
            theme.print_text(&format!("*Météo : {}*", forecastdayhour.condition.text));
            theme.print_text(&format!("*Vitesse du vent : {}km/h*", forecastdayhour.wind_kph));
            theme.print_text(&format!("*Pourcentage de précipitations : {}%*", forecastdayhour.chance_of_rain));
            theme.print_text(&format!("*Quantité de précipitations : {} mm*", forecastdayhour.precip_mm));
        }
        theme.print_text("---");
    }
}

fn render_weathers(weathers: &Vec<Weather>) {
    let theme = theme::default();
    theme.print_text("# Prévisions métérologiques\n\n");
    for weather in weathers {
        render_weather(weather, &theme);
    }
}

fn get_render_weather_know_city(days: i16) -> io::Result<()> {
    let theme = theme::default();
    let mut user_choice = String::new();

    while user_choice.trim() != "0" {
        user_choice.clear();
        theme.print_text(" ");
        theme.print_text("De quelle ville voulez-vous avoir les prévisions métérologiques ? (entrer le numéro de votre choix) : ");
        theme.print_text(" ");
        theme.print_text(&format!("*1. Braine-l'Alleud*"));
        theme.print_text(&format!("*2. Bruxelles*"));
        theme.print_text(&format!("*3. Namur*"));
        theme.print_text(&format!("*4. Waterloo*"));
        theme.print_text(&format!("*5. Lillois*"));
        theme.print_text(&format!("*6. Anvers*"));
        theme.print_text(&format!("*7. Liège*"));
        theme.print_text(&format!("*8. Ophain*"));
        theme.print_text(&format!("*9. Arlon*"));
        theme.print_text(&format!("*10. Boondael*"));
        theme.print_text(" ");
        theme.print_text(&format!("*0. Retour*"));
        theme.print_text(" ");

        let stdin = io::stdin();
        stdin.read_line(&mut user_choice);

        match user_choice.trim() {
            "1" => {
                let weather = request_weather("Braine-Alleud", days).unwrap();
                render_weather(&weather, &theme);
            }
            "2" => {
                let weather = request_weather("Brussels", days).unwrap();
                render_weather(&weather, &theme);
            }
            "3" => {
                let weather = request_weather("Namur", days).unwrap();
                render_weather(&weather, &theme);
            }
            "4" => {
                let weather = request_weather("Waterloo(Belgium)", days).unwrap();
                render_weather(&weather, &theme);
            }
            "5" => {
                let weather = request_weather("Lillois", days).unwrap();
                render_weather(&weather, &theme);
            }
            "6" => {
                let weather = request_weather("Anvers", days).unwrap();
                render_weather(&weather, &theme);
            }
            "7" => {
                let weather = request_weather("Liege", days).unwrap();
                render_weather(&weather, &theme);
            }
            "8" => {
                let weather = request_weather("Ophain", days).unwrap();
                render_weather(&weather, &theme);
            }
            "9" => {
                let weather = request_weather("Arlon", days).unwrap();
                render_weather(&weather, &theme);
            }
            "10" => {
                let weather = request_weather("Boendael", days).unwrap();
                render_weather(&weather, &theme);
            }
            "0" => theme.print_text("Retour"),
            _ => theme.print_text("Entrée incorrecte"),
        }
    }

    Ok(())
}

fn get_render_weather_unknow_city() -> io::Result<()> {
    let theme = theme::default();
    let mut user_choice = String::new();

    while user_choice.trim() != "Retour" {
        user_choice.clear();
        theme.print_text(" ");
        theme.print_text("De quelle ville voulez-vous avoir les prévisions métérologiques ?");
        theme.print_text("Entrer le nom de la ville de votre choix ou \"Retour\" pour retourner en arrière : ");
        theme.print_text(" ");

        let stdin = io::stdin();
        stdin.read_line(&mut user_choice);

        if user_choice != "Retour" {
            let weather = request_weather(user_choice.trim(), 1).unwrap();
            render_weather(&weather, &theme);
        }
    }

    Ok(())
}

fn user_operations() -> io::Result<()> {
    let theme = theme::default();
    let mut user_choice = String::new();

    while user_choice.trim() != "0" {
        user_choice.clear();
        theme.print_text(" ");
        theme.print_text("Que voulez-vous faire ? (entrer le numéro de votre choix) : ");
        theme.print_text(" ");
        theme.print_text(&format!("*1. Avoir les prévisions métérologiques d'une ville qui est dans la liste*"));
        theme.print_text(&format!("*2. Avoir les prévisions métérologiques d'une ville qui n'est pas dans la liste*"));
        theme.print_text(&format!("*3. Avoir les prévisions métérologiques de demain et d'après-demain d'une ville *"));
        theme.print_text(" ");
        theme.print_text(&format!("*0. Quitter le programme*"));
        theme.print_text(" ");

        let stdin = io::stdin();
        stdin.read_line(&mut user_choice);

        match user_choice.trim() {
            "0" => theme.print_text("Fermeture du programme"),
            "1" => {get_render_weather_know_city(1);}
            "2" => {get_render_weather_unknow_city();}
            "3" => {get_render_weather_know_city(3);}
            _ => theme.print_text("Entrée incorrecte"),
        }
    }

    Ok(())
}

fn main() {
    let mut weathers: Vec<Weather> = Vec::new();

    weathers = get_weathers(weathers);

    render_weathers(&weathers);

    user_operations();
}