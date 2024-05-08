use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{io, thread};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr)?;
    info!("Dredis: listening on: {}", addr);
    loop {
        let (stream, raddr) = listener.accept()?;
        info!("Accepted connection from: {}", raddr);
        thread::spawn(move || {
            if let Err(e) = process_redis_conn(stream, raddr) {
                warn!("Error processing conn with {}: {:?}", raddr, e);
            }
            Ok::<_, anyhow::Error>(())
        });
    }
}

fn process_redis_conn(mut stream: TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        let mut buf = vec![0; BUF_SIZE];
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf[0..n]);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n")?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(anyhow!("read from connection err: {}", e)),
        }
    }
    warn!("connection {:?} closed", raddr);
    Ok(())
}
