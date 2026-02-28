use geometric_alg_codegen::*;
use std::io::BufWriter;

macro_rules! algebra {
    ($e0_squared:expr, $e1_squared:expr) => {
        algebra_def! {
            s := 1,
            e0 := e0,
            e1 := e1,
            e01 := e0 * e1,
            e0 ^ 2 = $e0_squared,
            e1 ^ 2 = $e1_squared,
        }
    };
    ($e0_squared:expr, $e1_squared:expr, $e2_squared:expr) => {
        algebra_def! {
            s := 1,
            e0 := e0,
            e1 := e1,
            e2 := e2,
            e01 := e0 * e1,
            e20 := e2 * e0,
            e12 := e1 * e2,
            e012 := e0 * e1 * e2,
            e0 ^ 2 = $e0_squared,
            e1 ^ 2 = $e1_squared,
            e2 ^ 2 = $e2_squared,
        }
    };
    ($e0_squared:expr, $e1_squared:expr, $e2_squared:expr, $e3_squared:expr) => {
        algebra_def! {
            s := 1,
            e0 := e0,
            e1 := e1,
            e2 := e2,
            e3 := e3,
            e01 := e0 * e1,
            e02 := e0 * e2,
            e03 := e0 * e3,
            e12 := e1 * e2,
            e31 := e3 * e1,
            e23 := e2 * e3,
            e021 := e0 * e2 * e1,
            e013 := e0 * e1 * e3,
            e032 := e0 * e3 * e2,
            e123 := e1 * e2 * e3,
            e0123 := e0 * e1 * e2 * e3,
            e0 ^ 2 = $e0_squared,
            e1 ^ 2 = $e1_squared,
            e2 ^ 2 = $e2_squared,
            e3 ^ 2 = $e3_squared,
        }
    };
}

macro_rules! algebra_def {
    ($($input:tt)*) => {
        algebra_def_impl!([] [] $($input)*)
    };
}

macro_rules! algebra_def_impl {
    ([$($generator:expr,)*] $blades:tt $generator_name:ident ^ 2 = $generator_square:expr, $($rest:tt)*) => {
        algebra_def_impl!([$($generator,)* (stringify!($generator_name), $generator_square),] $blades $($rest)*)
    };
    ($generators:tt [$($blade:expr,)*] $name:ident := 1, $($rest:tt)*) => {
        algebra_def_impl!($generators [$($blade,)* (stringify!($name), vec![]),] $($rest)*)
    };
    ($generators:tt [$($blade:expr,)*] $name:ident := $head:ident $(* $tail:ident)*, $($rest:tt)*) => {
        algebra_def_impl!($generators [$($blade,)* (stringify!($name), vec![stringify!($head) $(, stringify!($tail))*]),] $($rest)*)
    };
    ($generators:tt $blades:tt) => {
        GeometricAlgebraRecord::new(
            $generators.into(),
            $blades.into(),
        )
    };
}

fn main() -> std::io::Result<()> {
    for (name, alg) in [
        ("epga1d", algebra!(-1, 1)),
        ("ppga1d", algebra!(0, 1)),
        ("hpga1d", algebra!(1, 1)),
        ("epga2d", algebra!(-1, 1, 1)),
        ("ppga2d", algebra!(0, 1, 1)),
        ("hpga2d", algebra!(1, 1, 1)),
        ("epga3d", algebra!(-1, 1, 1, 1)),
        ("ppga3d", algebra!(0, 1, 1, 1)),
        ("hpga3d", algebra!(1, 1, 1, 1)),
    ] {
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
