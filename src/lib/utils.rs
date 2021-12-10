/// Utils methods to for common tasks of the library.

use std::collections::HashMap;
use serde_json::Value;
use std::error::Error;
use std::str;

use crate::config;

/// Read input parameter (es. name)
pub fn get_input_parameter<'a>(buffer: &'a[u8; 1024], endpoint_name: &str) -> Result<&'a str, Box<dyn Error>> {
    let buffer_to_str = match str::from_utf8(buffer) {
        Ok(s) => s,
        Err(_) => {
            println!("Cannot read the buffer of the request."); 
            return Err("Cannot read buffer of the request.".into())
        }
    };
    let split_buffer: Vec<&str> = buffer_to_str.split(' ').collect();
    let params: Vec<&str> = split_buffer[1].split(endpoint_name).collect();
    let param = params[1];
    match param.chars().all(char::is_alphanumeric) {
        true => Ok(param),
        false => {
            println!("The parameter is not in a valid format."); 
            Err("The parameter is not in a valid format.".into())
        }
    }
}

/// Convert a string to HashMap
pub fn str_to_map(response: String) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let map: HashMap<String, Value> = match &response.is_empty() {
        true => {
            println!("The parameter is not in a valid format."); 
            return Err("The parameter is not in a valid format.".into())
        }
        false =>
            serde_json::from_str(&response)?
    };
    Ok(map)
}

/// Extract a value from an hashMap
pub fn get_value_from_map(json_pokemon_map: &HashMap<String, Value>, key: &str) -> Result<String, Box<dyn Error>> {
    let value: String = match json_pokemon_map.get(key){
        Some(s) => {
            if s.is_array(){
                s[0].to_string()
            } else {
                s.to_string()
            }
        },
        None => {
            println!("Not able to read the key \"{}\".", key);
            if key == "error" {
                String::new()
            } else {
                return Err(format!("Not able to read the key {}.", key).into())
            }
        }
    }; 
    Ok(value)
}

pub async fn send_endpoint_request(remote_endpoint: String, description: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response: String = match description.is_empty(){
        true => {
            client.get(remote_endpoint).send().await?.text().await?
        }
        false => {
            let params = [("text", description)];
            let url = reqwest::Url::parse_with_params(&remote_endpoint, &params)?;
            client.get(url).form(&params).send().await?.text().await?
        }
    };
    let map_translator: HashMap<String, Value> = match str_to_map(response) {
        Ok(s) => s,
        Err(_) => {
            println!("Cannot read the response, not a json format."); 
            return Err("Cannot read the response, not a json format.".into());
        }
    };
    Ok(map_translator)
}

pub async fn translate(legendary_status: &str, habitat: &str, description: &str) -> Result<String, Box<dyn Error>> {
    let translator_endpoint: String;
    if legendary_status == "true" || habitat == "cave" {
        translator_endpoint = format!("{}{}", config::TRANSLATOR_BASE_ENDPOINT, config::YODA_TRANSLATOR);
    } else {
        translator_endpoint = format!("{}{}", config::TRANSLATOR_BASE_ENDPOINT, config::SHAKESPEARE_TRANSLATOR);
    }
    let map_translator = send_endpoint_request(translator_endpoint, description).await?;
    
    let map_error: String = get_value_from_map(&map_translator, "error")?;
    
    let translation: String;
    if map_error.is_empty() {
        let map_success: String = get_value_from_map(&map_translator, "contents")?;
        let map_success_msg: HashMap<String, Value> = str_to_map(map_success)?;
        translation = get_value_from_map(&map_success_msg, "translated")?;
    } else {
        let map_error_msg: HashMap<String, Value> = str_to_map(map_error)?;
        translation = format!("**Error from translation endpoint**: {}", get_value_from_map(&map_error_msg, "message")?);
    }
    Ok(translation.replace("\\\"", ""))
}