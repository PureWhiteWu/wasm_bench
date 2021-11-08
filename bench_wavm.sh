#!/usr/bin/env bash

arr=(
    small_encode
    small_decode
    medium_encode
    medium_decode
    large_encode
    large_decode
)

echo "wavm: compiling..."

for t in ${arr[@]}; do
  wavm compile --target-cpu znver2 --enable all target/wasm32-wasi/release/$t.wasm target/wasm32-wasi/release/$t.wavm
done

echo "wavm: running..."

for t in ${arr[@]}; do
  echo "wavm $t:"
  wavm run --enable all --precompiled target/wasm32-wasi/release/$t.wavm
done

echo "wavm end..."
