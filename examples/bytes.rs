use anyhow::Result;
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"Hello, world!");
    buf.put(&b"yutian"[..]);

    println!("{:?}", buf);

    let a = buf.split();
    let mut b = a.freeze();
    let c = b.split_to(12);

    println!("{:?}", c);

    println!("{:?}", b);
    println!("{:?}", buf);

    Ok(())
}