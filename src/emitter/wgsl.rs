use crate::ast::{ExprRepr, Implementation, Ownership, StmtRepr, Structure};
use crate::emitter::{Emitter, EmitterTrait};
use itertools::{Itertools, Position};
use std::io::Write;
use symbol::Symbol;

pub struct WGSLLang;

impl<Buffer> EmitterTrait for Emitter<Buffer, WGSLLang>
where
    Buffer: Write,
{
    fn emit_preamble(&mut self) -> std::io::Result<()> {
        write!(self, "// Automatically generated")?;
        self.newline()?;
        Ok(())
    }

    fn emit_structure(&mut self, Structure { ident, fields }: &Structure) -> std::io::Result<()> {
        self.newline()?;
        write!(self, "struct {ident} {{")?;
        {
            self.indent();
            for (field_ident, field_type) in fields {
                self.newline()?;
                write!(self, "{field_ident}: {field_type},")?;
            }
            if fields.is_empty() {
                self.newline()?;
                write!(self, "_phantom: f32,")?;
            }
            self.dedent();
        }
        self.newline()?;
        write!(self, "}}")?;
        self.newline()?;
        Ok(())
    }

    fn emit_implementation(
        &mut self,
        Implementation {
            ident: _,
            op_ident,
            self_type,
            generic_items,
            associate_items: _,
            self_param_item,
            param_items,
            return_type,
            body,
        }: &Implementation,
    ) -> std::io::Result<()> {
        self.newline()?;
        write!(
            self,
            "fn {fn_ident}({params}){return_type} {{",
            fn_ident = mangle(
                op_ident,
                self_type,
                generic_items.iter().map(|(_, generic_type)| generic_type),
            ),
            params = self_param_item
                .iter()
                .chain(param_items)
                .map(|(param_ident, param_type)| match param_type {
                    Ownership::Owned(param_type) => format!("{param_ident}: {param_type}"),
                    Ownership::BorrowedMut(param_type) =>
                        format!("{param_ident}: ptr<function, {param_type}>"),
                })
                .join(", "),
            return_type = return_type
                .map(|return_type| format!(" -> {return_type}"))
                .unwrap_or_default(),
        )?;
        {
            self.indent();
            for stmt in &body.stmts {
                self.newline()?;
                self.emit_stmt(stmt)?;
            }
            if let Some(expr) = body.expr.as_ref() {
                self.newline()?;
                write!(self, "return ")?;
                self.emit_expr(expr)?;
                write!(self, ";")?;
            }
            self.dedent();
        }
        self.newline()?;
        write!(self, "}}")?;
        self.newline()?;
        Ok(())
    }

    fn emit_expr_repr(&mut self, expr_repr: &ExprRepr) -> std::io::Result<()> {
        match expr_repr {
            ExprRepr::Variable { ident } => {
                write!(self, "{ident}")?;
            }
            ExprRepr::Literal { value } => {
                write!(self, "{value}.0")?;
            }
            ExprRepr::Struct { ident, fields } => {
                write!(self, "{ident} {{")?;
                {
                    self.indent();
                    for (field_ident, field_expr) in fields {
                        self.newline()?;
                        write!(self, "{field_ident}: ")?;
                        self.emit_expr(field_expr)?;
                        write!(self, ",")?;
                    }
                    if fields.is_empty() {
                        self.newline()?;
                        write!(self, "_phantom: 0.0,")?;
                    }
                    self.dedent();
                }
                self.newline()?;
                write!(self, "}}")?;
            }
            ExprRepr::Field { expr, ident } => {
                self.emit_expr(expr)?;
                write!(self, ".{ident}")?;
            }
            ExprRepr::CallBuiltin {
                ident: _,
                op_ident,
                self_expr,
            } => {
                write!(self, "{op_ident}(")?;
                self.emit_expr(self_expr)?;
                write!(self, ")")?;
            }
            ExprRepr::CallFunction {
                ident: _,
                op_ident,
                self_type,
                generic_types,
                param_exprs,
            } => {
                write!(
                    self,
                    "{fn_ident}(",
                    fn_ident = mangle(op_ident, self_type, generic_types),
                )?;
                for (position, param_expr) in param_exprs.iter().with_position() {
                    self.emit_expr(param_expr)?;
                    if matches!(position, Position::First | Position::Middle) {
                        write!(self, ", ")?;
                    }
                }
                write!(self, ")")?;
            }
            ExprRepr::CallMethod {
                ident: _,
                op_ident,
                self_type,
                generic_types,
                self_expr,
                param_exprs,
            } => {
                write!(
                    self,
                    "{fn_ident}(",
                    fn_ident = mangle(op_ident, self_type, generic_types),
                )?;
                for (position, param_expr) in std::iter::once(self_expr)
                    .chain(param_exprs.iter())
                    .with_position()
                {
                    self.emit_expr(param_expr)?;
                    if matches!(position, Position::First | Position::Middle) {
                        write!(self, ", ")?;
                    }
                }
                write!(self, ")")?;
            }
            ExprRepr::Neg { expr } => {
                write!(self, "-")?;
                self.emit_expr(expr)?;
            }
            ExprRepr::Add { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " + ")?;
                self.emit_expr(rhs)?;
            }
            ExprRepr::Sub { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " - ")?;
                self.emit_expr(rhs)?;
            }
            ExprRepr::Mul { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " * ")?;
                self.emit_expr(rhs)?;
            }
            ExprRepr::Div { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " / ")?;
                self.emit_expr(rhs)?;
            }
        }
        Ok(())
    }

    fn emit_stmt_repr(&mut self, stmt_repr: &StmtRepr) -> std::io::Result<()> {
        match stmt_repr {
            StmtRepr::Expr { expr } => {
                self.emit_expr(expr)?;
                write!(self, ";")?;
            }
            StmtRepr::AddAssign { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " += ")?;
                self.emit_expr(rhs)?;
                write!(self, ";")?;
            }
            StmtRepr::SubAssign { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " -= ")?;
                self.emit_expr(rhs)?;
                write!(self, ";")?;
            }
            StmtRepr::MulAssign { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " *= ")?;
                self.emit_expr(rhs)?;
                write!(self, ";")?;
            }
            StmtRepr::DivAssign { lhs, rhs } => {
                self.emit_expr(lhs)?;
                write!(self, " /= ")?;
                self.emit_expr(rhs)?;
                write!(self, ";")?;
            }
        }
        Ok(())
    }
}

fn mangle<'s>(
    op_ident: &'s Symbol,
    self_type: &'s Symbol,
    generic_types: impl IntoIterator<Item = &'s Symbol>,
) -> String {
    [self_type, op_ident]
        .into_iter()
        .chain(generic_types)
        .join("_")
        .to_lowercase()
}
