extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json::{Json};

use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

fn main() {
    let client = Client::new();

    let mut res = client.get("http://api.openweathermap.org/data/2.5/weather?q=Paris, France")
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let json_body = Json::from_str(&body).unwrap();
    let json_object = json_body.as_object().unwrap();
    let weather = json_object.get("weather").unwrap();

    println!("{:?}",
             weather[0]
             .as_object().unwrap()
             .get("description").unwrap()
             .as_string().unwrap());
}
