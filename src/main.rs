use mem_dbg::{DbgFlags, MemDbg};
use mwe_mem_dbg::Buf;

fn main() {
    const N: usize = 1_000_000;

    let mut buf = Buf::with_capacity(N);
    for i in 0..N {
        buf.pos.push(i * 10);
        buf.hash.push(i as u32);
    }

    let compact_buf = buf.compact();
    eprintln!("Memory usage:");
    compact_buf
        .mem_dbg(DbgFlags::HUMANIZE | DbgFlags::PERCENTAGE | DbgFlags::COLOR)
        .unwrap();
}
