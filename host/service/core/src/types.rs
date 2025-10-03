use serde::{Deserialize, Serialize};
use std::fmt;
use std::task::Waker;

/// The type of IO operation requested by the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IoOp {
    /// Read data from the namespace.
    Read,
    /// Write data to the namespace.
    Write,
    /// Flush cached data to persistent media.
    Flush,
    /// Discard blocks without writing them.
    Discard,
}

impl IoOp {
    /// Return the string representation used for metrics labels and logging.
    pub fn as_str(&self) -> &'static str {
        match self {
            IoOp::Read => "read",
            IoOp::Write => "write",
            IoOp::Flush => "flush",
            IoOp::Discard => "discard",
        }
    }
}

/// Flags that modify the IO request semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct IoFlags {
    /// Force unit access. When set, bypass intermediate caches.
    pub fua: bool,
    /// Ensure ordering with respect to previous requests.
    pub barrier: bool,
}

/// Alias for the completion notification primitive associated with an IO.
pub type IoCompletion = Waker;

/// Description of an IO operation flowing through the service.
pub struct IoDesc {
    /// Operation type.
    pub op: IoOp,
    /// Namespace identifier the operation targets.
    pub namespace_id: u32,
    /// Starting logical block address.
    pub lba: u64,
    /// Length of the transfer in logical blocks.
    pub length: u32,
    /// Additional IO flags.
    pub flags: IoFlags,
    /// Completion callback used to resume futures waiting on this IO.
    #[allow(dead_code)]
    pub completion: Option<IoCompletion>,
}

impl IoDesc {
    /// Construct a new IO descriptor with the provided parameters.
    pub fn new(
        op: IoOp,
        namespace_id: u32,
        lba: u64,
        length: u32,
        flags: IoFlags,
        completion: Option<IoCompletion>,
    ) -> Self {
        Self {
            op,
            namespace_id,
            lba,
            length,
            flags,
            completion,
        }
    }
}

impl fmt::Debug for IoDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IoDesc")
            .field("op", &self.op)
            .field("namespace_id", &self.namespace_id)
            .field("lba", &self.lba)
            .field("length", &self.length)
            .field("flags", &self.flags)
            .field("has_completion", &self.completion.is_some())
            .finish()
    }
}
