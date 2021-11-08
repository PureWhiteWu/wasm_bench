fn main() {
    let mut buf = wasm_bench::FEED_REQUEST_DATA.to_owned();
    let mut bufs = Vec::with_capacity(10000);
    bufs.resize(10000, buf.clone());
    unsafe {
        let _: wasm_bench::model::feed_request::FeedRequest =
            simd_json::from_slice(buf.as_bytes_mut()).unwrap();

        let start = std::time::Instant::now();
        for i in 0..10000 {
            let _: wasm_bench::model::feed_request::FeedRequest =
                simd_json::from_slice(bufs[i].as_bytes_mut()).unwrap();
        }
        println!("{:?}", start.elapsed() / 10000);
    }
}
