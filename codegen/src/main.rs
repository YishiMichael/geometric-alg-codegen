#![allow(clippy::type_complexity)]

mod algebra;
mod ast;
mod syntax;

use crate::algebra::NamedGeometricAlgebra;
use crate::syntax::Syntax;
use clap::Parser;
use clap_stdin::FileOrStdin;
use std::path::PathBuf;

// macro_rules! algebra {
//     ($e0_squared:expr, $e1_squared:expr) => {
//         algebra_def! {
//             s := 1,
//             e0 := e0,
//             e1 := e1,
//             e01 := e0 * e1,
//             e0 ^ 2 = $e0_squared,
//             e1 ^ 2 = $e1_squared,
//         }
//     };
//     ($e0_squared:expr, $e1_squared:expr, $e2_squared:expr) => {
//         algebra_def! {
//             s := 1,
//             e0 := e0,
//             e1 := e1,
//             e2 := e2,
//             e01 := e0 * e1,
//             e20 := e2 * e0,
//             e12 := e1 * e2,
//             e012 := e0 * e1 * e2,
//             e0 ^ 2 = $e0_squared,
//             e1 ^ 2 = $e1_squared,
//             e2 ^ 2 = $e2_squared,
//         }
//     };
//     ($e0_squared:expr, $e1_squared:expr, $e2_squared:expr, $e3_squared:expr) => {
//         algebra_def! {
//             s := 1,
//             e0 := e0,
//             e1 := e1,
//             e2 := e2,
//             e3 := e3,
//             e01 := e0 * e1,
//             e02 := e0 * e2,
//             e03 := e0 * e3,
//             e12 := e1 * e2,
//             e31 := e3 * e1,
//             e23 := e2 * e3,
//             e021 := e0 * e2 * e1,
//             e013 := e0 * e1 * e3,
//             e032 := e0 * e3 * e2,
//             e123 := e1 * e2 * e3,
//             e0123 := e0 * e1 * e2 * e3,
//             e0 ^ 2 = $e0_squared,
//             e1 ^ 2 = $e1_squared,
//             e2 ^ 2 = $e2_squared,
//             e3 ^ 2 = $e3_squared,
//         }
//     };
// }

// macro_rules! algebra_def {
//     ($($input:tt)*) => {
//         algebra_def_impl!([] [] $($input)*)
//     };
// }

// macro_rules! algebra_def_impl {
//     ([$($generator:expr,)*] $blades:tt $generator_name:ident ^ 2 = $generator_square:expr, $($rest:tt)*) => {
//         algebra_def_impl!([$($generator,)* (stringify!($generator_name), $generator_square),] $blades $($rest)*)
//     };
//     ($generators:tt [$($blade:expr,)*] $blade_name:ident := 1, $($rest:tt)*) => {
//         algebra_def_impl!($generators [$($blade,)* (stringify!($blade_name), vec![]),] $($rest)*)
//     };
//     ($generators:tt [$($blade:expr,)*] $blade_name:ident := $head:ident $(* $tail:ident)*, $($rest:tt)*) => {
//         algebra_def_impl!($generators [$($blade,)* (stringify!($blade_name), vec![stringify!($head) $(, stringify!($tail))*]),] $($rest)*)
//     };
//     ($generators:tt $blades:tt) => {
//         GeometricAlgebraRecord::new(
//             $generators.into(),
//             $blades.into(),
//         )
//     };
// }

// fn emit_algebras(
//     items: &[(&str, GeometricAlgebraRecord<&str>)],
// ) -> Result<(), Box<dyn std::error::Error>> {
//     // let mut rust_file = std::io::BufWriter::new(std::fs::File::create("rust.rs")?);
//     let mut rust_file = std::fs::OpenOptions::new()
//         .append(true)
//         .open("rust.rs")
//         .ok()
//         .map(std::io::BufWriter::new);
//     let rust_dir = Some(std::path::Path::new("rust")).filter(|path| path.exists());
//     let wgsl_dir = Some(std::path::Path::new("wgsl")).filter(|path| path.exists());
//     let glsl_dir = Some(std::path::Path::new("glsl")).filter(|path| path.exists());
//     // std::fs::create_dir_all(rust_dir)?;
//     // std::fs::create_dir_all(wgsl_dir)?;
//     // std::fs::create_dir_all(glsl_dir)?;

//     for (name, alg) in items {
//         // let path = rust_dir.join(name).with_extension("rs");
//         if let Some(rust_file) = rust_file.as_mut() {
//             writeln!(rust_file as &mut dyn std::io::Write, "pub mod {name};")?;
//         }

//         if let Some(rust_dir) = rust_dir {
//             alg.emit(
//                 &mut std::io::BufWriter::new(std::fs::File::create(
//                     rust_dir.join(name).with_extension("rs"),
//                 )?),
//                 Syntax::Rust,
//                 "f32",
//             )?;
//         }

//         // writeln!(
//         //     &mut rust_file as &mut dyn std::io::Write,
//         //     r#"pub mod {name} {{ include!(concat!(env!("OUT_DIR"), "/rust/{name}.rs")); }}"#
//         // )?;

//         // let path = wgsl_dir.join(name).with_extension("wgsl");
//         if let Some(wgsl_dir) = wgsl_dir {
//             alg.emit(
//                 &mut std::io::BufWriter::new(std::fs::File::create(
//                     wgsl_dir.join(name).with_extension("wgsl"),
//                 )?),
//                 Syntax::Wgsl,
//                 "f32",
//             )?;
//         }

//         // let path = glsl_dir.join(name).with_extension("glsl");
//         if let Some(glsl_dir) = glsl_dir {
//             alg.emit(
//                 &mut std::io::BufWriter::new(std::fs::File::create(
//                     glsl_dir.join(name).with_extension("glsl"),
//                 )?),
//                 Syntax::Glsl,
//                 "float",
//             )?;
//         }
//     }
//     Ok(())
// }

// if !std::process::Command::new("naga")
//     .arg(&path)
//     .status()?
//     .success()
// {
//     panic!("WGSL validation failed");
// }

// if !std::process::Command::new("glslangValidator")
//     .args(["-S", "vert"])
//     .arg(&path)
//     .status()?
//     .success()
// {
//     panic!("GLSL validation failed");
// }

#[derive(Parser)]
#[command(version, about)]
struct Args {
    input: FileOrStdin<NamedGeometricAlgebra>,

    #[arg(long)]
    rust: Option<PathBuf>,

    #[arg(long)]
    rust_f16: Option<PathBuf>,

    #[arg(long)]
    rust_f64: Option<PathBuf>,

    #[arg(long)]
    rust_f128: Option<PathBuf>,

    #[arg(long)]
    wgsl: Option<PathBuf>,

    #[arg(long)]
    wgsl_f16: Option<PathBuf>,

    #[arg(long)]
    glsl: Option<PathBuf>,

    #[arg(long)]
    glsl_double: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::try_parse()?;
    let alg = args.input.contents()?;
    let record = alg.record();

    if let Some(rust) = args.rust.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(rust)?),
            Syntax::Rust,
            &"f32",
            &"",
        )?;
    }
    if let Some(rust_f16) = args.rust_f16.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(rust_f16)?),
            Syntax::Rust,
            &"f16",
            &"_f16",
        )?;
    }
    if let Some(rust_f64) = args.rust_f64.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(rust_f64)?),
            Syntax::Rust,
            &"f64",
            &"_f64",
        )?;
    }
    if let Some(rust_f128) = args.rust_f128.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(rust_f128)?),
            Syntax::Rust,
            &"f128",
            &"_f128",
        )?;
    }
    if let Some(wgsl) = args.wgsl.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(wgsl)?),
            Syntax::Wgsl,
            &"f32",
            &"",
        )?;
    }
    if let Some(wgsl_f16) = args.wgsl_f16.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(wgsl_f16)?),
            Syntax::Wgsl,
            &"f16",
            &"h",
        )?;
    }
    if let Some(glsl) = args.glsl.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(glsl)?),
            Syntax::Glsl,
            &"float",
            &"",
        )?;
    }
    if let Some(glsl_double) = args.glsl_double.as_ref() {
        record.emit(
            &mut std::io::BufWriter::new(std::fs::File::create(glsl_double)?),
            Syntax::Glsl,
            &"double",
            &"lf",
        )?;
    }
    Ok(())
}
