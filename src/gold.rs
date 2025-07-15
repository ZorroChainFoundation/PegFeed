use chrono::Utc;
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldPrice {
    pub price: f64,
    pub source: String,
    pub timestamp: String,
}

/// Fetch from all available APIs and return results
pub async fn fetch_gold_price() -> Result<Vec<GoldPrice>, reqwest::Error> {
    let client = Client::new();
    let mut results = Vec::new();

    // 1. MetalpriceAPI (Free Tier)
    if let Ok(resp) = client
        .get("https://api.metalpriceapi.com/v1/latest?base=USD&symbols=XAU&apikey=demo")
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(price) = json["rates"]["XAU"].as_f64() {
                results.push(GoldPrice {
                    price,
                    source: "metalpriceapi".into(),
                    timestamp: Utc::now().to_rfc3339(),
                });
            }
        }
    }

    // 2. ExchangeRate-API (Fake proxy for XAU/USD)
    if let Ok(resp) = client
        .get("https://api.exchangerate-api.com/v4/latest/USD")
        .send()
        .await
    {
        if let Ok(json) = resp.json::<serde_json::Value>().await {
            if let Some(price) = json["rates"]["XAU"].as_f64() {
                results.push(GoldPrice {
                    price,
                    source: "exchangerate-api".into(),
                    timestamp: Utc::now().to_rfc3339(),
                });
            }
        }
    }

    Ok(results)
}
