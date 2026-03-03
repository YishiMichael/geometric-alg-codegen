mkdir -Force ./geometric-alg/src/generated/rust | Out-Null
mkdir -Force ./geometric-alg/src/generated/wgsl | Out-Null
mkdir -Force ./geometric-alg/src/generated/glsl | Out-Null

Get-ChildItem ./geometric-alg/src/algebra/*.txt | ForEach-Object {
    $name = $_.BaseName

    ./target/debug/geometric-alg-codegen `
        --rust "./geometric-alg/src/generated/rust/$name.rs" `
        --wgsl "./geometric-alg/src/generated/wgsl/$name.wgsl" `
        --glsl "./geometric-alg/src/generated/glsl/$name.glsl" `
        -- "./geometric-alg/src/algebra/$name.txt"
}
