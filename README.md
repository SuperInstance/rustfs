# RustFS

[![CI](https://github.com/SuperInstance/rustfs/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/rustfs/actions)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![Rust: 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](rust-toolchain.toml)

**High-performance, S3-compatible distributed object storage in Rust.**

RustFS is a feature-complete object storage server implementing the Amazon S3 API, built for self-hosted infrastructure that needs MinIO-class capabilities with Rust's memory safety and performance characteristics.

---

## Features

- **Full S3 API** — Buckets, objects, multipart uploads, versioning, lifecycle, replication, encryption, IAM/STS, CORS, tagging, and more
- **Erasure coding** — Reed-Solomon with SIMD acceleration for data durability without full replication overhead
- **Server-side encryption** — SSE-S3, SSE-KMS, and SSE-C (customer-provided keys)
- **S3 Select** — Server-side SQL queries on CSV, JSON, and Parquet objects (backed by Apache DataFusion)
- **Multi-protocol gateways** — WebDAV, FTPS, and SFTP access to the same object namespace (feature flags)
- **IAM & OIDC** — Fine-grained access policies, STS token vending, OpenID Connect integration
- **Bitrot healing** — Automatic detection and repair of corrupted data via background scanner
- **Observability** — OpenTelemetry tracing, Prometheus metrics, structured logging
- **Web console** — Built-in admin UI at port 9001
- **Single binary** — Minimal footprint, no runtime dependencies, low memory usage

## Quick Start

### Docker (recommended)

```bash
docker compose -f docker-compose-simple.yml up -d
```

This starts RustFS on:
- **S3 API:** `http://localhost:9000`
- **Console:** `http://localhost:9001`

Default credentials: `rustfsadmin` / `rustfsadmin` (change in production!)

### From Source

```bash
# Requires Rust 1.95+
cargo build --release
./target/release/rustfs server /data/rustfs{0...3}
```

### Connect Your S3 Client

**Python (boto3):**
```python
import boto3

s3 = boto3.client(
    "s3",
    endpoint_url="http://localhost:9000",
    aws_access_key_id="rustfsadmin",
    aws_secret_access_key="rustfsadmin",
)

# Create a bucket and upload a file
s3.create_bucket(Bucket="my-bucket")
s3.put_object(Bucket="my-bucket", Key="hello.txt", Body=b"Hello, RustFS!")
print(s3.get_object(Bucket="my-bucket", Key="hello.txt")["Body"].read())
```

**Rust (aws-sdk-s3):**
```rust
use aws_sdk_s3::Client;
use aws_config::Region;

let config = aws_config::from_env()
    .endpoint_url("http://localhost:9000")
    .region(Region::new("us-east-1"))
    .load()
    .await;
let client = Client::new(&config);

client.create_bucket().bucket("my-bucket").send().await?;
client.put_object()
    .bucket("my-bucket")
    .key("hello.txt")
    .body(bytes::Bytes::from("Hello, RustFS!"))
    .send()
    .await?;
```

**AWS CLI:**
```bash
aws --endpoint-url http://localhost:9000 s3 mb s3://my-bucket
aws --endpoint-url http://localhost:9000 s3 cp file.txt s3://my-bucket/
```

## Architecture

RustFS is a Cargo workspace with 39 library crates organized in layers:

```
Request → Server → App (use cases) → Storage (ECFS) → ECStore → Rio (I/O pipeline) → Disk
```

| Layer | Crates | Responsibility |
|-------|--------|---------------|
| Server | `rustfs/src/server/` | HTTP listener, TLS, CORS, compression, routing |
| Admin | `rustfs/src/admin/` | Admin API, web console handlers |
| App | `object_usecase`, `bucket_usecase`, `multipart_usecase` | S3 operation orchestration |
| Storage | `ecfs`, `ecstore` | Erasure coding, encryption, data distribution |
| I/O | `rio`, `io-core`, `io-metrics` | Zero-copy I/O, buffer pools, backpressure |
| Security | `crypto`, `signer`, `credentials`, `iam`, `kms` | Auth, encryption, key management |
| Infra | `lock`, `heal`, `scanner`, `notify`, `audit` | Distributed coordination, maintenance |

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full code map and dependency DAG.

## Configuration

RustFS is configured primarily through environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `RUSTFS_VOLUMES` | Disk paths for erasure coding (e.g., `/data/rustfs{0...3}`) | Required |
| `RUSTFS_ADDRESS` | S3 API listen address | `:9000` |
| `RUSTFS_CONSOLE_ADDRESS` | Console listen address | `:9001` |
| `RUSTFS_ACCESS_KEY` | Root access key | Required |
| `RUSTFS_SECRET_KEY` | Root secret key | Required |
| `RUSTFS_CONSOLE_ENABLE` | Enable web console | `false` |
| `RUSTFS_TLS_PATH` | TLS certificate directory | (none) |
| `RUSTFS_OBS_LOGGER_LEVEL` | Log level | `info` |

See [docker-compose-simple.yml](docker-compose-simple.yml) for a complete example.

## API Coverage

### Object Operations
- `PutObject` / `GetObject` / `HeadObject` / `DeleteObject`
- `CopyObject` / `RestoreObject`
- `SelectObjectContent` (SQL on CSV/JSON/Parquet)
- `PutObjectExtract` (auto-extract archives)

### Multipart Upload
- `CreateMultipartUpload` / `UploadPart` / `CompleteMultipartUpload`
- `AbortMultipartUpload` / `ListParts` / `UploadPartCopy`

### Bucket Operations
- `CreateBucket` / `DeleteBucket` / `HeadBucket` / `ListBuckets`
- Bucket lifecycle, CORS, replication, encryption, versioning, tagging, policy, notification

### Advanced
- IAM user/policy management via Admin API
- STS token vending for temporary credentials
- Bitrot detection and auto-healing
- Background data usage scanner

## Development

```bash
# Format, lint, check, and test
just pre-commit

# Or individually:
just fmt          # cargo fmt --all
just clippy       # cargo clippy --all-targets --all-features
just check        # cargo check --all-targets
just test         # cargo nextest run + doc tests
```

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full development workflow.

## SuperInstance Ecosystem

RustFS serves as the storage backbone for the [SuperInstance](https://github.com/SuperInstance) ecosystem:

- **SmartCRDT** — Document snapshots stored as versioned S3 objects with lifecycle tiering
- **conservation-spectral** — Spectral analysis artifacts and processed data
- **plato-room** — Spatial tile data served with cache headers for real-time room rendering
- **ESP32 sensors** — Time-partitioned sensor readings backed up via S3 with server-side query (S3 Select)

## Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) — Full code map, crate reference, and dependency DAG
- [CONTRIBUTING.md](CONTRIBUTING.md) — Development workflow and PR process
- [CHANGELOG.md](CHANGELOG.md) — Release history
- [SECURITY.md](SECURITY.md) — Security policy

## License

Apache License 2.0 — see [LICENSE](LICENSE).
