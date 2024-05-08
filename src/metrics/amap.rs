use crate::Metrics;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AmapMetrics {
    pub fn new(metrics_name: &[&'static str]) -> Self {
        let map = metrics_name
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }
}

impl Metrics for AmapMetrics {
    fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let cnt = self
            .data
            .get(key)
            .ok_or_else(|| anyhow!("key {} not found", key))?;
        cnt.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?
        }
        Ok(())
    }
}
