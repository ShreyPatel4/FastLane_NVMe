use opentelemetry::trace::TraceError;
use prometheus::Error as PrometheusError;
use std::string::FromUtf8Error;
use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;

/// Convenient result alias for core operations.
pub type CoreResult<T> = Result<T, CoreError>;

/// Errors surfaced from the core crate.
#[derive(Debug, Error)]
pub enum CoreError {
    /// The single-producer single-consumer ring buffer is full.
    #[error("ring buffer is full")]
    RingFull,

    /// The ring buffer is empty when attempting to pop.
    #[error("ring buffer is empty")]
    RingEmpty,

    /// Errors that occur while registering or manipulating metrics.
    #[error("metrics error: {0}")]
    Metrics(#[from] PrometheusError),

    /// Errors when converting UTF-8 metric responses.
    #[error("utf8 conversion error: {0}")]
    Utf8(#[from] FromUtf8Error),

    /// Tracing subscriber initialization failure.
    #[error("tracing initialization failed: {0}")]
    Tracing(#[from] SetGlobalDefaultError),

    /// Errors while configuring OpenTelemetry exporters.
    #[error("opentelemetry error: {0}")]
    OpenTelemetry(#[from] TraceError),
}
