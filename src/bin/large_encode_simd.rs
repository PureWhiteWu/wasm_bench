fn main() {
    let mut buf = wasm_bench::TWITTER_DATA.to_owned();
    unsafe {
        let data: wasm_bench::model::twitter::Twitter =
            simd_json::from_slice(buf.as_bytes_mut()).unwrap();

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            simd_json::to_string(&data).unwrap();
        }
        println!("{:?}", start.elapsed() / 1000);
    }
}
