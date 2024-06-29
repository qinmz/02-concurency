use std::sync::mpsc;
// TODO： mpsc不知道是啥
use anyhow::{self, Result};
use std::thread;
use std::time::Duration;

const NUM_PRODUCER: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    // 创建一个通道，返回一个发送者（tx）和一个接收者（rx）
    let (tx, rx) = mpsc::channel();

    // 创建一个生产者线程
    for i in 0..NUM_PRODUCER {
        let tx = tx.clone();
        thread::spawn(move || produce(i, tx));
    }

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
    });

    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Failed to join consumer thread: {:?}", e))?;

    Ok(())
}

fn produce(idex: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idex, value))?;
        thread::sleep(Duration::from_secs(1));
    }
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Msg { idx, value }
    }
}
