use std::time::Instant;
use std::hint::black_box;
use aethelarch_core::{aethelarch_matvec_ternary, sbe_attention_coincidence};

fn main() {
    println!("AETHELARCH CORE MOBILE EXECUTION HARNESS");

    let length = 100 * 1024 * 1024; // 100 MiB
    let x = vec![0xAAu8; length];
    let m = vec![0xFFu8; length];
    let s = vec![0x0Fu8; length];

    let start = Instant::now();
    let result = unsafe {
        black_box(aethelarch_matvec_ternary(
            x.as_ptr(),
            m.as_ptr(),
            s.as_ptr(),
            length,
        ))
    };
    let duration = start.elapsed();

    let seconds = duration.as_secs_f64();
    let gops = (length as f64 * 8.0) / seconds / 1e9;
    let bandwidth = (length as f64 * 3.0) / seconds / 1e9;

    println!("Throughput      : {:.2} GOPS", gops);
    println!("Memory Bandwidth: {:.2} GB/s", bandwidth);
    println!("Dot Product Result: {}", result);

    let coincidence = unsafe { sbe_attention_coincidence(0b1101_0010, 0b1001_0000) };
    println!("SBE Coincidence Score: {} bits", coincidence);
}
