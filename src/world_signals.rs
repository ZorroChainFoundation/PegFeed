use std::collections::HashMap;
use serde_json::Value;
use chrono::{DateTime, Utc};
use reqwest;
use log::warn;

pub struct WorldSignals;

impl WorldSignals {
    /// Fetch a factor value (e.g., gold, compute) from predefined endpoint with timestamp check.
    pub async fn fetch_factor_value(factor: &str) -> f64 {
        let endpoints: HashMap<&str, &str> = HashMap::from([
            ("gold", "https://pegfeed.io/gold"),
            ("energy", "https://pegfeed.io/energy"),
            ("compute", "https://pegfeed.io/gpu"),
            ("commodities", "https://pegfeed.io/commodities"),
            ("oil_coal", "https://pegfeed.io/oil"),
            ("nuclear", "https://pegfeed.io/nuclear"),
            ("export_index", "https://pegfeed.io/export"),
            ("geo_risk", "https://pegfeed.io/geopolitics"),
            ("system_redundancy", "https://pegfeed.io/redundancy")
        ]);

        if let Some(url) = endpoints.get(factor) {
            if let Ok(resp) = reqwest::get(*url).await {
                if let Ok(json) = resp.json::<Value>().await {
                    if let Some(price) = json.get("price_usd").and_then(|v| v.as_f64()) {
                        if let Some(ts) = json.get("timestamp").and_then(|v| v.as_str()) {
                            if WorldSignals::is_recent(ts) {
                                return price;
                            }
                        }
                    }
                }
            } else {
                warn!("⚠️ Could not fetch factor '{}'", factor);
            }
        }

        0.0 // fallback
    }

    fn is_recent(timestamp_str: &str) -> bool {
        if let Ok(parsed_time) = DateTime::parse_from_rfc3339(timestamp_str) {
            let now = Utc::now();
            let age = now.signed_duration_since(parsed_time.with_timezone(&Utc));
            return age.num_minutes() <= 30;
        }
        false
    }
}


// This file is part of ZorroChain Core.
// Copyright (c) 2025 ZorroChain Foundation
// Licensed under the Mozilla Public License, v. 2.0
// See LICENSE.md in the root for full license text.
