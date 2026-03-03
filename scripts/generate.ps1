mkdir -Force ./geometric-alg/src/rust | Out-Null
mkdir -Force ./geometric-alg/src/wgsl | Out-Null
mkdir -Force ./geometric-alg/src/glsl | Out-Null

Get-ChildItem ./scripts/algebra/*.txt | ForEach-Object {
    $name = $_.BaseName

    ./target/debug/geometric-alg-codegen `
        --rust "./geometric-alg/src/rust/$name.rs" `
        --wgsl "./geometric-alg/src/wgsl/$name.wgsl" `
        --glsl "./geometric-alg/src/glsl/$name.glsl" `
        -- "./scripts/algebra/$name.txt"
}
