#!/usr/bin/env bash

arr=(
    small_encode
    small_decode
    medium_encode
    medium_decode
    large_encode
    large_decode
)

echo "wasmtime: compiling using cranelift..."

for t in ${arr[@]}; do
  wasmtime compile --wasm-features all -o target/wasm32-wasi/release/$t.wasmtime target/wasm32-wasi/release/$t.wasm
done

echo "wasmtime: running cranelift version..."

for t in ${arr[@]}; do
  echo "wasmtime $t cranelift:"
  wasmtime run --wasm-features all --allow-precompiled target/wasm32-wasi/release/$t.wasmtime
done

echo "wasmtime end..."
