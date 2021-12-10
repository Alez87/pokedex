/// Main method.
///

extern crate web_server;
use web_server::create_server;

fn main() {
    match create_server() {
        Ok(_) => (),
        Err(e) => println!("Error during server creation: {}.", e),
    };
}
