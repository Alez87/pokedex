/// Library to create a web_server and manage the incoming request.

mod utils;
pub mod config;

use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use std::net::TcpListener;

/// Create a listener and start an async task for any incoming request.
pub fn create_server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(config::HOST_PORT)?;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        async_std::task::spawn(async {
            match handle_connection(stream).await {
                Ok(_) => (),
                Err(_) => println!("Error during connection."),
            };
        });
    }
    Ok(())
}

/// Handle the incoming request.
async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    stream.read_exact(&mut buffer)?;
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let endpoint1: &str = &format!("{} {}", config::ENDPOINT_GET_METHOD, config::ENDPOINT1_NAME);
    let endpoint2: &str = &format!("{} {}", config::ENDPOINT_GET_METHOD, config::ENDPOINT2_NAME);

    let (status_line, filename, result) = 
        if buffer.starts_with(endpoint2.as_bytes()) {
            let param = utils::get_input_parameter(&buffer, config::ENDPOINT2_NAME)?;
            let (name, description, habitat, is_legendary) = endpoint_process(param).await?;
            let translation: String = utils::translate(&is_legendary, &habitat, &description).await?;
            let result: String = format!("{{name: {}, description: {}, habitat: {}, is_legendary: {}}}", name, translation, habitat, is_legendary);
            ("HTTP/1.1 200 OK", config::OK_PAGE, result)
        } else if buffer.starts_with(endpoint1.as_bytes()) {
            let param = utils::get_input_parameter(&buffer, config::ENDPOINT1_NAME)?;
            let (name, description, habitat, is_legendary) = endpoint_process(param).await?;
            let result = format!("{{name: {}, description: {}, habitat: {}, is_legendary: {}}}", name, description, habitat, is_legendary);
            ("HTTP/1.1 200 OK", config::OK_PAGE, result)
        }
        else {
            ("HTTP/1.1 404 NOT FOUND", config::NOTFOUND_PAGE, "".to_string())
        };

    let contents: String = match result.is_empty(){
        true => fs::read_to_string(filename)?,
        false => result,
    };
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

/// Retrieve the information, given a name.
async fn endpoint_process(name: &str) -> Result<(&str, String, String, String), Box<dyn Error>> {
    let remote_endpoint_id = format!("{}pokemon/{}", config::POKEMON_BASE_ENDPOINT, name);
    println!("Endpoint: {}", remote_endpoint_id);
    let pokemon_map: HashMap<String, Value> = utils::send_endpoint_request(remote_endpoint_id, "").await?;
    let id: String = utils::get_value_from_map(&pokemon_map, "id")?;
    println!("{{Name: {}, Id: {}}}", name, id);

    let remote_endpoint_species: String = format!("{}pokemon-species/{}", config::POKEMON_BASE_ENDPOINT, id);
    let map_pokemon_species = utils::send_endpoint_request(remote_endpoint_species, "").await?;
    
    let legendary_status: String = utils::get_value_from_map(&map_pokemon_species, "is_legendary")?;
    
    let habitat: String = utils::get_value_from_map(&map_pokemon_species, "habitat")?;
    let map_habitat = utils::str_to_map(habitat)?;
    let habitat: String = utils::get_value_from_map(&map_habitat, "name")?;
    
    let flavor_text: String = utils::get_value_from_map(&map_pokemon_species, "flavor_text_entries")?;
    let map_flavor_text = utils::str_to_map(flavor_text)?;
    let standard_description: String = utils::get_value_from_map(&map_flavor_text, "flavor_text")?.replace("\\n", " ").replace("\\f", " ");
    
    Ok((name, standard_description, habitat, legendary_status))
}
