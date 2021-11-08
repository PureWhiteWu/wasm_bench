fn main() {
    let _: wasm_bench::model::feed_request::FeedRequest =
        serde_json::from_str(wasm_bench::FEED_REQUEST_DATA).unwrap();

    let start = std::time::Instant::now();
    for _ in 0..10000 {
        let _: wasm_bench::model::feed_request::FeedRequest =
            serde_json::from_str(wasm_bench::FEED_REQUEST_DATA).unwrap();
    }
    println!("{:?}", start.elapsed() / 10000);
}
