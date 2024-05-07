use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, Mutex};
#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<Mutex<HashMap<String, i64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let key = key.into();
        let mut data = self.data.lock().map_err(|e| anyhow!("{}", e.to_string()))?;
        let cnt = data.entry(key).or_insert(0);
        *cnt += 1;
        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.data.lock().map_err(|_e| fmt::Error {})?;
        for (k, v) in data.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }
        Ok(())
    }
}
