# rustfs

**S3-compatible object storage in Rust** — a lightweight, high-performance object storage server implementing the Amazon S3 API.

## What This Gives You

- **S3-compatible API** — drop-in replacement for Amazon S3 in development and production
- **Rust performance** — memory-safe, zero-copy, multi-threaded serving
- **Local deployment** — run object storage on your own infrastructure
- **Minimal footprint** — single binary, low resource usage

## Quick Start

```bash
cargo run -- --data-dir ./data --port 9000
```

Then configure your S3 client:
```python
import boto3
s3 = boto3.client("s3", endpoint_url="http://localhost:9000")
s3.create_bucket(Bucket="my-bucket")
```

## How It Fits

Storage infrastructure for the SuperInstance ecosystem. Stores `SmartCRDT` snapshots, `conservation-spectral` artifacts, and `plato-room` tile data.

## License

MIT
