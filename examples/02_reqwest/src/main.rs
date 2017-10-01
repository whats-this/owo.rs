extern crate owo;
extern crate reqwest;

use owo::OwoReqwestRequester;
use reqwest::Client;
use std::fs::File;
use std::io::{self, Read, Write};
use std::env;

// You would want to handle results in a normal program.
fn main() {
    // Retrieve the token from an environment variable named "OWO_TOKEN".
    let token = env::var("OWO_TOKEN")
        .expect("Must provide OWO_TOKEN as an environment variable");

    // Create the reqwest Client.
    let client = Client::new();

    // Read a filepath to upload from user input:
    let mut input = String::new();
    print!("Enter the path on the filesystem to upload:\n>");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let input_trimmed = input.trim();

    // Read the content of the file to a buffer.
    let mut file = File::open(input_trimmed).expect("Error opening file");
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).expect("Error reading file");

    let response = client.upload_file(&token, buffer);

    println!("Response: {:?}", response);
}
