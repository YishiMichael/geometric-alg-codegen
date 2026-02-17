use std::io::BufWriter;

use geometric_alg_codegen::*;

fn main() -> std::io::Result<()> {
    let d2 = GeometricAlgebraDimension::new(2, "s e0 e1 e01".split_whitespace());
    let d3 = GeometricAlgebraDimension::new(3, "s e0 e1 e01 e2 -e20 e12 e012".split_whitespace());
    let d4 = GeometricAlgebraDimension::new(
        4,
        "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123".split_whitespace(),
    );
    for (name, items) in [
        ("epga1d", d2.items([-1, 1])),
        ("ppga1d", d2.items([0, 1])),
        ("hpga1d", d2.items([1, 1])),
        ("epga2d", d3.items([-1, 1, 1])),
        ("ppga2d", d3.items([0, 1, 1])),
        ("hpga2d", d3.items([1, 1, 1])),
        ("epga3d", d4.items([-1, 1, 1, 1])),
        ("ppga3d", d4.items([0, 1, 1, 1])),
        ("hpga3d", d4.items([1, 1, 1, 1])),
    ] {
        Emitter::new(
            BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/rust/{name}.rs"
            ))?),
            RustLang,
        )
        .emit_items(&items)?;
        Emitter::new(
            BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/wgsl/{name}.wgsl"
            ))?),
            WGSLLang,
        )
        .emit_items(&items)?;
        Emitter::new(
            BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/glsl/{name}.glsl"
            ))?),
            GLSLLang,
        )
        .emit_items(&items)?;
    }

    Ok(())
}
