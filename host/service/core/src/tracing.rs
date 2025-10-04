use crate::error::CoreResult;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{resource::Resource, runtime::Tokio, trace::Config as TraceConfig};
use std::env;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

/// Guard returned from [`init_tracing`] that keeps global tracing state alive.
pub struct TracingGuard {
    otel_installed: bool,
}

impl Drop for TracingGuard {
    fn drop(&mut self) {
        if self.otel_installed {
            opentelemetry::global::shutdown_tracer_provider();
        }
    }
}

/// Initialize tracing with stdout logging and optional OpenTelemetry export.
pub fn init_tracing(service_name: &str) -> CoreResult<TracingGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let fmt_layer = tracing_subscriber::fmt::layer();

    let otlp_endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok();

    if let Some(endpoint) = otlp_endpoint {
        let exporter = opentelemetry_otlp::new_exporter()
            .tonic()
            .with_endpoint(endpoint);
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(TraceConfig::default().with_resource(Resource::new(vec![
                KeyValue::new("service.name", service_name.to_string()),
            ])))
            .with_exporter(exporter)
            .install_batch(Tokio)?;

        let otel_layer = OpenTelemetryLayer::new(tracer);
        let subscriber = Registry::default()
            .with(env_filter)
            .with(fmt_layer)
            .with(otel_layer);

        tracing::subscriber::set_global_default(subscriber)?;
        Ok(TracingGuard {
            otel_installed: true,
        })
    } else {
        let subscriber = Registry::default().with(env_filter).with(fmt_layer);
        if let Err(err) = tracing::subscriber::set_global_default(subscriber) {
            if !err
                .to_string()
                .contains("global default subscriber has already been set")
            {
                return Err(err.into());
            }
        }
        Ok(TracingGuard {
            otel_installed: false,
        })
    }
}
