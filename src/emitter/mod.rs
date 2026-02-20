mod glsl;
mod rust;
mod wgsl;

pub use glsl::GLSLLang;
pub use rust::RustLang;
pub use wgsl::WGSLLang;

use crate::ast::{Ast, Expr, Implementation, Record, Stmt, Stringifier, Structure};
use std::io::Write;

pub trait Emitter {
    fn emit_preamble(&self, writer: &mut Writer) -> std::io::Result<()>;
    fn emit_structure<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        structure: &Structure<A>,
    ) -> std::io::Result<()>;
    fn emit_implementation<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        implementation: &Implementation<A>,
    ) -> std::io::Result<()>;
    fn emit_expr<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        expr_repr: &Expr<A>,
    ) -> std::io::Result<()>;
    fn emit_stmt<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        stmt_repr: &Stmt<A>,
    ) -> std::io::Result<()>;

    fn emit_record<A: Ast>(
        &self,
        writer: &mut Writer,
        stringifier: &dyn Stringifier<A>,
        record: &Record<A>,
    ) -> std::io::Result<()> {
        self.emit_preamble(writer)?;
        for structure in &record.structures {
            self.emit_structure(writer, stringifier, structure)?;
        }
        for implementation in &record.implementations {
            self.emit_implementation(writer, stringifier, implementation)?;
        }
        Ok(())
    }
}

pub struct Writer<'w> {
    buffer: &'w mut dyn Write,
    indents: usize,
}

impl<'w> Writer<'w> {
    pub fn new(buffer: &'w mut dyn Write) -> Self {
        Self { buffer, indents: 0 }
    }

    pub fn buffer(&mut self) -> &mut dyn Write {
        &mut self.buffer
    }
}

impl Writer<'_> {
    pub fn indent(&mut self) {
        self.indents += 1;
    }

    pub fn dedent(&mut self) {
        self.indents -= 1;
    }

    pub fn newline(&mut self) -> std::io::Result<()> {
        writeln!(self.buffer())?;
        let indents = self.indents;
        write!(self.buffer(), "{:width$}", "", width = 4 * indents)?;
        Ok(())
    }
}
