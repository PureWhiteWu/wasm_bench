#!/usr/bin/env bash

arr=(
    small_encode
    small_encode_simd
    small_decode
    small_decode_simd
    medium_encode
    medium_encode_simd
    medium_decode
    medium_decode_simd
    large_encode
    large_encode_simd
    large_decode
    large_decode_simd
)

echo "native: running native version..."

for t in ${arr[@]}; do
  echo "native $t:"
  target/release/$t
done

echo "native end..."
