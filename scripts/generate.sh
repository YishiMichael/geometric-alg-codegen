#!/usr/bin/env bash

mkdir -p ./geometric-alg/src/generated/rust
mkdir -p ./geometric-alg/src/generated/wgsl
mkdir -p ./geometric-alg/src/generated/glsl

for file in ./geometric-alg/src/algebra/*.txt; do
    name="$(basename "$file" .txt)"

    ./target/debug/geometric-alg-codegen \
        --rust "./geometric-alg/src/generated/rust/${name}.rs" \
        --wgsl "./geometric-alg/src/generated/wgsl/${name}.wgsl" \
        --glsl "./geometric-alg/src/generated/glsl/${name}.glsl" \
        -- "$file"
done
