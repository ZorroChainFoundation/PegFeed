use pqcrypto_traits::sign::{DetachedSignature, PublicKey as TraitPublicKey};
use crate::{gold, energy, compute, labor, valuta};
use crate::region::Region;
use crate::snapshot_chain::{PegSnapshot, PegSnapshotSigned};
use crate::vault::Vault;

use chrono::Utc;
use sha2::{Sha256, Digest};
use pqcrypto_dilithium::dilithium2::{self, SecretKey, PublicKey};
use serde_json::{self, json};
use hex;
use std::env;
use log::{info, warn, error};

pub struct PegFormula;

impl PegFormula {
    pub async fn compute_and_snapshot(
        vault: &Vault,
        sk: &SecretKey,
        pk: &PublicKey
    ) -> Option<PegSnapshotSigned> {
        // Toggle mock mode via env var
        let is_mock_mode = env::var("ZOR_MOCK").is_ok();

        // 1. Collect Feed Data (with logging)
        let gold_val = match gold::fetch_gold_price().await {
            Ok(data) if !data.is_empty() => {
                let avg = data.iter().map(|p| p.price).sum::<f64>() / data.len() as f64;
                info!("🟡 Gold Avg: {}", avg);
                avg
            },
            _ => {
                warn!("⚠️ Gold feed unavailable. Using default 0.0");
                0.0
            }
        };

        let energy_val = match energy::fetch_energy_data().await {
            Ok(data) if !data.is_empty() => {
                let avg = data.iter().map(|e| e.oil_usd).sum::<f64>() / data.len() as f64;
                info!("🟠 Energy Avg: {}", avg);
                avg
            },
            _ => {
                warn!("⚠️ Energy feed unavailable. Using default 0.0");
                0.0
            }
        };

        let compute_val = match compute::fetch_compute_cost().await {
            Ok(data) if !data.is_empty() => {
                let avg = data.iter().map(|c| c.cloud_avg_usd).sum::<f64>() / data.len() as f64;
                info!("🔵 Compute Avg: {}", avg);
                avg
            },
            _ => {
                warn!("⚠️ Compute feed unavailable. Using default 0.0");
                0.0
            }
        };

        let labor_val = {
            let index = labor::calculate_labor_index(Region::Global, 1000, 3000, "ZorroChain Est.");
            info!("🧑‍🏭 Labor Index: {}", index.index_value);
            index.index_value
        };

        let valuta_val = match valuta::fetch_valuta().await {
            Ok(v) => {
                let usd_rates: Vec<f64> = v.into_iter()
                    .filter(|r| r.base == "USD")
                    .map(|r| r.rate)
                    .collect();
                let avg = usd_rates.iter().sum::<f64>() / usd_rates.len().max(1) as f64;
                info!("💱 Valuta USD Avg: {}", avg);
                avg
            },
            Err(e) => {
                warn!("⚠️ Valuta feed failed: {e}. Using default 1.0");
                1.0
            }
        };

        // 2. Compute or Mock ZOR Value
        let zor = if is_mock_mode {
            info!("🧪 MOCK MODE ENABLED: ZOR = 1.0");
            1.0
        } else {
            let value = (gold_val * 0.3)
                + (energy_val * 0.2)
                + (compute_val * 0.15)
                + (labor_val * 0.2)
                + (valuta_val * 0.15);
            info!("🧮 ZOR Value: {}", value);
            value
        };

        // 3. Snapshot Construction + Hash
        let snapshot_input = json!({
            "gold": gold_val,
            "energy": energy_val,
            "compute": compute_val,
            "labor": labor_val,
            "valuta": valuta_val,
            "zor": zor,
            "mode": if is_mock_mode { "mock" } else { "live" }
        });

        let input_json = snapshot_input.to_string();
        let input_hash = Sha256::digest(input_json.as_bytes());
        let input_hash_hex = hex::encode(&input_hash);

        let snapshot = PegSnapshot {
            timestamp: Utc::now().to_rfc3339(),
            input_hash: input_hash_hex.clone(),
            snapshot_hash: hex::encode(Sha256::digest(input_hash_hex.as_bytes())),
            zor_value: zor,
        };

        // 4. Sign Snapshot
        let snapshot_bytes = match serde_json::to_vec(&snapshot) {
            Ok(b) => b,
            Err(e) => {
                error!("❌ Failed to serialize snapshot: {e}");
                return None;
            }
        };

        let sig = dilithium2::detached_sign(&snapshot_bytes, sk);
        let signed = PegSnapshotSigned {
            snapshot: snapshot.clone(),
            signature: hex::encode(sig.as_bytes()),
            public_key: hex::encode(pk.as_bytes()),
        };

        // 5. Vault Submission
        match vault.submit_snapshot(snapshot) {
            Ok(_) => {
                info!("✅ Snapshot submitted to vault.");
                Some(signed)
            },
            Err(e) => {
                error!("❌ Vault submission failed: {e}");
                None
            }
        }
    }
}

// This file is part of ZorroChain Core.
// Copyright (c) 2025 ZorroChain Foundation
// Licensed under the Mozilla Public License, v. 2.0
// See LICENSE.md in the root for full license text.
