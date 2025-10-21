use reqwest;
use serde_json::Value;
use tokio::{join,select};

/// Fetches the USD price of a cryptocurrency from CoinGecko.
async fn get_usd(name: &str) -> String {
    // Build API URL
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
        name
    );

    // Try to fetch data, return error string if any step fails
    let res = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(_) => return format!("Failed to fetch {name}"),
    };

    let text = match res.text().await {
        Ok(t) => t,
        Err(_) => return format!("Failed to fetch {name}"),
    };

    let data: Value = match serde_json::from_str(&text) {
        Ok(d) => d,
        Err(_) => return format!("Failed to parse {name}"),
    };

    // Extract price as f64, return formatted string or fallback
    match data[name]["usd"].as_f64() {
        Some(price) => format!("${:.2}", price),
        None => format!("Price not found for {name}"),
    }
}

#[tokio::main]
async fn main() {
    // let (eth_price, btc_price) = join!(get_usd("ethereum"), get_usd("bitcoin"));
    // println!("Ethereum: {}", eth_price);
    // println!("Bitcoin: {}", btc_price);
    let result = select!{
        val=get_usd("ethereum") =>{println!("ETH:{val}");
            val},
        val=get_usd("bitcoin") =>{println!("BTC:{val}");
            val},
    };
    println!("{result}");
}
