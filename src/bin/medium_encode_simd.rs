fn main() {
    let mut buf = wasm_bench::FEED_REQUEST_DATA.to_owned();
    unsafe {
        let data: wasm_bench::model::feed_request::FeedRequest =
            simd_json::from_slice(buf.as_bytes_mut()).unwrap();

        let start = std::time::Instant::now();
        for _ in 0..10000 {
            simd_json::to_string(&data).unwrap();
        }
        println!("{:?}", start.elapsed() / 10000);
    }
}
