//! Core state machines and data structures for Azure Storage Offload.

use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub depth: u16,
    pub io_size_bytes: u32,
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            depth: 64,
            io_size_bytes: 4096,
        }
    }
}

pub fn initialize() -> anyhow::Result<()> {
    let config = QueueConfig::default();
    info!(?config, "Initializing core service state");
    Ok(())
}
