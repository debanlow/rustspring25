use std::thread;
use std::time::Duration;
use serde_json::Value;
use ureq;

fn fetch_bitcoin_price() -> Result<f64, String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    let json: Value = response.into_json().map_err(|e| e.to_string())?;
    json["bitcoin"]["usd"]
        .as_f64()
        .ok_or("Failed to parse Bitcoin price".to_string())
}

fn fetch_ethereum_price() -> Result<f64, String> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    let json: Value = response.into_json().map_err(|e| e.to_string())?;
    json["ethereum"]["usd"]
        .as_f64()
        .ok_or("Failed to parse Ethereum price".to_string())
}

fn fetch_sp500_price() -> Result<f64, String> {
    let url = "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=SPY&apikey=126XD11U03PZWS66";
    let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    let json: Value = response.into_json().map_err(|e| e.to_string())?;

    let time_series = json.get("Time Series (Daily)").ok_or("Missing time series")?;
    let latest_entry = time_series.as_object()
        .and_then(|map| map.iter().next())
        .ok_or("No time series data")?;

    let close_price = latest_entry.1.get("4. close")
        .and_then(|v| v.as_str())
        .ok_or("Missing close price")?
        .parse::<f64>()
        .map_err(|_| "Failed to parse close price".to_string())?;

    Ok(close_price)
}

fn main() {
    loop {
        match fetch_bitcoin_price() {
            Ok(price) => println!("Bitcoin: ${:.2}", price),
            Err(e) => println!("Failed to fetch Bitcoin price: {}", e),
        }

        match fetch_ethereum_price() {
            Ok(price) => println!("Ethereum: ${:.2}", price),
            Err(e) => println!("Failed to fetch Ethereum price: {}", e),
        }

        match fetch_sp500_price() {
            Ok(price) => println!("S&P 500 (SPY): ${:.2}", price),
            Err(e) => println!("Failed to fetch S&P 500 price: {}", e),
        }

        thread::sleep(Duration::from_secs(30));
    }
}
