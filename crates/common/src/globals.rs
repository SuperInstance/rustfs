// Copyright 2024 RustFS Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(non_upper_case_globals)] // FIXME

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tonic::transport::Channel;

pub static GLOBAL_LOCAL_NODE_NAME: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".to_string()));
pub static GLOBAL_RUSTFS_HOST: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".to_string()));
pub static GLOBAL_RUSTFS_PORT: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("9000".to_string()));
pub static GLOBAL_RUSTFS_ADDR: LazyLock<RwLock<String>> = LazyLock::new(|| RwLock::new("".to_string()));
pub static GLOBAL_CONN_MAP: LazyLock<RwLock<HashMap<String, Channel>>> = LazyLock::new(|| RwLock::new(HashMap::new()));
pub static GLOBAL_ROOT_CERT: LazyLock<RwLock<Option<Vec<u8>>>> = LazyLock::new(|| RwLock::new(None));
pub static GLOBAL_MTLS_IDENTITY: LazyLock<RwLock<Option<MtlsIdentityPem>>> = LazyLock::new(|| RwLock::new(None));
pub static GLOBAL_OUTBOUND_TLS_GENERATION: LazyLock<AtomicU64> = LazyLock::new(|| AtomicU64::new(0));
/// Global initialization time of the RustFS node.
pub static GLOBAL_INIT_TIME: LazyLock<RwLock<Option<DateTime<Utc>>>> = LazyLock::new(|| RwLock::new(None));

/// Set the global local node name.
///
/// # Arguments
/// * `name` - A string slice representing the local node name.
pub async fn set_global_local_node_name(name: &str) {
    *GLOBAL_LOCAL_NODE_NAME.write().await = name.to_string();
}

/// Get the global local node name.
///
/// # Returns
/// * `String` - The local node name.
pub async fn get_global_local_node_name() -> String {
    GLOBAL_LOCAL_NODE_NAME.read().await.clone()
}

/// Set the global RustFS initialization time to the current UTC time.
pub async fn set_global_init_time_now() {
    let now = Utc::now();
    *GLOBAL_INIT_TIME.write().await = Some(now);
}

/// Get the global RustFS initialization time.
///
/// # Returns
/// * `Option<DateTime<Utc>>` - The initialization time if set.
pub async fn get_global_init_time() -> Option<DateTime<Utc>> {
    *GLOBAL_INIT_TIME.read().await
}

/// Set the global RustFS address used for gRPC connections.
///
/// # Arguments
/// * `addr` - A string slice representing the RustFS address (e.g., "https://node1:9000").
pub async fn set_global_addr(addr: &str) {
    *GLOBAL_RUSTFS_ADDR.write().await = addr.to_string();
}

/// Set the global root CA certificate for outbound gRPC clients.
/// This certificate is used to validate server TLS certificates.
/// When set to None, clients use the system default root CAs.
///
/// # Arguments
/// * `cert` - A vector of bytes representing the PEM-encoded root CA certificate.
pub async fn set_global_root_cert(cert: Vec<u8>) {
    *GLOBAL_ROOT_CERT.write().await = Some(cert);
}

/// Set the global mTLS identity (cert+key PEM) for outbound gRPC clients.
/// When set, clients will present this identity to servers requesting/requiring mTLS.
/// When None, clients proceed with standard server-authenticated TLS.
///
/// # Arguments
/// * `identity` - An optional MtlsIdentityPem struct containing the cert and key PEM.
pub async fn set_global_mtls_identity(identity: Option<MtlsIdentityPem>) {
    *GLOBAL_MTLS_IDENTITY.write().await = identity;
}

/// Set the global outbound TLS generation.
pub fn set_global_outbound_tls_generation(generation: u64) {
    GLOBAL_OUTBOUND_TLS_GENERATION.store(generation, Ordering::Relaxed);
}

/// Get the global outbound TLS generation.
pub fn get_global_outbound_tls_generation() -> u64 {
    GLOBAL_OUTBOUND_TLS_GENERATION.load(Ordering::Relaxed)
}

/// Evict a stale/dead connection from the global connection cache.
/// This is critical for cluster recovery when a node dies unexpectedly (e.g., power-off).
/// By removing the cached connection, subsequent requests will establish a fresh connection.
///
/// # Arguments
/// * `addr` - The address of the connection to evict.
pub async fn evict_connection(addr: &str) {
    let removed = GLOBAL_CONN_MAP.write().await.remove(addr);
    if removed.is_some() {
        tracing::warn!("Evicted stale connection from cache: {}", addr);
    }
}

/// Check if a connection exists in the cache for the given address.
///
/// # Arguments
/// * `addr` - The address to check.
///
/// # Returns
/// * `bool` - True if a cached connection exists, false otherwise.
pub async fn has_cached_connection(addr: &str) -> bool {
    GLOBAL_CONN_MAP.read().await.contains_key(addr)
}

/// Clear all cached connections. Useful for full cluster reset/recovery.
pub async fn clear_all_connections() {
    let mut map = GLOBAL_CONN_MAP.write().await;
    let count = map.len();
    map.clear();
    if count > 0 {
        tracing::warn!("Cleared {} cached connections from global map", count);
    }
}
/// Optional client identity (cert+key PEM) for outbound mTLS.
///
/// When present, gRPC clients will present this identity to servers requesting/requiring mTLS.
/// When absent, clients proceed with standard server-authenticated TLS.
#[derive(Clone, Debug)]
pub struct MtlsIdentityPem {
    pub cert_pem: Vec<u8>,
    pub key_pem: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_and_get_local_node_name() {
        set_global_local_node_name("test-node-1").await;
        assert_eq!(get_global_local_node_name().await, "test-node-1");
    }

    #[tokio::test]
    async fn test_set_and_get_addr() {
        set_global_addr("https://node1:9000").await;
        let addr = GLOBAL_RUSTFS_ADDR.read().await.clone();
        assert_eq!(addr, "https://node1:9000");
    }

    #[tokio::test]
    async fn test_set_and_get_root_cert() {
        set_global_root_cert(vec![1, 2, 3]).await;
        let cert = GLOBAL_ROOT_CERT.read().await.clone();
        assert_eq!(cert, Some(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn test_mtls_identity() {
        let identity = MtlsIdentityPem {
            cert_pem: b"cert".to_vec(),
            key_pem: b"key".to_vec(),
        };
        set_global_mtls_identity(Some(identity.clone())).await;
        let stored = GLOBAL_MTLS_IDENTITY.read().await.clone();
        assert!(stored.is_some());
        let stored = stored.unwrap();
        assert_eq!(stored.cert_pem, b"cert");
        assert_eq!(stored.key_pem, b"key");

        set_global_mtls_identity(None).await;
        assert!(GLOBAL_MTLS_IDENTITY.read().await.is_none());
    }

    #[test]
    fn test_outbound_tls_generation() {
        set_global_outbound_tls_generation(42);
        assert_eq!(get_global_outbound_tls_generation(), 42);
        set_global_outbound_tls_generation(99);
        assert_eq!(get_global_outbound_tls_generation(), 99);
    }

    #[tokio::test]
    async fn test_connection_cache_evict_and_clear() {
        // We can't easily insert a real Channel, but we can test evict/has_cached on empty cache.
        assert!(!has_cached_connection("no-such-addr").await);
        evict_connection("no-such-addr").await; // should not panic
        clear_all_connections().await; // should not panic
    }

    #[tokio::test]
    async fn test_set_and_get_init_time() {
        assert!(get_global_init_time().await.is_none());
        set_global_init_time_now().await;
        let t = get_global_init_time().await;
        assert!(t.is_some());
    }
}
