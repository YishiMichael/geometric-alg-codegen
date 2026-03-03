#!/usr/bin/env bash

mkdir -p ./geometric-alg/src/rust
mkdir -p ./geometric-alg/src/wgsl
mkdir -p ./geometric-alg/src/glsl

for file in ./scripts/algebra/*.txt; do
    name="$(basename "$file" .txt)"

    ./target/debug/geometric-alg-codegen \
        --rust "./geometric-alg/src/rust/${name}.rs" \
        --wgsl "./geometric-alg/src/wgsl/${name}.wgsl" \
        --glsl "./geometric-alg/src/glsl/${name}.glsl" \
        -- "$file"
done
