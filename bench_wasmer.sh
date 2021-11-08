#!/usr/bin/env bash

arr=(
    small_encode
    small_decode
    medium_encode
    medium_decode
    large_encode
    large_decode
)

echo "wasmer: compiling using llvm..."

for t in ${arr[@]}; do
  wasmer compile --enable-all --llvm target/wasm32-wasi/release/$t.wasm -o target/wasm32-wasi/release/$t.wasmer.llvm
done

echo "wasmer: compiling using cranelift..."

for t in ${arr[@]}; do
  wasmer compile --enable-all --cranelift target/wasm32-wasi/release/$t.wasm -o target/wasm32-wasi/release/$t.wasmer.cranelift
done

echo "wasmer: running llvm version..."

for t in ${arr[@]}; do
  echo "wasmer $t llvm:"
  wasmer run --enable-all target/wasm32-wasi/release/$t.wasmer.llvm
done

echo "wasmer: running cranelift version..."

for t in ${arr[@]}; do
  echo "wasmer $t cranelift:"
  wasmer run --enable-all target/wasm32-wasi/release/$t.wasmer.cranelift
done

echo "wasmer end..."
