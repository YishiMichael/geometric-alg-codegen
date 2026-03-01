use geometric_alg_codegen::{GeometricAlgebraRecord, Syntax};
use std::env;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::Command;

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
    ($generators:tt [$($blade:expr,)*] $blade_name:ident := 1, $($rest:tt)*) => {
        algebra_def_impl!($generators [$($blade,)* (stringify!($blade_name), vec![]),] $($rest)*)
    };
    ($generators:tt [$($blade:expr,)*] $blade_name:ident := $head:ident $(* $tail:ident)*, $($rest:tt)*) => {
        algebra_def_impl!($generators [$($blade,)* (stringify!($blade_name), vec![stringify!($head) $(, stringify!($tail))*]),] $($rest)*)
    };
    ($generators:tt $blades:tt) => {
        GeometricAlgebraRecord::new(
            $generators.into(),
            $blades.into(),
        )
    };
}

fn emit_algebras(items: &[(&str, GeometricAlgebraRecord<&str>)]) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let out_dir = Path::new(&out_dir);

    let mut rust_file = BufWriter::new(fs::File::create(out_dir.join("rust.rs"))?);
    let rust_dir = out_dir.join("rust");
    let wgsl_dir = out_dir.join("wgsl");
    let glsl_dir = out_dir.join("glsl");
    fs::create_dir_all(&rust_dir)?;
    fs::create_dir_all(&wgsl_dir)?;
    fs::create_dir_all(&glsl_dir)?;

    for (name, alg) in items {
        let path = rust_dir.join(name).with_extension("rs");
        alg.emit(
            &mut BufWriter::new(fs::File::create(&path)?),
            Syntax::Rust,
            "f32",
        )?;
        writeln!(
            &mut rust_file,
            r#"pub mod {name} {{ include!(concat!(env!("OUT_DIR"), "/rust/{name}.rs")); }}"#
        )?;

        let path = wgsl_dir.join(name).with_extension("wgsl");
        alg.emit(
            &mut BufWriter::new(fs::File::create(&path)?),
            Syntax::Wgsl,
            "f32",
        )?;
        if !Command::new("naga").arg(&path).status()?.success() {
            panic!("WGSL validation failed");
        }

        let path = glsl_dir.join(name).with_extension("glsl");
        alg.emit(
            &mut BufWriter::new(fs::File::create(&path)?),
            Syntax::Glsl,
            "float",
        )?;
        if !Command::new("glslangValidator")
            .args(["-S", "vert"])
            .arg(&path)
            .status()?
            .success()
        {
            panic!("GLSL validation failed");
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    emit_algebras(&[
        ("epga1d", algebra!(-1, 1)),
        ("ppga1d", algebra!(0, 1)),
        ("hpga1d", algebra!(1, 1)),
        ("epga2d", algebra!(-1, 1, 1)),
        ("ppga2d", algebra!(0, 1, 1)),
        ("hpga2d", algebra!(1, 1, 1)),
        ("epga3d", algebra!(-1, 1, 1, 1)),
        ("ppga3d", algebra!(0, 1, 1, 1)),
        ("hpga3d", algebra!(1, 1, 1, 1)),
    ])
}
