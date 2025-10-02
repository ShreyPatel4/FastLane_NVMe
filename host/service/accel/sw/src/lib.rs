//! Software acceleration routines.

use anyhow::Result;

pub fn checksum(data: &[u8]) -> Result<u32> {
    Ok(data
        .iter()
        .fold(0u32, |acc, byte| acc.wrapping_add(*byte as u32)))
}
