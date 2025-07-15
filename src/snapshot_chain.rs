use std::fs::{self, File};
use std::path::Path;
use std::io::{Write, Read};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PegSnapshot {
    pub timestamp: String,
    pub input_hash: String,
    pub snapshot_hash: String,
    pub zor_value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PegSnapshotSigned {
    pub snapshot: PegSnapshot,
    pub signature: String,
    pub public_key: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotChain {
    pub snapshots: Vec<PegSnapshot>,
}

impl SnapshotChain {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    pub fn latest(&self) -> Option<&PegSnapshot> {
        self.snapshots.last()
    }

    /// Append a new snapshot after validating hash chain
    pub fn append(&mut self, snapshot: PegSnapshot) -> Result<(), String> {
        if let Some(prev) = self.latest() {
            if snapshot.input_hash == prev.snapshot_hash {
                self.snapshots.push(snapshot);
                Ok(())
            } else {
                Err("Snapshot hash chain broken!".to_string())
            }
        } else {
            self.snapshots.push(snapshot);
            Ok(())
        }
    }

    pub fn export_json(&self) -> String {
        serde_json::to_string(&self.snapshots).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn verify_chain(&self) -> bool {
        for i in 1..self.snapshots.len() {
            if self.snapshots[i].input_hash != self.snapshots[i - 1].snapshot_hash {
                return false;
            }
        }
        true
    }

    pub fn load_or_new<P: AsRef<Path>>(path: P) -> Self {
        match fs::read_to_string(&path) {
            Ok(content) => {
                serde_json::from_str(&content).unwrap_or_else(|_| SnapshotChain::new())
            }
            Err(_) => SnapshotChain::new(),
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) {
        if let Ok(mut file) = File::create(path) {
            let json = self.export_json();
            let _ = file.write_all(json.as_bytes());
        }
    }
}
