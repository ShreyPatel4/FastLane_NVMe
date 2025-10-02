//! RDMA transport bindings leveraging a C shim.

use anyhow::Result;
use tracing::info;

#[link(name = "rdma_client")]
extern "C" {
    fn rdma_client_initialize() -> i32;
    fn rdma_client_post_write(
        queue: *const std::os::raw::c_char,
        buffer: *const std::os::raw::c_void,
        length: usize,
    ) -> i32;
}

pub fn initialize() -> Result<()> {
    let rc = unsafe { rdma_client_initialize() };
    if rc != 0 {
        anyhow::bail!("rdma_client_initialize failed: {rc}");
    }
    info!("RDMA client initialized");
    Ok(())
}

pub fn post_write(queue: &str, data: &[u8]) -> Result<()> {
    use std::ffi::CString;

    let queue_c = CString::new(queue).expect("queue name must not contain null bytes");
    let rc = unsafe {
        rdma_client_post_write(
            queue_c.as_ptr(),
            data.as_ptr() as *const std::os::raw::c_void,
            data.len(),
        )
    };
    if rc != 0 {
        anyhow::bail!("rdma_client_post_write failed: {rc}");
    }
    info!(queue, len = data.len(), "RDMA post write completed");
    Ok(())
}
