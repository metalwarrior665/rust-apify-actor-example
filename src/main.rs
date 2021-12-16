use reqwest;
use serde::Deserialize;

mod helpers;

use helpers::{http_request_get, get_input, set_output};

// Rust is a typed language so we have to deserialize from JSON string to a type
#[derive(Deserialize)]
pub struct Input {
    url: String
}

// This code works both locally and on Apify platform
fn main () {
    // Get input that contains URL
    let input = get_input();
    println!("Loaded input with URL: {}", &input.url);
    // Doing a synchronous HTTP call for simplicity
    // For production code, use async/await instead
    // and handle error gracefully
    let html = http_request_get(&input.url);
    println!("Got HTML from URL: {}", &input.url);

    // Save output HTML
    set_output(html);
    println!("Saved OUTPUT.html. Actor finished");
}