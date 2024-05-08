use anyhow::Result;
use rand::Rng;
use std::thread;
use std::time::Duration;

pub trait Metrics {
    fn inc(&self, key: impl AsRef<str>) -> Result<()>;
}

pub fn task_worker(idx: usize, metrics: impl Metrics + Send + 'static) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

pub fn request_worker(metrics: impl Metrics + Send + 'static) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
