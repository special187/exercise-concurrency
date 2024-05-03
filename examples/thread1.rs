use anyhow::{anyhow, Result};
use rand::random;
use std::sync::mpsc::Sender;
use std::{sync::mpsc, thread, time};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    data: usize,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("{:?}", msg);
        }
        println!("consumer exit");
        365
    });

    let ret = consumer
        .join()
        .map_err(|e| anyhow!("Thread join err: {:?}", e))?;

    println!("secret: {}", ret);

    Ok(())
}

fn producer(idx: usize, tx: Sender<Msg>) -> Result<()> {
    loop {
        let msg = Msg {
            idx,
            data: random(),
        };
        tx.send(msg)?;
        if random::<u8>() % 2 == 0 {
            break;
        }
        let t = random::<u8>() as u64 * 10;
        thread::sleep(time::Duration::from_millis(t));
    }
    println!("producer {} exit", idx);
    Ok(())
}
