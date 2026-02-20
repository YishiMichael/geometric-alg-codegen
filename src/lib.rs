#![allow(clippy::type_complexity)]

mod algebra;
mod ast;
mod syntax;

pub use crate::algebra::{GeometricAlgeberaRecord, GeometricAlgebraBuilder};
pub use syntax::*;
