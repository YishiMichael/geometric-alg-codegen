Get-ChildItem ./src/algebra/*.txt | ForEach-Object {
    $name = $_.BaseName

    ./target/debug/geometric-alg-codegen `
        --rust "./src/generated/rust/$name.rs" `
        --wgsl "./src/generated/wgsl/$name.wgsl" `
        --glsl "./src/generated/glsl/$name.glsl" `
        -- "./src/algebra/$name.txt"
}
