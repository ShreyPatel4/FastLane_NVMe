//! TCP transport bindings for Azure Storage Offload.

use anyhow::Result;
use tracing::info;

pub fn connect(endpoint: &str) -> Result<()> {
    info!(%endpoint, "Establishing TCP transport");
    Ok(())
}
