fn main() {
    unsafe {
        let mut buf = wasm_bench::TWITTER_DATA.to_owned();
        let mut bufs = Vec::with_capacity(1000);
        bufs.resize(1000, buf.clone());
        let _: wasm_bench::model::twitter::Twitter =
            simd_json::from_slice(buf.as_bytes_mut()).unwrap();

        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _: wasm_bench::model::twitter::Twitter =
                simd_json::from_slice(bufs[i].as_bytes_mut()).unwrap();
        }
        println!("{:?}", start.elapsed() / 1000);
    }
}
