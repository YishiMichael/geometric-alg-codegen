use crate::ast::{ExprRepr, Implementation, Ownership, StmtRepr, Structure};
use crate::emitter::{Emitter, EmitterTrait};
use itertools::{Itertools, Position};
use std::io::Write;

pub struct RustLang;

impl<Buffer> EmitterTrait for Emitter<Buffer, RustLang>
where
    Buffer: Write,
{
    fn emit_preamble(&mut self) -> std::io::Result<()> {
        write!(self, "// Automatically generated")?;
        self.newline()?;
        write!(self, "use crate::traits::*;")?;
        self.newline()?;
        Ok(())
    }

    fn emit_structure(&mut self, Structure { ident, fields }: Structure) -> std::io::Result<()> {
        self.newline()?;
        write!(self, "#[derive(Clone, Copy, Debug, Default)]")?;
        self.newline()?;
        if fields.is_empty() {
            write!(self, "pub struct {ident};")?;
        } else {
            write!(self, "pub struct {ident} {{")?;
            {
                self.indent();
                for (field_ident, field_type) in fields {
                    self.newline()?;
                    write!(self, "pub {field_ident}: {field_type},")?;
                }
                self.dedent();
            }
            self.newline()?;
            write!(self, "}}")?;
        }
        self.newline()?;
        Ok(())
    }

    fn emit_implementation(
        &mut self,
        Implementation {
            ident,
            op_ident,
            self_type,
            generic_items,
            associate_items,
            self_param_item,
            param_items,
            return_type,
            body,
        }: Implementation,
    ) -> std::io::Result<()> {
        self.newline()?;
        write!(
            self,
            "impl {ident}{generics} for {self_type} {{",
            generics = if generic_items.is_empty() {
                String::new()
            } else {
                format!(
                    "<{generics}>",
                    generics = generic_items
                        .into_iter()
                        .map(|(_, generic_type)| generic_type.as_str())
                        .join(", "),
                )
            },
        )?;
        {
            self.indent();
            for (associate_ident, associate_type) in associate_items {
                self.newline()?;
                write!(self, "type {associate_ident} = {associate_type};")?;
            }
            self.newline()?;
            write!(
                self,
                "fn {op_ident}({params}){return_type} {{",
                params = self_param_item
                    .into_iter()
                    .map(
                        |(self_param_ident, self_param_type)| match self_param_type {
                            Ownership::Owned(_) => format!("{self_param_ident}"),
                            Ownership::BorrowedMut(_) => format!("&mut {self_param_ident}"),
                        }
                    )
                    .chain(
                        param_items
                            .into_iter()
                            .map(|(param_ident, param_type)| format!(
                                "{param_ident}: {param_type}",
                                param_type = match param_type {
                                    Ownership::Owned(param_type) => format!("{param_type}"),
                                    Ownership::BorrowedMut(param_type) =>
                                        format!("&mut {param_type}"),
                                },
                            ))
                    )
                    .join(", "),
                return_type = return_type
                    .map(|return_type| format!(" -> {return_type}"))
                    .unwrap_or_default(),
            )?;
            {
                self.indent();
                for stmt in body.stmts {
                    self.newline()?;
                    self.emit_stmt(stmt)?;
                }
                if let Some(expr) = body.expr {
                    self.newline()?;
                    self.emit_expr(expr)?;
                }
                self.dedent();
            }
            self.newline()?;
            write!(self, "}}")?;
            self.dedent();
        }
        self.newline()?;
        write!(self, "}}")?;
        self.newline()?;
        Ok(())
    }

    fn emit_expr_repr(&mut self, expr_repr: ExprRepr) -> std::io::Result<()> {
        match expr_repr {
            ExprRepr::Variable { ident } => {
                write!(self, "{ident}")?;
            }
            ExprRepr::Literal { value } => {
                write!(self, "{value}.0")?;
            }
            ExprRepr::Struct { ident, fields } => {
                if fields.is_empty() {
                    write!(self, "{ident}")?;
                } else {
                    write!(self, "{ident} {{")?;
                    {
                        self.indent();
                        for (field_ident, field_expr) in fields {
                            self.newline()?;
                            write!(self, "{field_ident}: ")?;
                            self.emit_expr(field_expr)?;
                            write!(self, ",")?;
                        }
                        self.dedent();
                    }
                    self.newline()?;
                    write!(self, "}}")?;
                }
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
                self.emit_expr(self_expr)?;
                write!(self, ".{op_ident}()")?;
            }
            ExprRepr::CallFunction {
                ident: _,
                op_ident,
                self_type,
                generic_types: _,
                param_exprs,
            } => {
                write!(self, "{self_type}::{op_ident}(")?;
                for (position, param_expr) in param_exprs.into_iter().with_position() {
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
                self_type: _,
                generic_types: _,
                self_expr,
                param_exprs,
            } => {
                self.emit_expr(self_expr)?;
                write!(self, ".{op_ident}(")?;
                for (position, param_expr) in param_exprs.into_iter().with_position() {
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

    fn emit_stmt_repr(&mut self, stmt_repr: StmtRepr) -> std::io::Result<()> {
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
