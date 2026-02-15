use crate::ast::{ExprRepr, Implementation, Operation, Ownership, PretypeSymbol, StmtRepr, Struct};
use crate::emitter::{Emitter, Writer};
use itertools::Itertools;
use std::io::Write;
use symbol::Symbol;

struct RustEmitter;

impl<W> Emitter<W> for RustEmitter
where
    W: Write,
{
    fn emit_preamble(&self, writer: &mut Writer<W>) {
        writer.write("// Automatically generated");
        writer.newline();
    }

    fn emit_struct(&self, writer: &mut Writer<W>, Struct { ident, fields }: Struct) {
        writer.newline();
        writer.write(format!("struct {ident} {{"));
        {
            let _ = writer.indent();
            for (field_ident, field_type) in fields {
                writer.newline();
                writer.write(format!("{field_ident}: {field_type},"));
            }
        }
        writer.newline();
        writer.write(format!("}}"));
        writer.newline();
    }

    fn emit_operation(
        &self,
        writer: &mut Writer<W>,
        Operation {
            ident,
            op_ident,
            generics,
            associates,
            self_param_item,
            param_items,
            return_pretype,
        }: Operation,
    ) {
        fn format_pretype(pretype: Ownership<PretypeSymbol>) -> String {
            match pretype {
                Ownership::Owned(pretype_symbol) => format!(
                    "{}",
                    match pretype_symbol {
                        PretypeSymbol::SelfSymbol => format!("Self"),
                        PretypeSymbol::Generic(symbol) => format!("{symbol}"),
                        PretypeSymbol::Associate(symbol) => format!("Self::{symbol}"),
                        PretypeSymbol::Fixed(symbol) => format!("{symbol}"),
                    }
                ),
                Ownership::BorrowedMut(pretype_symbol) => format!(
                    "&mut {}",
                    match pretype_symbol {
                        PretypeSymbol::SelfSymbol => format!("Self"),
                        PretypeSymbol::Generic(symbol) => format!("{symbol}"),
                        PretypeSymbol::Associate(symbol) => format!("Self::{symbol}"),
                        PretypeSymbol::Fixed(symbol) => format!("{symbol}"),
                    }
                ),
            }
        }

        writer.newline();
        writer.write(format!(
            "trait {ident}{generics} {{",
            generics = if generics.is_empty() {
                format!("")
            } else {
                format!(
                    "<{generics}>",
                    generics = generics.into_iter().map(Symbol::as_str).join(", "),
                )
            },
        ));
        {
            let _ = writer.indent();
            for associate_ident in associates {
                writer.newline();
                writer.write(format!("type {associate_ident};"));
            }
            writer.newline();
            writer.write(format!(
                "fn {op_ident}({params}){return_pretype};",
                params = self_param_item
                    .into_iter()
                    .map(
                        |(self_param_ident, self_param_pretype)| match self_param_pretype {
                            Ownership::Owned(_) => format!("{self_param_ident}"),
                            Ownership::BorrowedMut(_) => format!("&mut {self_param_ident}"),
                        }
                    )
                    .chain(
                        param_items
                            .into_iter()
                            .map(|(param_ident, param_pretype)| format!(
                                "{param_ident}: {param_pretype}",
                                param_pretype = format_pretype(param_pretype),
                            ))
                    )
                    .join(", "),
                return_pretype = match return_pretype {
                    None => format!(""),
                    Some(return_pretype) => format!(
                        " -> {return_pretype}",
                        return_pretype = format_pretype(return_pretype),
                    ),
                },
            ))
        }
        writer.newline();
        writer.write(format!("}}"));
        writer.newline();
    }

    fn emit_implementation(
        &self,
        writer: &mut Writer<W>,
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
    ) {
        fn format_type(r#type: Ownership<Symbol>) -> String {
            match r#type {
                Ownership::Owned(type_symbol) => format!("{type_symbol}"),
                Ownership::BorrowedMut(type_symbol) => format!("&mut {type_symbol}"),
            }
        }

        writer.newline();
        writer.write(format!(
            "impl {ident}{generics} for {self_type} {{",
            generics = if generic_items.is_empty() {
                format!("")
            } else {
                format!(
                    "<{generics}>",
                    generics = generic_items
                        .into_iter()
                        .map(|(_, generic_type)| generic_type.as_str())
                        .join(", "),
                )
            },
        ));
        {
            let _ = writer.indent();
            for (associate_ident, associate_type) in associate_items {
                writer.newline();
                writer.write(format!("type {associate_ident} = {associate_type};"));
            }
            writer.newline();
            writer.write(format!(
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
                                param_type = format_type(param_type),
                            ))
                    )
                    .join(", "),
                return_type = match return_type {
                    None => format!(""),
                    Some(return_type) =>
                        format!(" -> {return_type}", return_type = format_type(return_type)),
                },
            ));
            {
                let _ = writer.indent();
                for stmt in body.stmts {
                    writer.newline();
                    self.emit_stmt(writer, stmt);
                }
                for expr in body.expr {
                    writer.newline();
                    self.emit_expr(writer, expr);
                }
            }
            writer.newline();
            writer.write(format!("}}"));
        }
        writer.newline();
        writer.write(format!("}}"));
        writer.newline();
    }

    fn emit_expr_repr(&self, writer: &mut Writer<W>, expr_repr: ExprRepr) {
        match expr_repr {
            ExprRepr::Variable { ident } => {
                writer.write(ident);
            }
            ExprRepr::Literal { value } => {
                writer.write(value);
            }
            ExprRepr::Struct { ident, fields } => {
                writer.write(format!("{ident} {{"));
                {
                    let _ = writer.indent();
                    for (field_ident, field_expr) in fields {
                        writer.newline();
                        writer.write(format!("{field_ident}: "));
                        self.emit_expr(writer, field_expr);
                        writer.write(",");
                    }
                    writer.newline();
                    writer.write(format!("}}"));
                }
            }
            ExprRepr::Field { expr, ident } => {
                self.emit_expr(writer, expr);
                writer.write(format!(".{ident}"));
            }
            ExprRepr::CallFunction {
                ident: _,
                op_ident,
                self_type,
                generic_types: _,
                param_exprs,
            } => {
                writer.write(format!("{self_type}::{op_ident}("));
                for param_expr in itertools::Itertools::intersperse_with(
                    param_exprs.into_iter().map(Some),
                    || None,
                ) {
                    match param_expr {
                        Some(param_expr) => {
                            self.emit_expr(writer, param_expr);
                        }
                        None => {
                            writer.write(", ");
                        }
                    }
                }
                writer.write(")");
            }
            ExprRepr::CallMethod {
                ident: _,
                op_ident,
                self_type: _,
                generic_types: _,
                self_expr,
                param_exprs,
            } => {
                self.emit_expr(writer, self_expr);
                writer.write(format!(".{op_ident}("));
                for param_expr in itertools::Itertools::intersperse_with(
                    param_exprs.into_iter().map(Some),
                    || None,
                ) {
                    match param_expr {
                        Some(param_expr) => {
                            self.emit_expr(writer, param_expr);
                        }
                        None => {
                            writer.write(", ");
                        }
                    }
                }
                writer.write(")");
            }
            ExprRepr::Neg { expr } => {
                writer.write("-");
                self.emit_expr(writer, expr);
            }
            ExprRepr::Add { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" + ");
                self.emit_expr(writer, rhs);
            }
            ExprRepr::Sub { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" - ");
                self.emit_expr(writer, rhs);
            }
            ExprRepr::Mul { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" * ");
                self.emit_expr(writer, rhs);
            }
            ExprRepr::Div { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" / ");
                self.emit_expr(writer, rhs);
            }
        };
    }

    fn emit_stmt_repr(&self, writer: &mut Writer<W>, stmt_repr: StmtRepr) {
        match stmt_repr {
            StmtRepr::AddAssign { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" += ");
                self.emit_expr(writer, rhs);
            }
            StmtRepr::SubAssign { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" -= ");
                self.emit_expr(writer, rhs);
            }
            StmtRepr::MulAssign { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" *= ");
                self.emit_expr(writer, rhs);
            }
            StmtRepr::DivAssign { lhs, rhs } => {
                self.emit_expr(writer, lhs);
                writer.write(" /= ");
                self.emit_expr(writer, rhs);
            }
        }
    }
}
