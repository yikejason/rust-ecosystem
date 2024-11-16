use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

use anyhow::Result;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel(32);
    let handler = worker(rx);

    tokio::spawn(async move {
        let mut i = 0;
        loop {
            i += 1;
            println!("Send task {i}");
            tx.send(format!("task {i}")).await?;
        }
        #[allow(unreachable_code)]
        Ok::<(), anyhow::Error>(())
    });

    handler.join().unwrap();

    Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        while let Some(s) = rx.blocking_recv() {
            let result = expensive_blocking_task(s);
            println!("Result: {}", result);
        }
    })
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
