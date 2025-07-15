<!--
ZorroChain Core Protocol  
Copyright (c) 2025–present ZorroChain Foundation  
Licensed under the Mozilla Public License, v. 2.0  
https://mozilla.org/MPL/2.0/
-->

# 📊 PegFeed Engine — Economic Index & Sync Pricing Layer

The `pegfeed/` module anchors ZorroChain’s economic activity to a dynamic, decentralized value feed. It ingests regional signals, validator zone weightings, and oracle data to calculate sync fees, staking thresholds, and contract price logic — all entropy-aware and epoch-bound.

---

## 🧭 Purpose

- Establish cross-region, multi-signal economic consensus  
- Peg sync fees, staking logic, and contract costs to off-chain values  
- Detect and neutralize volatility, oracle spoofing, or signal tampering  
- Enable humanitarian pricing curves and civic cost stabilization

---

## 🔹 Components

### `src/logic.rs`
- Main peg engine  
- Aggregates oracles, civic weights, entropy filters  
- Applies EpochCache, volatility guards, and hybrid peg rules

### `src/types.rs`
- Core types: `PegIndex`, `RegionWeight`, `OracleProof`, `EpochCache`

### `src/tests/`
- Simulation suite for volatility management, regional index drift, and rollback safety

### `pegfeed-api/`
- Handles civic-sourced oracle injection and zone-based index sourcing  
- Signature validation for external economic signals

---

## 📝 Use Cases

- Wallet sync fee pegged to solar energy cost or fiat+entropy blend  
- Contract reward scaling based on energy or bandwidth pegging  
- Vault scoring based on index-linked asset value decay  
- Humanitarian zones with cost ceilings based on civic entropy inputs

---

## 📐 Algorithms / Primitives

- `EntropyWeightedMedian` – Resilient peg average under civic/region filtering  
- `OracleProof` – Civic-signed, entropy-filtered oracle stream hash  
- `EpochCache` – Cached peg index with rollback-safe recovery  
- `RegionAnchorMap` – Geospatial sync bias for peg validation  
- `HybridPegMerge` – ZUSD ↔ solar ↔ storage weighted combination model

---

## 🔗 Inter-module Dependencies

**Upstream:**  
- `pegfeed-api/`: Real-time signals and civic mesh feedback  
- `network/sync/`: Sync injection point and region handshake

**Downstream:**  
- `wallet/`: Calculates transaction fees and staking bounds  
- `vault/`: Classifies asset scores for entropy resilience  
- `contracts/`: Enforces peg-tied logic or inflation limits  
- `dao/`: May use PegFeed weight during resource budgeting proposals

---

## 🚨 Known Issues / Edge Cases

- `oracle-drift`: Oracles lag across zones → consensus mismatch  
- `region-mismatch`: Local mesh diverges from majority peg  
- `volatility-storm`: Peg blend components (e.g. ZUSD ↔ solar) misalign  
- `cache-desync`: Peg cache too stale → rollback loop in mirror replay

---

## ⌛ Recent Updates

- 2025-06-08: Added HybridPegMerge structure and entropy volatility guard  
- 2025-06-08: `pegfeed-api/` updated to support civic-signed region weights  
- 2025-06-08: README brought into canonical format

---

> _"Price is not just signal — it’s consensus wrapped in entropy."_
