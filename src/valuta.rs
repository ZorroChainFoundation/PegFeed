use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::region::Region;
use std::collections::HashMap;
use reqwest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRate {
    pub base: String,
    pub target: String,
    pub rate: f64,
    pub region: Region,
    pub source: String,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
struct ExchangeApiResponse {
    base: String,
    rates: HashMap<String, f64>,
}

pub async fn fetch_valuta() -> Result<Vec<CurrencyRate>, Box<dyn std::error::Error + Send + Sync>> {
    let base = "USD";
    let target_region_map = vec![
        ("EUR", Region::EuropeanUnion),
        ("CNY", Region::ChinaEastAsia),
        ("INR", Region::SouthAsia),
        ("BRL", Region::LatinAmerica),
        ("AED", Region::MENA),
    ];

    let targets: Vec<&str> = target_region_map.iter().map(|(code, _)| *code).collect();

    let url = format!(
        "https://api.exchangerate.host/latest?base={}&symbols={}",
        base,
        targets.join(",")
    );

    let response = reqwest::get(&url).await?;
    let parsed = response.json::<ExchangeApiResponse>().await?;
    let timestamp = Utc::now().to_rfc3339();

    let rates: Vec<CurrencyRate> = target_region_map
        .into_iter()
        .filter_map(|(code, region)| {
            parsed.rates.get(code).map(|rate| CurrencyRate {
                base: base.to_string(),
                target: code.to_string(),
                rate: *rate,
                region,
                source: "exchangerate.host".into(),
                timestamp: timestamp.clone(),
            })
        })
        .collect();

    if rates.is_empty() {
        Err("❌ No valid exchange rates returned".into())
    } else {
        Ok(rates)
    }
}
