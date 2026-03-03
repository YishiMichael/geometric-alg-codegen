#!/usr/bin/env bash

for file in ./geometric-alg/src/algebra/*.txt; do
    name="$(basename "$file" .txt)"

    ./target/debug/geometric-alg-codegen \
        --rust "./geometric-alg/src/generated/rust/${name}.rs" \
        --wgsl "./geometric-alg/src/generated/wgsl/${name}.wgsl" \
        --glsl "./geometric-alg/src/generated/glsl/${name}.glsl" \
        -- "$file"
done
