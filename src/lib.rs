#![allow(unused)]

mod algebra;
mod ast;
mod emitter;
mod traits;

#[test]
fn generate() -> std::io::Result<()> {
    use algebra::GeometricAlgebraDimension;
    use emitter::rust::RustLang;
    use emitter::{Emitter, EmitterTrait};

    let d2 = GeometricAlgebraDimension::new(2, "s e0 e1 e01");
    for (name, generator_squares) in [("epga1d", [-1, 1]), ("ppga1d", [0, 1]), ("hpga1d", [1, 1])] {
        let items = d2.items(generator_squares);
        let mut emitter = Emitter::new(std::fs::File::create(format!("src/{name}.rs"))?, RustLang);
        emitter.emit_items(items)?;
    }
    let d3 = GeometricAlgebraDimension::new(3, "s e0 e1 e01 e2 -e20 e12 e012");
    for (name, generator_squares) in [
        ("epga2d", [-1, 1, 1]),
        ("ppga2d", [0, 1, 1]),
        ("hpga2d", [1, 1, 1]),
    ] {
        let items = d3.items(generator_squares);
        let mut emitter = Emitter::new(std::fs::File::create(format!("src/{name}.rs"))?, RustLang);
        emitter.emit_items(items)?;
    }
    let d4 = GeometricAlgebraDimension::new(
        4,
        "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    );
    for (name, generator_squares) in [
        ("epga3d", [-1, 1, 1, 1]),
        ("ppga3d", [0, 1, 1, 1]),
        ("hpga3d", [1, 1, 1, 1]),
    ] {
        let items = d3.items(generator_squares);
        let mut emitter = Emitter::new(std::fs::File::create(format!("src/{name}.rs"))?, RustLang);
        emitter.emit_items(items)?;
    }
    Ok(())
}
