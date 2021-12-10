/// Unit tests for the web_server library.
///
/// The tests are divided per endpoint:
/// - general test
/// - test on endpoint1
/// - test on endpoint2 
///

mod common;
extern crate web_server;
use web_server::config;
use std::{thread, time::Duration};

#[cfg(test)]
mod tests_general {
    use std::fs;
    use super::*;
 
    #[async_std::test]
    async fn get_pokemon_wrong_route() {

        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));

        let route: &str = "/test";
        let response: String = match common::send_request(route).await {
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };

        let filename = "html_pages/notfound.html";
        let expected: String = fs::read_to_string(filename).unwrap();
        assert_eq!(expected, response);
    }
}

#[cfg(test)]
mod tests_endpoint1 {
    use super::*;

    #[async_std::test]
    async fn get_pokemon_empty_name() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));

        let route: &str = "/pokemon/";
        let response: String = match common::send_request(route).await {
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: String = format!("error sending request for url ({}{}{}): connection closed before message completed", config::PROTOCOL, config::HOST_PORT, route);
        assert_eq!(expected, response);
    }

    #[async_std::test]
    async fn get_pokemon_strange_name() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));

        let route: &str = "/pokemon/strange_name";
        let response: String = match common::send_request(route).await {
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: String = format!("error sending request for url ({}{}{}): connection closed before message completed", config::PROTOCOL, config::HOST_PORT, route);
        assert_eq!(expected, response);
    }

   #[async_std::test]
    async fn get_pokemon_not_legendary() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));
        let route: &str = "/pokemon/ditto";
        let response: String = common::send_request(route).await.unwrap();
        let expected: &str = "{name: ditto, description: \"Capable of copying an enemy's genetic code to instantly transform itself into a duplicate of the enemy.\", habitat: \"urban\", is_legendary: false}";
        assert_eq!(expected, response);
    }

    #[async_std::test]
    async fn get_pokemon_legendary() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));
        let route: &str = "/pokemon/mewtwo";
        let response: String = common::send_request(route).await.unwrap();
        let expected: &str = "{name: mewtwo, description: \"It was created by a scientist after years of horrific gene splicing and DNA engineering experiments.\", habitat: \"rare\", is_legendary: true}";
        assert_eq!(expected, response);
    }
}

mod tests_endpoint2 {
    use super::*;

    #[async_std::test]
    async fn get_pokemon_empty_name() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));

        let route: &str = "/pokemon/translated";
        let response: String = match common::send_request(route).await{
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: String = format!("error sending request for url ({}{}{}): connection closed before message completed", config::PROTOCOL, config::HOST_PORT, route);
        assert_eq!(expected, response);
    }

    #[async_std::test]
    async fn get_pokemon_strange_name() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));

        let route: &str = "/pokemon/translated/strange_name";
        let response: String = match common::send_request(route).await{
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: String = format!("error sending request for url ({}{}{}): connection closed before message completed", config::PROTOCOL, config::HOST_PORT, route);
        assert_eq!(expected, response);
    }

    #[async_std::test]
    async fn get_pokemon_shakespeare_translation() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));
        let route: &str = "/pokemon/translated/ditto";
        let response: String = match common::send_request(route).await {
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: &str = "{name: ditto, description: \"capable of copying an foe's genetic code to instantly transform itself into a duplicate of the foe.\", habitat: \"urban\", is_legendary: false}";
        assert_eq!(expected, response);
    }

    #[async_std::test]
    async fn get_pokemon_yoda_translation() {
        common::start_server().await;
        thread::sleep(Duration::from_millis(1000));
        let route: &str = "/pokemon/translated/mewtwo";
        let response: String = match common::send_request(route).await {
            Ok(s) => s,
            Err(err) =>  err.to_string(),
        };
        let expected: &str = "{name: mewtwo, description: \"Created by a scientist after years of horrific gene splicing and dna engineering experiments,  it was.\", habitat: \"rare\", is_legendary: true}";
        assert_eq!(expected, response);
    }
}
