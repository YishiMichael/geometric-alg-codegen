mod rust;

use crate::ast::{Expr, ExprRepr, Implementation, Items, Operation, Stmt, StmtRepr, Struct};
use std::cell::RefCell;
use std::io::Write;

pub trait Emitter<W> {
    fn emit_preamble(&self, _writer: &mut Writer<W>) {}

    fn emit_struct(&self, _writer: &mut Writer<W>, _struct: Struct) {}

    fn emit_operation(&self, _writer: &mut Writer<W>, _operation: Operation) {}

    fn emit_implementation(&self, _writer: &mut Writer<W>, _implementation: Implementation) {}

    fn emit_expr_repr(&self, _writer: &mut Writer<W>, _expr_repr: ExprRepr) {}

    fn emit_stmt_repr(&self, _writer: &mut Writer<W>, _stmt_repr: StmtRepr) {}

    fn emit_expr(&self, writer: &mut Writer<W>, expr: Expr) {
        self.emit_expr_repr(writer, *expr.repr);
    }

    fn emit_stmt(&self, writer: &mut Writer<W>, stmt: Stmt) {
        self.emit_stmt_repr(writer, *stmt.repr);
    }

    fn emit_items(&self, writer: &mut Writer<W>, items: Items) {
        self.emit_preamble(writer);
        items
            .structs
            .into_iter()
            .for_each(|r#struct| self.emit_struct(writer, r#struct));
        items
            .operations
            .into_iter()
            .for_each(|operation| self.emit_operation(writer, operation));
        items
            .implementations
            .into_iter()
            .for_each(|implementation| self.emit_implementation(writer, implementation));
    }
}

pub struct IndentGuard<'w> {
    indents: &'w RefCell<usize>,
}

impl<'w> From<&'w RefCell<usize>> for IndentGuard<'w> {
    fn from(indents: &'w RefCell<usize>) -> Self {
        *indents.borrow_mut() += 1;
        Self { indents }
    }
}

impl Drop for IndentGuard<'_> {
    fn drop(&mut self) {
        *self.indents.borrow_mut() -= 1;
    }
}

pub struct Writer<W> {
    buffer: W,
    indents: RefCell<usize>,
}

impl<W: Write> Writer<W> {
    pub fn indent(&self) -> IndentGuard {
        IndentGuard::from(&self.indents)
    }

    pub fn write(&mut self, s: impl std::fmt::Display) {
        write!(self.buffer, "{s}");
    }

    pub fn newline(&mut self) {
        writeln!(self.buffer);
        for _ in 0..(*self.indents.borrow()) {
            write!(self.buffer, "    ");
        }
    }
}
