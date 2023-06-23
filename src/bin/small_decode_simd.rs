#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut buf = wasm_bench::BOOK_DATA.to_owned();
    let mut bufs = Vec::with_capacity(100000);
    bufs.resize(100000, buf.clone());
    unsafe {
        let _: wasm_bench::model::book::Book = simd_json::from_slice(buf.as_bytes_mut()).unwrap();

        let start = std::time::Instant::now();
        for i in 0..100000 {
            let _: wasm_bench::model::book::Book =
                simd_json::from_slice(bufs[i].as_bytes_mut()).unwrap();
        }
        println!("{:?}", start.elapsed() / 100000);
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}

