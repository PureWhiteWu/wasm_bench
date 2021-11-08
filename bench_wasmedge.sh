#!/usr/bin/env bash

arr=(
    small_encode
    small_decode
    medium_encode
    medium_decode
    large_encode
    large_decode
)

echo "wasmedge: compiling to native..."

for t in ${arr[@]}; do
  wasmedgec --enable-all target/wasm32-wasi/release/$t.wasm target/wasm32-wasi/release/$t.wasmedge.so
done

echo "wasmedge: running native version..."

for t in ${arr[@]}; do
  echo "wasmedge $t native:"
  wasmedge --enable-all target/wasm32-wasi/release/$t.wasmedge.so
done

echo "wasmedge end..."
