extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;
use std::env;

use hyper::Client;
use hyper::header::Connection;

use rustc_serialize::json::{Json};

const API_URL: &'static str = "http://api.openweathermap.org/data/2.5/weather?q=";

fn main() {
    match env::args().nth(1) {
        Some(city) => {
            let (weather, region) = get_weather(city);
            println!("Current weather in {} is {}.", region, weather);
        }
        None => {
            println!("Usage: weather <city>");
            return;
        }
    };
}

fn get_weather(city: String) -> (String, String) {
    let url = API_URL.to_string() + &city;
    let client = Client::new();

    let mut res = client.get(&url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let json_body = Json::from_str(&body).unwrap();
    let json_object = json_body.as_object().unwrap();
    let weather = json_object.get("weather").unwrap();

    (weather[0]
        .as_object().unwrap()
        .get("description").unwrap()
        .to_string(), city)
}
