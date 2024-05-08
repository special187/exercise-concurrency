use anyhow::Result;
use concurrency::AmapMetrics;
use concurrency::{request_worker, task_worker};
use std::thread;
use std::time::Duration;
const N: usize = 2;
const M: usize = 4;
fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);
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
