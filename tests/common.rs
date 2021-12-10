/// Common functions used from any unit tests.

use std::error::Error;

extern crate web_server;
use web_server::*;

pub async fn start_server() {
    async_std::task::spawn(async move {
        match create_server() {
            Ok(_) => (),
            Err(e) => println!("Error during server creation: {}.", e),
        };
    });
}

pub async fn send_request(route: &str) -> Result<String, Box<dyn Error>>  {
    let url: &str = &format!("{}{}{}", config::PROTOCOL, config::HOST_PORT, route);
    let client = reqwest::Client::new();
    match client.get(url).send().await?.text().await {
        Ok(it) => Ok(it),
        Err(err) => Ok(err.to_string()),
    }
}

