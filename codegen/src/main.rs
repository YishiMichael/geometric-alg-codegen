#![allow(dead_code)]
#![allow(clippy::type_complexity)]

mod algebra;
mod ast;
mod syntax;

use crate::algebra::NamedGeometricAlgebra;
use crate::syntax::Syntax;
use clap::Parser;
use clap_stdin::FileOrStdin;
use std::path::PathBuf;

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
