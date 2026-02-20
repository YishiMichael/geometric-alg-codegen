use geometric_alg_codegen::*;
use std::io::BufWriter;

struct GeometricAlgebraItem {
    name: &'static str,
    generator_square_signs: &'static str,
    blades: &'static str,
}

const ITEMS: &[GeometricAlgebraItem] = &[
    GeometricAlgebraItem {
        name: "epga1d",
        generator_square_signs: "- +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraItem {
        name: "ppga1d",
        generator_square_signs: "0 +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraItem {
        name: "hpga1d",
        generator_square_signs: "+ +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraItem {
        name: "epga2d",
        generator_square_signs: "- + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraItem {
        name: "ppga2d",
        generator_square_signs: "0 + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraItem {
        name: "hpga2d",
        generator_square_signs: "+ + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraItem {
        name: "epga3d",
        generator_square_signs: "- + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraItem {
        name: "ppga3d",
        generator_square_signs: "0 + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraItem {
        name: "hpga3d",
        generator_square_signs: "+ + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
];

fn main() -> std::io::Result<()> {
    for GeometricAlgebraItem {
        name,
        generator_square_signs,
        blades,
    } in ITEMS
    {
        let alg = GeometricAlgebraBuilder::new(
            generator_square_signs.split_whitespace().map(|s| match s {
                "+" => 1,
                "0" => 0,
                "-" => -1,
                _ => unreachable!(),
            }),
            blades.split_whitespace(),
        )
        .build();
        alg.write(
            RustLang,
            &mut BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/rust/{name}.rs"
            ))?),
            "f32",
        )?;
        alg.write(
            WGSLLang,
            &mut BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/wgsl/{name}.wgsl"
            ))?),
            "f32",
        )?;
        alg.write(
            GLSLLang,
            &mut BufWriter::new(std::fs::File::create(format!(
                "geometric-alg/src/glsl/{name}.glsl"
            ))?),
            "float",
        )?;
    }
    Ok(())
}
