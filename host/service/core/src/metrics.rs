use crate::error::{CoreError, CoreResult};
use crate::types::IoOp;
use once_cell::sync::Lazy;
use prometheus::{
    Encoder, Histogram, HistogramOpts, IntCounter, IntCounterVec, IntGauge, Opts, Registry,
    TextEncoder,
};

static METRICS: Lazy<Metrics> = Lazy::new(|| Metrics::new().expect("metrics initialization"));

/// Access the global metrics handle used across the host service.
pub fn metrics() -> &'static Metrics {
    &METRICS
}

/// Aggregated metrics for the host service core.
pub struct Metrics {
    registry: Registry,
    pub io_latency_seconds: Histogram,
    pub io_errors_total: IntCounterVec,
    pub nvme_queue_depth: IntGauge,
    pub nvme_timeouts_total: IntCounter,
    pub rdma_cq_overflow_total: IntCounter,
}

impl Metrics {
    /// Create and register the metrics required by the service.
    pub fn new() -> CoreResult<Self> {
        let registry = Registry::new();

        let latency_opts = HistogramOpts::new(
            "io_latency_seconds",
            "Latency distribution of IO operations in seconds",
        )
        .buckets(vec![
            50e-6, 100e-6, 200e-6, 500e-6, 1e-3, 2e-3, 5e-3, 1e-2, 2e-2, 5e-2, 0.1, 0.25, 0.5, 1.0,
            2.0, 5.0,
        ]);
        let io_latency_seconds = Histogram::with_opts(latency_opts)?;
        registry.register(Box::new(io_latency_seconds.clone()))?;

        let io_errors_total = IntCounterVec::new(
            Opts::new(
                "io_errors_total",
                "Count of IO errors grouped by operation and reason",
            ),
            &["op", "reason"],
        )?;
        registry.register(Box::new(io_errors_total.clone()))?;

        let nvme_queue_depth = IntGauge::with_opts(Opts::new(
            "nvme_queue_depth",
            "Current NVMe queue depth observed by the service",
        ))?;
        registry.register(Box::new(nvme_queue_depth.clone()))?;

        let nvme_timeouts_total = IntCounter::with_opts(Opts::new(
            "nvme_timeouts_total",
            "Total NVMe command timeouts observed",
        ))?;
        registry.register(Box::new(nvme_timeouts_total.clone()))?;

        let rdma_cq_overflow_total = IntCounter::with_opts(Opts::new(
            "rdma_cq_overflow_total",
            "Total RDMA completion queue overflow events",
        ))?;
        registry.register(Box::new(rdma_cq_overflow_total.clone()))?;

        Ok(Self {
            registry,
            io_latency_seconds,
            io_errors_total,
            nvme_queue_depth,
            nvme_timeouts_total,
            rdma_cq_overflow_total,
        })
    }

    /// Render the registered metrics to a Prometheus-compatible text format.
    pub fn gather(&self) -> CoreResult<String> {
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        TextEncoder::new()
            .encode(&metric_families, &mut buffer)
            .map_err(CoreError::from)?;
        Ok(String::from_utf8(buffer)?)
    }

    /// Observe latency for a given IO.
    pub fn observe_io_latency(&self, seconds: f64) {
        self.io_latency_seconds.observe(seconds);
    }

    /// Increment the IO error counter for the provided operation and reason.
    pub fn inc_io_error(&self, op: IoOp, reason: &str) {
        self.io_errors_total
            .with_label_values(&[op.as_str(), reason])
            .inc();
    }

    /// Update the queue depth gauge.
    pub fn set_queue_depth(&self, depth: i64) {
        self.nvme_queue_depth.set(depth);
    }

    /// Increment the NVMe timeout counter.
    pub fn inc_nvme_timeout(&self) {
        self.nvme_timeouts_total.inc();
    }

    /// Increment the RDMA CQ overflow counter.
    pub fn inc_rdma_cq_overflow(&self) {
        self.rdma_cq_overflow_total.inc();
    }

    /// Provide read-only access to the underlying registry.
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
}
