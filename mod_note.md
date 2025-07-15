## 🎯 Purpose

Core **decentralized oracle** and **price feed aggregator**.  
Anchors network-wide economic truth and maintains **token stability** under all conditions.

`pegfeed/` is the economic heartbeat of ZorroChain — ensuring fair valuation, pegged consistency, and entropy-sealed proof across time and topology.

---

### 📂 `src/`

- Hosts the **primary engine logic** for pegged price computation  
- Collects **entropy-scored price feeds** from oracle, network, and local inputs  
- Executes **epoch-sync scoring** to derive stable market updates  
- Writes **snapshot chains** consumable by `vault/`, `wallet/`, and fallback systems (`mirror/`, `satlink/`)

---

### 📦 `Cargo.toml`

- Compiles as a **shared crate** for internal use across ZorroChain  
- Linked with:
  - `wallet/` → for local peg state tracking  
  - `mirror/` → for outbound fallback broadcast  
  - `contracts/peg.rs` → for on-chain verification and peg enforcement  

---

## 🧭 Future Plans

- [ ] Merge **oracle entropy** with **network relay latency scores** for adaptive trust  
- [ ] Implement **pegged sync-toll rate** per validator region  
- [ ] Add **vault fee simulation** layer (preview peg entropy cost before committing)  
- [ ] Build a **Visual Peg Monitor** for Foundation Board observability  

---

> 🛡️ **ZorroChain isn’t just a chain — it’s a survival currency platform.**  
> Pegfeed is what keeps sync costs fair, global, and human-resilient.

