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

// pub struct ComponentSignature<Component> {
//     pub ident: Symbol,
//     pub component: Component,
// }

pub trait ContextTrait: Sized {
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
            context: self,
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

    fn register_operation<
        Iter,
        Body,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const SELF: usize,
        const PARAMS: usize,
        const RETURN: usize,
    >(
        &self,
        items: &mut Items,
        ident: &'static str,
        op_ident: &'static str,
        generics: [&'static str; GENERICS],
        associates: [&'static str; ASSOCIATES],
        self_param_item: [(&'static str, Ownership<Pretype<Self::Type>>); SELF],
        param_items: [(&'static str, Ownership<Pretype<Self::Type>>); PARAMS],
        return_pretype: [Pretype<Self::Type>; RETURN],
        implementor: impl FnOnce([Symbol; SELF], [Symbol; PARAMS]) -> Iter,
    ) -> OperationSignature<'_, Self, Self::Type, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>
    where
        Iter: Iterator<
            Item = (
                Self::Type,
                [Self::Type; GENERICS],
                [Self::Type; ASSOCIATES],
                Body,
            ),
        >,
        Body: Into<ImplementationBody>,
    {
        let signature = OperationSignature {
            context: self,
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
        };
        items.implementations.extend(
            implementor(
                signature
                    .self_param_item
                    .each_ref()
                    .map(|(self_param_ident, _)| *self_param_ident),
                signature
                    .param_items
                    .each_ref()
                    .map(|(param_ident, _)| *param_ident),
            )
            .map(
                |(self_type, generic_types, associate_types, body)| Implementation {
                    ident: signature.ident,
                    op_ident: signature.op_ident,
                    self_type: self.resolve_type(&self_type),
                    generic_items: signature
                        .generics
                        .into_iter()
                        .zip_eq(
                            generic_types
                                .each_ref()
                                .map(|r#type| self.resolve_type(r#type)),
                        )
                        .collect(),
                    associate_items: signature
                        .associates
                        .into_iter()
                        .zip_eq(
                            associate_types
                                .each_ref()
                                .map(|r#type| self.resolve_type(r#type)),
                        )
                        .collect(),
                    self_param_item: signature.self_param_item.first().map(
                        |(self_param_ident, self_param_pretype)| {
                            (
                                *self_param_ident,
                                self_param_pretype.as_ref().map(|self_param_pretype| {
                                    self.resolve_type(self_param_pretype.eval_type(
                                        &self_type,
                                        generic_types.each_ref(),
                                        associate_types.each_ref(),
                                    ))
                                }),
                            )
                        },
                    ),
                    param_items: Vec::from(signature.param_items.each_ref().map(
                        |(param_ident, param_pretype)| {
                            (
                                *param_ident,
                                param_pretype.as_ref().map(|param_pretype| {
                                    self.resolve_type(param_pretype.eval_type(
                                        &self_type,
                                        generic_types.each_ref(),
                                        associate_types.each_ref(),
                                    ))
                                }),
                            )
                        },
                    )),
                    return_type: signature.return_pretype.first().map(|return_pretype| {
                        self.resolve_type(return_pretype.eval_type(
                            &self_type,
                            generic_types.each_ref(),
                            associate_types.each_ref(),
                        ))
                    }),
                    body: body.into(),
                },
            ),
        );
        signature
    }
}

// pub struct Context<Component, Type> {
//     resolve_component: Rc<dyn Fn(&Component) -> Symbol>,
//     resolve_type: Rc<dyn Fn(&Type) -> Symbol>,
//     items: Items,
// }

// impl<Component, Type> Context<Component, Type> {
//     pub fn new(
//         resolve_component: impl Fn(&Component) -> Symbol,
//         resolve_type: impl Fn(&Type) -> Symbol,
//     ) -> Self {
//         Self {
//             resolve_component: Rc::new(resolve_component),
//             resolve_type: Rc::new(resolve_type),
//             items: Items {
//                 structures: Vec::new(),
//                 implementations: Vec::new(),
//             },
//         }
//     }

//     pub fn into_items(self) -> Items {
//         self.items
//     }

// }

pub struct StructureSignature<'ctx, Context, Component> {
    context: &'ctx Context,
    ident: Symbol,
    fields: Vec<Component>,
    // pub constructor_expr: fn(Symbol, Box<dyn Fn(Expr, Component) -> Option<Expr>>) -> Expr,
    // pub accessor_expr: fn(Expr, Component) -> Option<Expr>,
}

impl<Context, Component> StructureSignature<'_, Context, Component>
where
    Context: ContextTrait<Component = Component>,
{
    pub fn construct(&self, field: impl Fn(&Component) -> Expr) -> Expr {
        ExprRepr::Struct {
            ident: self.ident,
            fields: self
                .fields
                .iter()
                .map(|component| (self.context.resolve_component(component), field(component)))
                .collect(),
        }
        .into()
    }

    pub fn access(&self, expr: Expr, component: &Component) -> Expr {
        ExprRepr::Field {
            expr,
            ident: self.context.resolve_component(component),
        }
        .into()
    }

    // pub fn register(self, items: &mut Items, field: Fn(&Component) -> Type) -> Self {
    //     items.structures.push(Structure {
    //         ident: self.ident,
    //         fields: self
    //             .fields
    //             .iter()
    //             .map(|(field_ident, field_component)| {
    //                 (*field_ident, (self.resolve_component)(field_component))
    //             })
    //             .collect(),
    //     });
    //     self
    // }
}

pub struct OperationSignature<
    'ctx,
    Context,
    Type,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> {
    context: &'ctx Context,
    ident: Symbol,
    op_ident: Symbol,
    generics: [Symbol; GENERICS],
    associates: [Symbol; ASSOCIATES],
    self_param_item: [(Symbol, Ownership<Pretype<Type>>); SELF],
    param_items: [(Symbol, Ownership<Pretype<Type>>); PARAMS],
    return_pretype: [Pretype<Type>; RETURN],
}

// impl<
//         Type,
//         const GENERICS: usize,
//         const ASSOCIATES: usize,
//         const SELF: usize,
//         const PARAMS: usize,
//         const RETURN: usize,
//     > OperationSignature<Type, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>
// {
//     pub fn register<Iter, Body>(
//         self,
//         items: &mut Items,
//         implementation_items: impl Fn([Symbol; SELF], [Symbol; PARAMS]) -> Iter,
//     ) -> Self
//     where
//         Type: Clone,
//         Iter: Iterator<Item = (Type, [Type; GENERICS], [Type; ASSOCIATES], Body)>,
//         Body: Into<ImplementationBody>,
//     {

//         self
//     }
// }

impl<Context, Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<'_, Context, Type, GENERICS, ASSOCIATES, 0, PARAMS, 1>
where
    Context: ContextTrait<Type = Type>,
{
    pub fn call(
        &self,
        self_type: &Type,
        generic_types: [&Type; GENERICS],
        param_exprs: [Expr; PARAMS],
    ) -> Expr {
        ExprRepr::CallFunction {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self.context.resolve_type(self_type),
            generic_types: Vec::from(generic_types.map(|r#type| self.context.resolve_type(r#type))),
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<Context, Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<'_, Context, Type, GENERICS, ASSOCIATES, 1, PARAMS, 1>
where
    Context: ContextTrait<Type = Type>,
{
    pub fn call(
        &self,
        self_type: &Type,
        generic_types: [&Type; GENERICS],
        self_expr: Expr,
        param_exprs: [Expr; PARAMS],
    ) -> Expr {
        ExprRepr::CallMethod {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self.context.resolve_type(self_type),
            generic_types: Vec::from(generic_types.map(|r#type| self.context.resolve_type(r#type))),
            self_expr,
            param_exprs: Vec::from(param_exprs),
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

pub enum StmtRepr {
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
