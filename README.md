# Example Apify actor in Rust

Minimal example for developing Apify actor in [Rust programming language](https://www.rust-lang.org/). The code is [open source](https://github.com/metalwarrior665/rust-apify-actor-example).

The example:
- Reads user provided URL from input
- Does HTTP request to get HTML from this URL
- Saves HTML into OUTPUT record in Key Value Store

This actor works both on Apify platform and locally.

## Local usage
For local usage, you need to: 
1. Clone the [repository](https://github.com/metalwarrior665/rust-apify-actor-example)
2. [Install Rust](https://www.rust-lang.org/tools/install)
3. Create local storage with `apify init` or manually create folder path `apify_storage/key_value_stores/default/`
4. Add `INPUT.json` file input inside this path that looks like this: 
```json
{
    "url": "https://apify.com"
}
```
5. Build and run the solution with `cargo run`
6. You can find `OUTPUT.html` next to `INPUT.json`

## Current problems
- Apify currently doesn't allow increasing memory for builds so they sometimes fails by running out of memory
- Apify caches the Docker images on its servers. Rust is not yet used much there so a lot of runs have slow cold starts.

## Where next?
If you want to see more complex scraping project in Rust, check [Rust Scraper](https://apify.com/lukaskrivka/rust-scraper) which is also [open source](https://github.com/metalwarrior665/actor-rust-scraper)
