use crate::region::Region;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct LaborIndex {
    pub region: Region,
    pub validator_count: u32,
    pub commit_count: u32,
    pub index_value: f64,
    pub timestamp: String,
    pub source: String,
}

/// Basic synthetic index calculation for validator activity
pub fn calculate_labor_index(region: Region, validators: u32, commits: u32, source: &str) -> LaborIndex {
    let index_value = (validators as f64 * 0.6) + (commits as f64 * 0.4);

    LaborIndex {
        region,
        validator_count: validators,
        commit_count: commits,
        index_value,
        timestamp: Utc::now().to_rfc3339(),
        source: source.into(),
    }
}

/// Example usage / sample metrics (remove or wrap in a test if unused)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn print_sample_labor_metrics() {
        let labor_metrics = vec![
            calculate_labor_index(Region::NorthAmerica, 1200, 3500, "Zorro Validator Logs"),
            calculate_labor_index(Region::EuropeanUnion, 980, 2700, "Zorro Validator Logs"),
            calculate_labor_index(Region::SouthAsia, 1500, 4200, "Zorro Validator Logs"),
        ];

        for metric in labor_metrics {
            println!("{:?}", metric);
        }
    }
}
