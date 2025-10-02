use anyhow::Result;
use serde::Serialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Serialize)]
struct HealthResponse<'a> {
    status: &'a str,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .ok();

    info!("Starting admin API stub");
    let response = HealthResponse { status: "ok" };
    let body = serde_json::to_string_pretty(&response)?;
    println!("{body}");
    Ok(())
}
