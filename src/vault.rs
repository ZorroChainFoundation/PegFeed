use crate::snapshot_chain::{SnapshotChain, PegSnapshot};
use std::sync::Mutex;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// Main vault struct holding the ZOR snapshot chain
pub struct Vault {
    chain: Mutex<SnapshotChain>,
    store_path: PathBuf,
}

impl Vault {
    /// Initialize new vault and create storage directory if missing
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        let store_path = path.into();
        if !store_path.exists() {
            let _ = fs::create_dir_all(&store_path);
        }

        let mut full_path = store_path.clone();
        full_path.push("snapshots.json");

        let chain = Mutex::new(SnapshotChain::load_or_new(&full_path));

        Self {
            chain,
            store_path,
        }
    }

    /// Submit a snapshot to the vault and persist to disk
    pub fn submit_snapshot(&self, snapshot: PegSnapshot) -> Result<(), String> {
        let mut chain = self.chain.lock().unwrap();

        chain.append(snapshot.clone())?;

        // Write chain to file
        let mut full_path = self.store_path.clone();
        full_path.push("snapshots.json");
        chain.save(&full_path);

        // Optionally: write single snapshot by hash
        let mut file_path = self.store_path.clone();
        let hash = &snapshot.snapshot_hash[..8];
        file_path.push(format!("snapshot_{}.json", hash));

        let json = serde_json::to_string_pretty(&snapshot).unwrap_or_default();
        let mut file = File::create(file_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Return latest snapshot if available
    pub fn latest(&self) -> Option<PegSnapshot> {
        let chain = self.chain.lock().unwrap();
        chain.latest().cloned()
    }

    /// Export the entire snapshot chain as JSON
    pub fn export_json(&self) -> String {
        let chain = self.chain.lock().unwrap();
        chain.export_json()
    }

    /// Optional: verify integrity of the full hash chain
    pub fn verify_integrity(&self) -> bool {
        let chain = self.chain.lock().unwrap();
        chain.verify_chain()
    }
}
