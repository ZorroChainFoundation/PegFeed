use chrono::Utc;
use crate::region::Region;

pub struct EnergyMetrics {
    pub region: Region,
    pub oil_usd: f64,
    pub gas_usd: f64,
    pub source: String,
    pub timestamp: String,
}

pub async fn fetch_energy_data() -> Vec<EnergyMetrics> {
    vec![
        EnergyMetrics {
            region: Region::NorthAmerica,
            oil_usd: 82.5,
            gas_usd: 3.1,
            source: "EIA".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        EnergyMetrics {
            region: Region::EuropeanUnion,
            oil_usd: 85.3,
            gas_usd: 6.2,
            source: "ODRÉ".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        EnergyMetrics {
            region: Region::ChinaEastAsia,
            oil_usd: 81.0,
            gas_usd: 4.8,
            source: "IEA".into(),
            timestamp: Utc::now().to_rfc3339(),
        },
        // Add more regions as needed...
    ]
}
