#![allow(clippy::type_complexity)]

mod algebra;
mod ast;
mod emitter;

pub use crate::algebra::{GeometricAlgeberaRecord, GeometricAlgebraBuilder};
pub use emitter::*;
