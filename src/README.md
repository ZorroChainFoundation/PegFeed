<!--
@defgroup PegFeedModule Pegged Economic Feed System
@ingroup ZorroChainCodex
Handles deterministic pricing via multi-input oracle synthesis and entropy-anchored snapshot feeds.
-->

<!--
ZorroChain Core Protocol  
Copyright (c) 2025–present ZorroChain Foundation  
Licensed under the Mozilla Public License, v. 2.0  
https://mozilla.org/MPL/2.0/
-->

# ⚖️ Pegged Economic Feed (`pegfeed/`)

The `pegfeed/` module delivers **deterministic economic signals** through weighted oracle consensus, region-aware valuation models, and snapshot-chained price feeds. Designed to function under **partial network isolation**, it ensures the ZorroChain economy remains consistent, accountable, and entropy-anchored even across mesh or orbital fallback topologies.

> “In the silence of markets, the chain must still speak value.”

---

## 🧭 Purpose

- Derive trust-minimized price signals for core resources (gold, labor, energy)  
- Maintain audit-safe economic state via hash-linked snapshot chains  
- Enforce peg stability using vault-based entropy anchoring  
- Enable validator and public node valuation without live market access  

---

## 🧩 Core Logic

- Computes multi-input prices from energy, labor, and gold via regional weightings  
- Hash-seals each economic state into snapshot blocks with audit trail  
- Uses entropy anchors from the `vault/` module to bind valuations to civic consensus  
- Supports fallback valuation sync via LoRa, satellite, or cold-boot replay  

---

## 🔗 Inter-module Dependencies

- `vault/` – Provides entropy signatures and secure snapshot anchoring  
- `valuta/` – Handles unit precision, pegged token logic, and rate encoding  
- `region/` – Modulates price calculations based on local/geopolitical regions  
- `world_signals/` – Transmits peg data via fallback radio or mesh signaling  

---

## 📐 Notable Algorithms / Primitives

- `PegEngine`: Core synthesizer for weighted resource valuation  
- `SnapshotChain`: Immutable log of price states with entropy-linked sealing  
- `ValutaUnit`: Precision-safe representation of pegged or floating economic units  
- `EntropyQuote`: Anchored valuation proof derived from civic entropy bundles  

---

## 🚨 Known Edge Cases / Sim Failures

- Snapshot drift due to unanchored entropy may cause state mismatch on recovery  
- Regional price divergence without quorum may trigger temporary valuation forks  
- Replay attacks possible if snapshot sequence checks are omitted  
- Partial vault compromise can skew downstream pegging unless entropy checks are strict  

---

## 🗂 Relevant Files

- `peg_engine.rs` – Price computation and input weighting logic  
- `valuta.rs` – Monetary representation and unit enforcement  
- `snapshot_chain.rs` – Snapshot ledger and replay safety logic  
- `vault.rs` – Entropy anchoring and signature validation  
- `region.rs` – Geo-weighted influence on price curves  
- `world_signals.rs` – Resilient signal broadcast for peg recovery  

---

## 🔁 Upstream/Downstream Modules

**Upstream:**  
- `compute/` – Supplies raw energy, labor, and gold valuations  
- `vault/` – Delivers civic entropy and anchor bundles  

**Downstream:**  
- `api/` – Exposes peg data to light clients and sync endpoints  
- `satlink/` – Broadcasts pegged snapshots to orbital relays  
- `reentry/` – Uses peg data to restore economic context during cold sync  

---

> ⚖️ **Note:**  
> `pegfeed/` is not just an oracle. It is the heartbeat of ZorroChain’s economic truth — audible even when markets are silent, and the Earth has lost signal.

