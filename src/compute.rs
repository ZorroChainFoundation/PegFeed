use crate::region::Region;

pub struct ComputeCost {
    pub region: Region,
    pub cloud_avg_usd: f64,  // e.g., per vCPU-hour
    pub grid_cost_kw: f64,   // per kilowatt-hour
    pub source: String,
    pub timestamp: String,
}

use chrono::Utc;

pub async fn fetch_compute_cost() -> Vec<ComputeCost> {
    vec![
        ComputeCost {
            region: Region::NorthAmerica,
            cloud_avg_usd: 0.023,
            grid_cost_kw: 0.14,
            source: "Azure NA Estimate".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        ComputeCost {
            region: Region::EuropeanUnion,
            cloud_avg_usd: 0.025,
            grid_cost_kw: 0.28,
            source: "AWS EU Public Data".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        ComputeCost {
            region: Region::ChinaEastAsia,
            cloud_avg_usd: 0.018,
            grid_cost_kw: 0.09,
            source: "Alibaba Cloud Public".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        ComputeCost {
            region: Region::SouthAsia,
            cloud_avg_usd: 0.022,
            grid_cost_kw: 0.11,
            source: "GCP India Pricing".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        // Add more regions...
    ]
}
