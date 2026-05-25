# SuperInstance/rustfs

Fork of [rustfs/rustfs](https://github.com/rustfs/rustfs) — S3-compatible distributed object storage in Rust.

## Fork Status

**No modifications from upstream.** This is a tracking fork. All changes come from upstream `rustfs/rustfs`.

```bash
# Sync with upstream
git fetch upstream
git merge upstream/main
```

## What RustFS Is

S3-compatible object storage built in Rust. Apache 2.0 licensed (not AGPL). Alternative to MinIO with memory safety and performance from Rust.

### Key features

- Full S3 API compatibility
- OpenStack Swift API with Keystone auth
- Distributed and single-node modes
- Bitrot protection
- Bucket replication
- K8s Helm charts in `helm/`

### When to use this fork

- Your organization needs a private S3 endpoint
- You want to track upstream with the ability to patch
- Fleet integration with other SuperInstance services

## Build & Run

```bash
# Build
cargo build --release

# Single node
./target/release/rustfs server /data

# Distributed (4 nodes)
./target/release/rustfs server \
  http://node{1...4}/data{1...4}

# Docker
docker compose -f docker-compose-simple.yml up
```

### Configuration

Environment variables (same as upstream):

```bash
RUSTFS_ROOT_USER=admin          # Default access key
RUSTFS_ROOT_PASSWORD=changeme   # Default secret key
RUSTFS_BROWSER=on               # Web console
RUSTFS_DOMAIN=mydomain.com      # Virtual-hosted buckets
```

### S3 Client Setup

```python
import boto3

s3 = boto3.client(
    "s3",
    endpoint_url="http://localhost:9000",
    aws_access_key_id="admin",
    aws_secret_access_key="changeme",
)

s3.create_bucket(Bucket="my-bucket")
s3.put_object(Bucket="my-bucket", Key="test.txt", Body=b"hello")
```

## Architecture Reference

For full architecture docs, see [ARCHITECTURE.md](ARCHITECTURE.md). For deployment, see [docker-compose.yml](docker-compose.yml) and [helm/](helm/).

Upstream docs: https://docs.rustfs.com/

## License

Apache 2.0 (upstream license).
