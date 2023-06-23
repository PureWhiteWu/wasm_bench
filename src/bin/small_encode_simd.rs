#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let data: wasm_bench::model::book::Book = serde_json::from_str(wasm_bench::BOOK_DATA).unwrap();

    let start = std::time::Instant::now();
    for _ in 0..100000 {
        simd_json::to_string(&data).unwrap();
    }
    println!("{:?}", start.elapsed() / 100000);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
