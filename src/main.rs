use pegfeed::peg_engine::PegFormula;
use pegfeed::vault::Vault;
use pqcrypto_dilithium::dilithium2;
use chrono::Utc;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use std::env;
use log::{info, warn, error};
use env_logger;

const SNAPSHOT_PATH: &str = "./vault_data";

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();
    info!("🚀 Starting Zorro PegFeed Oracle Node");
    info!("⛓ Vault Path: {}", SNAPSHOT_PATH);

    // Check for mock mode
    let mock_mode = env::var("ZOR_MOCK").is_ok();
    if mock_mode {
        warn!("🧪 Running in MOCK MODE (ZOR = 1.0)");
    }

    // Initialize vault and keypair (future: persist keypair if needed)
    let vault = Arc::new(Vault::new(SNAPSHOT_PATH));
    let (pk, sk) = dilithium2::keypair();

    // 60-second tick loop
    let mut ticker = interval(Duration::from_secs(60));

    loop {
        ticker.tick().await;
        let now = Utc::now().to_rfc3339();
        info!("🕒 Tick @ {}", now);

        match PegFormula::compute_and_snapshot(&vault, &sk, &pk).await {
            Some(signed) => {
                info!("✅ Snapshot stored: {}", signed.snapshot.snapshot_hash);

                // Future hook: broadcast snapshot
                // world_signals::broadcast(signed.clone()).await;

            },
            None => {
                error!("❌ Snapshot generation failed");
            }
        }
    }
}

// This file is part of ZorroChain Core.
// Copyright (c) 2025 ZorroChain Foundation
// Licensed under the Mozilla Public License, v. 2.0
// See LICENSE.md in the root for full license text.
