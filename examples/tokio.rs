use std::{thread, time::Duration};

use tokio::{fs, runtime::Runtime, time::sleep};

fn main() {
    let handle = thread::spawn(|| {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(run(&rt))
    });

    handle.join().unwrap();
}

async fn run(rt: &Runtime) {
    rt.spawn(async {
        println!("future 1");
        let content = fs::read("Cargo.toml").await.unwrap();
        println!("{:?}", content.len());
    });

    rt.spawn(async {
        println!("future 2");
        let result = expensive_block_task("Hello, world!".to_string());
        println!("Result: {}", result);
    });

    sleep(Duration::from_secs(1)).await;
}

fn expensive_block_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    blake3::hash(s.as_bytes()).to_string()
}
