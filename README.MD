# Azure Storage Offload

Azure Storage Offload is a production-quality storage offload stack that exposes NVMe semantics to guests while leveraging RDMA and acceleration offloads under the hood.

## Repository Layout

- `Cargo.toml` — Rust workspace connecting host services and tooling crates.
- `docs/` — Design, operations, and benchmarking guides.
- `device/` — VFIO-user NVMe shim and Windows miniport skeleton.
- `host/` — Host-side services, transports, and acceleration modules.
- `telemetry/` — Prometheus rules and Grafana dashboards.
- `test/` — FIO, Diskspd, and chaos experiment scaffolding.
- `tools/` — Developer tooling such as trace viewers and packet analyzers.
- `deploy/` — Deployment manifests for Kubernetes and Docker environments.

## Getting Started

```bash
make build
make test
make fmt
make clippy
```

The workspace uses the stable Rust toolchain pinned via `rust-toolchain.toml` with `rustfmt` and `clippy` components enabled.
