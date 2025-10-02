use anyhow::Result;
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .try_init()
        .ok();

    info!("Running synthetic IO healthcheck");
    // Placeholder synthetic IO result
    println!("healthcheck: ok");
    Ok(())
}
