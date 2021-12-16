use std::env;
use std::fs;
use crate::Input;
use crate::reqwest;
use serde_json::{from_str};

/// Returns true if we are running on Apify platform
pub fn is_on_apify() -> bool {
    match env::var("APIFY_IS_AT_HOME") {
        Ok(ref x) if x == "1"  => true,
        _ => false
    }
}

/// Get HTML string from a URL
pub fn http_request_get(url: &str) -> String {
    reqwest::blocking::get(url)
        .expect("Didn't get response")
        .text()
        .expect("Response doesn't contain HTML")
}

/// Put HTML string as a payload into a URL
fn http_request_put(url: &str, payload: String) {
    // Put request requires building a HTTP client variable
    let client = reqwest::blocking::Client::new();
    client.put(url)
        .header(reqwest::header::CONTENT_TYPE, "text/html")
        .body(payload)
        .send()
        .expect("Put request failed");
}

/// Get record from Key Value Store where INPUT is stored
pub fn get_input () -> Input {
    let is_on_apify = is_on_apify();

    let json = if is_on_apify {
        let default_kv = env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID").unwrap();
        let input_url = format!("https://api.apify.com/v2/key-value-stores/{}/records/INPUT", default_kv);
        let val = http_request_get(&input_url);
        // println!("Loaded value from KV -> {}", val);
        val
    } else {
        // Standard path where local INPUT is stored
        fs::read_to_string("apify_storage/key_value_stores/default/INPUT.json")
            .expect("INPUT.json was not found on your local path apify_storage/key_value_stores/default/INPUT.json")
    };

    // Convert JSON string into Input and return
    from_str(&json).expect("INPUT.json is not a valid JSON")
}

/// Put HTML as OUTPUT record to Key Value Store
pub fn set_output (html: String) {
    let is_on_apify = is_on_apify();

   if is_on_apify {
        let default_kv = env::var("APIFY_DEFAULT_KEY_VALUE_STORE_ID").unwrap();
        let token = env::var("APIFY_TOKEN").unwrap();
        let output_url = format!("https://api.apify.com/v2/key-value-stores/{}/records/OUTPUT?token={}", default_kv, token);
        http_request_put(&output_url, html);
    } else {
        // Standard path where local INPUT is stored
        fs::write("apify_storage/key_value_stores/default/OUTPUT.html", html)
            .expect("OUTPUT.json cannot be saved to your local path apify_storage/key_value_stores/default/OUTPUT.html")
    };
}