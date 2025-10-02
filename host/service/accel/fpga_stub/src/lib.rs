//! FPGA acceleration stub implementing the software trait for now.

use anyhow::Result;

pub fn offload_operation(name: &str) -> Result<()> {
    tracing::info!(%name, "Invoking FPGA stub operation");
    Ok(())
}
