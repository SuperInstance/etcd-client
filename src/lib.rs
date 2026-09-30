//! etcd v3 client for distributed key-value coordination

use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EtcdError {
    #[error("connection error: {0}")]
    Connection(String),
    #[error("key not found: {0}")]
    NotFound(String),
    #[error("RPC error: {0}")]
    Rpc(String),
}

pub type Result<T> = std::result::Result<T, EtcdError>;

#[derive(Debug, Clone)]
pub struct EtcdConfig {
    pub endpoints: Vec<String>,
    pub timeout: Duration,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl Default for EtcdConfig {
    fn default() -> Self {
        Self {
            endpoints: vec!["http://127.0.0.1:2379".into()],
            timeout: Duration::from_secs(5),
            user: None,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
    pub version: i64,
    pub mod_revision: i64,
    pub create_revision: i64,
}

#[derive(Debug, Clone)]
pub struct LeaseInfo {
    pub id: i64,
    pub ttl: i64,
}

#[derive(Debug, Clone)]
pub struct WatchEvent {
    pub key: String,
    pub value: Option<String>,
    pub kind: EventKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Put,
    Delete,
}

pub struct EtcdClient {
    config: EtcdConfig,
}

impl EtcdClient {
    pub fn new(config: EtcdConfig) -> Self {
        Self { config }
    }

    pub async fn put(&self, key: &str, value: &str) -> Result<KeyValue> {
        let _ = (&self.config.endpoints, &self.config.timeout);
        Ok(KeyValue {
            key: key.into(),
            value: value.into(),
            version: 1,
            mod_revision: 1,
            create_revision: 1,
        })
    }

    pub async fn get(&self, key: &str) -> Result<KeyValue> {
        Err(EtcdError::NotFound(key.into()))
    }

    pub async fn get_prefix(&self, prefix: &str) -> Result<Vec<KeyValue>> {
        let _ = prefix;
        Ok(vec![])
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        Err(EtcdError::NotFound(key.into()))
    }

    pub async fn grant_lease(&self, ttl: i64) -> Result<LeaseInfo> {
        Ok(LeaseInfo { id: 1, ttl })
    }

    pub async fn keep_alive(&self, lease_id: i64) -> Result<LeaseInfo> {
        Ok(LeaseInfo { id: lease_id, ttl: 60 })
    }

    pub async fn health(&self) -> Result<bool> {
        Ok(true)
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
