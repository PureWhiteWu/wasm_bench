fn main() {
    let data: wasm_bench::model::twitter::Twitter =
        serde_json::from_str(wasm_bench::TWITTER_DATA).unwrap();

    let start = std::time::Instant::now();
    for _ in 0..1000 {
        serde_json::to_string(&data).unwrap();
    }
    println!("{:?}", start.elapsed() / 1000);
}
