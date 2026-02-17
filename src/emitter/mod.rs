pub mod rust;

use crate::ast::{Expr, ExprRepr, Implementation, Items, Stmt, StmtRepr, Structure};
use std::io::Write;

pub trait EmitterTrait {
    fn emit_preamble(&mut self) -> std::io::Result<()>;

    fn emit_structure(&mut self, _structure: Structure) -> std::io::Result<()>;

    fn emit_implementation(&mut self, _implementation: Implementation) -> std::io::Result<()>;

    fn emit_expr_repr(&mut self, _expr_repr: ExprRepr) -> std::io::Result<()>;

    fn emit_stmt_repr(&mut self, _stmt_repr: StmtRepr) -> std::io::Result<()>;

    fn emit_expr(&mut self, expr: Expr) -> std::io::Result<()> {
        self.emit_expr_repr(*expr.repr)
    }

    fn emit_stmt(&mut self, stmt: Stmt) -> std::io::Result<()> {
        self.emit_stmt_repr(*stmt.repr)
    }

    fn emit_items(&mut self, items: Items) -> std::io::Result<()> {
        self.emit_preamble()?;
        for structure in items.structures {
            self.emit_structure(structure)?;
        }
        for implementation in items.implementations {
            self.emit_implementation(implementation)?;
        }
        Ok(())
    }
}

pub struct Emitter<Buffer, Lang> {
    buffer: Buffer,
    lang: Lang,
    indents: usize,
}

impl<Buffer, Lang> Emitter<Buffer, Lang> {
    pub fn new(buffer: Buffer, lang: Lang) -> Self {
        Self {
            buffer,
            lang,
            indents: 0,
        }
    }
}

impl<Buffer, Lang> std::ops::Deref for Emitter<Buffer, Lang> {
    type Target = Buffer;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl<Buffer, Lang> std::ops::DerefMut for Emitter<Buffer, Lang> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer
    }
}

impl<Buffer, Lang> Emitter<Buffer, Lang>
where
    Buffer: Write,
{
    pub fn indent(&mut self) {
        self.indents += 1;
    }

    pub fn dedent(&mut self) {
        self.indents -= 1;
    }

    pub fn newline(&mut self) -> std::io::Result<()> {
        writeln!(self)?;
        let indents = self.indents;
        write!(self, "{:width$}", "", width = 4 * indents)?;
        Ok(())
    }
}
