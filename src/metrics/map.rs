use crate::Metrics;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct HmapMetrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl HmapMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Metrics for HmapMetrics {
    fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref().to_string();
        let mut data = self.data.lock().map_err(|e| anyhow!("{}", e.to_string()))?;
        let cnt = data.entry(key).or_insert(0);
        *cnt += 1;
        Ok(())
    }
}

impl Default for HmapMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for HmapMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.data.lock().map_err(|_e| fmt::Error {})?;
        for (k, v) in data.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }
        Ok(())
    }
}
