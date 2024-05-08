use anyhow::Result;
use concurrency::HmapMetrics;
use concurrency::{request_worker, task_worker};
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = HmapMetrics::new();
    println!("{}", metrics);
    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}
