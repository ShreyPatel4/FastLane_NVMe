//! Core primitives shared across host service components.

pub mod error;
pub mod metrics;
pub mod rings;
pub mod tracing;
pub mod types;

pub use error::{CoreError, CoreResult};
pub use types::{IoDesc, IoFlags, IoOp};
