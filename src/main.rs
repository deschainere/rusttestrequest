use reqwest::blocking::Client;
use serde_json::json;
use std::time::Instant;

fn main() {
    let item_id = 1;
    let price = 1.1;

    let start = Instant::now();

    let data = json!({
        "ids": [item_id],
        "partner": "359688187",
        "token": "YAwTGWnG",
        "max_price": price,
        "custom_id": item_id
    });

    let client = Client::new();
    let res = client
        .post("https://api.lis-skins.com/v1/market/buy")
        .json(&data)
        .send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let json: serde_json::Value = response.json().unwrap();
                println!("Response: {:?}", json);
            } else {
                println!("Error: {}", response.status());
            }
        }
        Err(e) => println!("Request failed: {:?}", e),
    }

    println!("Buying finished for: {:?}", start.elapsed());
}