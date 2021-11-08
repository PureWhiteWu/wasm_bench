fn main() {
    let _: wasm_bench::model::book::Book = serde_json::from_str(wasm_bench::BOOK_DATA).unwrap();

    let start = std::time::Instant::now();
    for _ in 0..100000 {
        let _: wasm_bench::model::book::Book = serde_json::from_str(wasm_bench::BOOK_DATA).unwrap();
    }
    println!("{:?}", start.elapsed() / 100000);
}
