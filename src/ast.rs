use itertools::Itertools;
use symbol::Symbol;

#[derive(Clone, Copy)]
pub enum Ownership<T> {
    Owned(T),
    BorrowedMut(T),
}

impl<T> Ownership<T> {
    pub fn as_ref(&self) -> Ownership<&T> {
        match self {
            Self::Owned(t) => Ownership::Owned(t),
            Self::BorrowedMut(t) => Ownership::BorrowedMut(t),
        }
    }

    pub fn map<F, U>(self, f: F) -> Ownership<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Owned(t) => Ownership::Owned(f(t)),
            Self::BorrowedMut(t) => Ownership::BorrowedMut(f(t)),
        }
    }
}

pub enum Pretype<Type> {
    SelfBining,
    GenericBinding(usize),
    AssociateBinding(usize),
    Fixed(Type),
}

impl<Type> Pretype<Type> {
    fn eval_type<'t, const GENERICS: usize, const ASSOCIATES: usize>(
        &'t self,
        self_type: &'t Type,
        generic_types: [&'t Type; GENERICS],
        associate_types: [&'t Type; ASSOCIATES],
    ) -> &'t Type {
        match self {
            Self::SelfBining => self_type,
            Self::GenericBinding(index) => generic_types[*index],
            Self::AssociateBinding(index) => associate_types[*index],
            Self::Fixed(r#type) => r#type,
        }
    }
}

#[derive(Default)]
pub struct Items {
    pub structures: Vec<Structure>,
    pub implementations: Vec<Implementation>,
}

pub struct Structure {
    pub ident: Symbol,
    pub fields: Vec<(Symbol, Symbol)>,
}

pub struct Implementation {
    pub ident: Symbol,
    pub op_ident: Symbol,
    pub self_type: Symbol,
    pub generic_items: Vec<(Symbol, Symbol)>,
    pub associate_items: Vec<(Symbol, Symbol)>,
    pub self_param_item: Option<(Symbol, Ownership<Symbol>)>,
    pub param_items: Vec<(Symbol, Ownership<Symbol>)>,
    pub return_type: Option<Symbol>,
    pub body: ImplementationBody,
}

pub struct ImplementationBody {
    pub stmts: Vec<Stmt>,
    pub expr: Option<Expr>,
}

impl From<Vec<Stmt>> for ImplementationBody {
    fn from(stmts: Vec<Stmt>) -> Self {
        Self { stmts, expr: None }
    }
}

impl From<Expr> for ImplementationBody {
    fn from(expr: Expr) -> Self {
        Self {
            stmts: Vec::new(),
            expr: Some(expr),
        }
    }
}

pub trait Resolve: Sized {
    type Component;
    type Type;

    fn resolve_component(&self, component: &Self::Component) -> Symbol;
    fn resolve_type(&self, r#type: &Self::Type) -> Symbol;

    fn register_structure(
        &self,
        items: &mut Items,
        ident: impl AsRef<str>,
        fields: impl Iterator<Item = Self::Component>,
        field_type: impl Fn(&Self::Component) -> Self::Type,
    ) -> StructureSignature<'_, Self, Self::Component> {
        let signature = StructureSignature {
            resolver: self,
            ident: Symbol::from(ident),
            fields: fields.collect_vec(),
        };
        items.structures.push(Structure {
            ident: signature.ident,
            fields: signature
                .fields
                .iter()
                .map(|component| {
                    (
                        self.resolve_component(component),
                        self.resolve_type(&field_type(component)),
                    )
                })
                .collect(),
        });
        signature
    }

    fn define_operation<
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const SELF: usize,
        const PARAMS: usize,
        const RETURN: usize,
    >(
        &self,
        ident: &'static str,
        op_ident: &'static str,
        generics: [&'static str; GENERICS],
        associates: [&'static str; ASSOCIATES],
        self_param_item: [(&'static str, Ownership<Pretype<Self::Type>>); SELF],
        param_items: [(&'static str, Ownership<Pretype<Self::Type>>); PARAMS],
        return_pretype: [Pretype<Self::Type>; RETURN],
    ) -> OperationSignature<'_, Self, Self::Type, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN> {
        OperationSignature {
            resolver: self,
            ident: Symbol::from(ident),
            op_ident: Symbol::from(op_ident),
            generics: generics.map(Symbol::from),
            associates: associates.map(Symbol::from),
            self_param_item: self_param_item.map(|(self_param_ident, self_param_pretype)| {
                (Symbol::from(self_param_ident), self_param_pretype)
            }),
            param_items: param_items
                .map(|(param_ident, param_pretype)| (Symbol::from(param_ident), param_pretype)),
            return_pretype,
        }
    }
}

pub struct StructureSignature<'r, Resolver, Component> {
    resolver: &'r Resolver,
    ident: Symbol,
    fields: Vec<Component>,
}

impl<Resolver, Component> StructureSignature<'_, Resolver, Component>
where
    Resolver: Resolve<Component = Component>,
{
    pub fn construct(&self, field: impl Fn(&Component) -> Expr) -> Expr {
        ExprRepr::Struct {
            ident: self.ident,
            fields: self
                .fields
                .iter()
                .map(|component| (self.resolver.resolve_component(component), field(component)))
                .collect(),
        }
        .into()
    }

    pub fn access(&self, expr: Expr, component: &Component) -> Expr {
        ExprRepr::Field {
            expr,
            ident: self.resolver.resolve_component(component),
        }
        .into()
    }
}

pub struct OperationSignature<
    'r,
    Resolver,
    Type,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> {
    resolver: &'r Resolver,
    ident: Symbol,
    op_ident: Symbol,
    generics: [Symbol; GENERICS],
    associates: [Symbol; ASSOCIATES],
    self_param_item: [(Symbol, Ownership<Pretype<Type>>); SELF],
    param_items: [(Symbol, Ownership<Pretype<Type>>); PARAMS],
    return_pretype: [Pretype<Type>; RETURN],
}

impl<
        Resolver,
        Type,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const SELF: usize,
        const PARAMS: usize,
        const RETURN: usize,
    > OperationSignature<'_, Resolver, Type, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>
where
    Resolver: Resolve<Type = Type>,
{
    pub fn register_implementation<Body>(
        &self,
        items: &mut Items,
        self_type: Type,
        generic_types: [Type; GENERICS],
        associate_types: [Type; ASSOCIATES],
        implementor: impl FnOnce([Symbol; SELF], [Symbol; PARAMS]) -> Body,
    ) where
        Body: Into<ImplementationBody>,
    {
        items.implementations.push(Implementation {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self.resolver.resolve_type(&self_type),
            generic_items: self
                .generics
                .into_iter()
                .zip_eq(
                    generic_types
                        .each_ref()
                        .map(|r#type| self.resolver.resolve_type(r#type)),
                )
                .collect(),
            associate_items: self
                .associates
                .into_iter()
                .zip_eq(
                    associate_types
                        .each_ref()
                        .map(|r#type| self.resolver.resolve_type(r#type)),
                )
                .collect(),
            self_param_item: self.self_param_item.first().map(
                |(self_param_ident, self_param_pretype)| {
                    (
                        *self_param_ident,
                        self_param_pretype.as_ref().map(|self_param_pretype| {
                            self.resolver.resolve_type(self_param_pretype.eval_type(
                                &self_type,
                                generic_types.each_ref(),
                                associate_types.each_ref(),
                            ))
                        }),
                    )
                },
            ),
            param_items: Vec::from(self.param_items.each_ref().map(
                |(param_ident, param_pretype)| {
                    (
                        *param_ident,
                        param_pretype.as_ref().map(|param_pretype| {
                            self.resolver.resolve_type(param_pretype.eval_type(
                                &self_type,
                                generic_types.each_ref(),
                                associate_types.each_ref(),
                            ))
                        }),
                    )
                },
            )),
            return_type: self.return_pretype.first().map(|return_pretype| {
                self.resolver.resolve_type(return_pretype.eval_type(
                    &self_type,
                    generic_types.each_ref(),
                    associate_types.each_ref(),
                ))
            }),
            body: implementor(
                self.self_param_item
                    .each_ref()
                    .map(|(self_param_ident, _)| *self_param_ident),
                self.param_items
                    .each_ref()
                    .map(|(param_ident, _)| *param_ident),
            )
            .into(),
        });
    }
}

impl<Resolver, Type, const ASSOCIATES: usize>
    OperationSignature<'_, Resolver, Type, 0, ASSOCIATES, 1, 0, 1>
where
    Resolver: Resolve<Type = Type>,
{
    pub fn call_builtin(&self, self_expr: Expr) -> Expr {
        ExprRepr::CallBuiltin {
            ident: self.ident,
            op_ident: self.op_ident,
            self_expr,
        }
        .into()
    }
}

impl<Resolver, Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<'_, Resolver, Type, GENERICS, ASSOCIATES, 0, PARAMS, 1>
where
    Resolver: Resolve<Type = Type>,
{
    pub fn call(
        &self,
        self_type: Type,
        generic_types: [Type; GENERICS],
        param_exprs: [Expr; PARAMS],
    ) -> Expr {
        ExprRepr::CallFunction {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self.resolver.resolve_type(&self_type),
            generic_types: Vec::from(
                generic_types.map(|r#type| self.resolver.resolve_type(&r#type)),
            ),
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<Resolver, Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<'_, Resolver, Type, GENERICS, ASSOCIATES, 1, PARAMS, 1>
where
    Resolver: Resolve<Type = Type>,
{
    pub fn call(
        &self,
        self_type: Type,
        generic_types: [Type; GENERICS],
        self_expr: Expr,
        param_exprs: [Expr; PARAMS],
    ) -> Expr {
        ExprRepr::CallMethod {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self.resolver.resolve_type(&self_type),
            generic_types: Vec::from(
                generic_types.map(|r#type| self.resolver.resolve_type(&r#type)),
            ),
            self_expr,
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<Resolver, Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<'_, Resolver, Type, GENERICS, ASSOCIATES, 1, PARAMS, 0>
where
    Resolver: Resolve<Type = Type>,
{
    pub fn call(
        &self,
        self_type: Type,
        generic_types: [Type; GENERICS],
        self_expr: Expr,
        param_exprs: [Expr; PARAMS],
    ) -> Stmt {
        StmtRepr::Expr {
            expr: ExprRepr::CallMethod {
                ident: self.ident,
                op_ident: self.op_ident,
                self_type: self.resolver.resolve_type(&self_type),
                generic_types: Vec::from(
                    generic_types.map(|r#type| self.resolver.resolve_type(&r#type)),
                ),
                self_expr,
                param_exprs: Vec::from(param_exprs),
            }
            .into(),
        }
        .into()
    }
}

pub enum ExprRepr {
    Variable {
        ident: Symbol,
    },
    Literal {
        value: u32,
    },
    Struct {
        ident: Symbol,
        fields: Vec<(Symbol, Expr)>,
    },
    Field {
        expr: Expr,
        ident: Symbol,
    },
    CallBuiltin {
        ident: Symbol,
        op_ident: Symbol,
        self_expr: Expr,
    },
    CallFunction {
        ident: Symbol,
        op_ident: Symbol,
        self_type: Symbol,
        generic_types: Vec<Symbol>,
        param_exprs: Vec<Expr>,
    },
    CallMethod {
        ident: Symbol,
        op_ident: Symbol,
        self_type: Symbol,
        generic_types: Vec<Symbol>,
        self_expr: Expr,
        param_exprs: Vec<Expr>,
    },
    Neg {
        expr: Expr,
    },
    Add {
        lhs: Expr,
        rhs: Expr,
    },
    Sub {
        lhs: Expr,
        rhs: Expr,
    },
    Mul {
        lhs: Expr,
        rhs: Expr,
    },
    Div {
        lhs: Expr,
        rhs: Expr,
    },
}

pub struct Expr {
    pub repr: Box<ExprRepr>,
}

impl From<ExprRepr> for Expr {
    fn from(repr: ExprRepr) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}

impl From<Symbol> for Expr {
    fn from(ident: Symbol) -> Self {
        ExprRepr::Variable { ident }.into()
    }
}

impl From<u32> for Expr {
    fn from(value: u32) -> Self {
        ExprRepr::Literal { value }.into()
    }
}

impl From<i32> for Expr {
    fn from(value: i32) -> Self {
        if value < 0 {
            ExprRepr::Neg {
                expr: ExprRepr::Literal {
                    value: value.unsigned_abs(),
                }
                .into(),
            }
            .into()
        } else {
            ExprRepr::Literal {
                value: value.unsigned_abs(),
            }
            .into()
        }
    }
}

impl std::ops::Neg for Expr {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ExprRepr::Neg { expr: self }.into()
    }
}

impl std::ops::Add for Expr {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ExprRepr::Add { lhs: self, rhs }.into()
    }
}

impl std::ops::Sub for Expr {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ExprRepr::Sub { lhs: self, rhs }.into()
    }
}

impl std::ops::Mul for Expr {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ExprRepr::Mul { lhs: self, rhs }.into()
    }
}

impl std::ops::Div for Expr {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ExprRepr::Div { lhs: self, rhs }.into()
    }
}

impl Expr {
    pub fn add_assign(self, rhs: Expr) -> Stmt {
        StmtRepr::AddAssign { lhs: self, rhs }.into()
    }

    pub fn sub_assign(self, rhs: Expr) -> Stmt {
        StmtRepr::SubAssign { lhs: self, rhs }.into()
    }

    pub fn mul_assign(self, rhs: Expr) -> Stmt {
        StmtRepr::MulAssign { lhs: self, rhs }.into()
    }

    pub fn div_assign(self, rhs: Expr) -> Stmt {
        StmtRepr::DivAssign { lhs: self, rhs }.into()
    }
}

pub enum StmtRepr {
    Expr { expr: Expr },
    AddAssign { lhs: Expr, rhs: Expr },
    SubAssign { lhs: Expr, rhs: Expr },
    MulAssign { lhs: Expr, rhs: Expr },
    DivAssign { lhs: Expr, rhs: Expr },
}

pub struct Stmt {
    pub repr: Box<StmtRepr>,
}

impl From<StmtRepr> for Stmt {
    fn from(repr: StmtRepr) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}
