use symbol::Symbol;

pub trait ToSymbol {
    fn to_symbol(&self) -> Symbol;
}

#[derive(Clone, Copy)]
pub enum Ownership<T> {
    Owned(T),
    BorrowedMut(T),
}

impl<T> Ownership<T> {
    fn as_ref(&self) -> Ownership<&T> {
        match self {
            Self::Owned(t) => Ownership::Owned(t),
            Self::BorrowedMut(t) => Ownership::BorrowedMut(t),
        }
    }

    fn map<F, U>(self, f: F) -> Ownership<U>
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

pub enum PretypeSymbol {
    SelfSymbol,
    Generic(Symbol),
    Associate(Symbol),
    Fixed(Symbol),
}

impl<Type> Pretype<Type>
where
    Type: ToSymbol,
{
    fn eval_symbol<const GENERICS: usize, const ASSOCIATES: usize>(
        &self,
        generics: &[Symbol; GENERICS],
        associates: &[Symbol; ASSOCIATES],
    ) -> PretypeSymbol {
        match self {
            Self::SelfBining => PretypeSymbol::SelfSymbol,
            Self::GenericBinding(index) => PretypeSymbol::Generic(generics[*index]),
            Self::AssociateBinding(index) => PretypeSymbol::Associate(associates[*index]),
            Self::Fixed(r#type) => PretypeSymbol::Fixed(r#type.to_symbol()),
        }
    }

    fn eval_type<'t, const GENERICS: usize, const ASSOCIATES: usize>(
        &'t self,
        self_type: &'t Type,
        generic_types: &'t [Type; GENERICS],
        associate_types: &'t [Type; ASSOCIATES],
    ) -> &'t Type {
        match self {
            Self::SelfBining => self_type,
            Self::GenericBinding(index) => generic_types.each_ref()[*index],
            Self::AssociateBinding(index) => associate_types.each_ref()[*index],
            Self::Fixed(r#type) => r#type,
        }
    }
}

#[derive(Default)]
pub struct Items {
    pub structs: Vec<Struct>,
    pub operations: Vec<Operation>,
    pub implementations: Vec<Implementation>,
}

impl Items {
    pub fn add_struct<Type>(
        &mut self,
        r#type: Type,
        fields: impl Iterator<Item = (Symbol, Type)>,
    ) -> Type
    where
        Type: ToSymbol,
    {
        self.structs.push(Struct {
            ident: r#type.to_symbol(),
            fields: fields
                .map(|(field_ident, field_type)| (field_ident, field_type.to_symbol()))
                .collect(),
        });
        r#type
    }

    pub fn add_operation<
        Type,
        Body,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const SELF: usize,
        const PARAMS: usize,
        const RETURN: usize,
    >(
        &mut self,
        ident: &'static str,
        op_ident: &'static str,
        signature: &OperationSignature<Type, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>,
        implementation_items: impl Iterator<Item = (Type, [Type; GENERICS], [Type; ASSOCIATES])>,
        implementor: impl Fn(
            Type,
            [Type; GENERICS],
            [Type; ASSOCIATES],
            [Symbol; SELF],
            [Symbol; PARAMS],
        ) -> Body,
    ) -> OperationHandle<GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>
    where
        Type: ToSymbol,
        Body: Into<ImplementationBody>,
    {
        let generics = signature.generics.each_ref().map(Symbol::from);
        let associates = signature.associates.each_ref().map(Symbol::from);
        self.operations.push(Operation {
            ident: Symbol::from(ident),
            op_ident: Symbol::from(op_ident),
            generics: Vec::from(generics.clone()),
            associates: Vec::from(associates.clone()),
            self_param_item: signature.self_param_item.iter().next().map(
                |(self_param_ident, self_param_pretype)| {
                    (
                        Symbol::from(self_param_ident),
                        self_param_pretype.as_ref().map(|self_param_pretype| {
                            self_param_pretype.eval_symbol(&generics, &associates)
                        }),
                    )
                },
            ),
            param_items: Vec::from(signature.param_items.each_ref().map(
                |(param_ident, param_pretype)| {
                    (
                        Symbol::from(param_ident),
                        param_pretype
                            .as_ref()
                            .map(|param_pretype| param_pretype.eval_symbol(&generics, &associates)),
                    )
                },
            )),
            return_pretype: signature
                .return_pretype
                .iter()
                .next()
                .map(|return_pretype| {
                    return_pretype
                        .as_ref()
                        .map(|return_pretype| return_pretype.eval_symbol(&generics, &associates))
                }),
        });
        self.implementations.extend(implementation_items.map(
            |(self_type, generic_types, associate_types)| {
                Implementation {
                    ident: Symbol::from(ident),
                    op_ident: Symbol::from(op_ident),
                    self_type: self_type.to_symbol(),
                    generic_items: signature
                        .generics
                        .iter()
                        .map(Symbol::from)
                        .zip(generic_types.iter().map(ToSymbol::to_symbol))
                        .collect(),
                    associate_items: signature
                        .associates
                        .iter()
                        .map(Symbol::from)
                        .zip(associate_types.iter().map(ToSymbol::to_symbol))
                        .collect(),
                    self_param_item: signature.self_param_item.iter().next().map(
                        |(self_param_ident, self_param_pretype)| {
                            (
                                Symbol::from(self_param_ident),
                                self_param_pretype.as_ref().map(|self_param_pretype| {
                                    self_param_pretype
                                        .eval_type(&self_type, &generic_types, &associate_types)
                                        .to_symbol()
                                }),
                            )
                        },
                    ),
                    param_items: Vec::from(signature.param_items.each_ref().map(
                        |(param_ident, param_pretype)| {
                            (
                                Symbol::from(param_ident),
                                param_pretype.as_ref().map(|param_pretype| {
                                    param_pretype
                                        .eval_type(&self_type, &generic_types, &associate_types)
                                        .to_symbol()
                                }),
                            )
                        },
                    )),
                    return_type: signature
                        .return_pretype
                        .iter()
                        .next()
                        .map(|return_pretype| {
                            return_pretype.as_ref().map(|return_pretype| {
                                return_pretype
                                    .eval_type(&self_type, &generic_types, &associate_types)
                                    .to_symbol()
                            })
                        }),
                    body: implementor(
                        self_type,
                        generic_types,
                        associate_types,
                        signature
                            .self_param_item
                            .each_ref()
                            .map(|(self_param_ident, _)| Symbol::from(self_param_ident)),
                        signature
                            .param_items
                            .each_ref()
                            .map(|(param_ident, _)| Symbol::from(param_ident)),
                    )
                    .into(),
                }
            },
        ));
        OperationHandle {
            ident: Symbol::from(ident),
            op_ident: Symbol::from(op_ident),
        }
    }
}

pub struct Struct {
    pub ident: Symbol,
    pub fields: Vec<(Symbol, Symbol)>,
}

pub struct Operation {
    pub ident: Symbol,
    pub op_ident: Symbol,
    pub generics: Vec<Symbol>,
    pub associates: Vec<Symbol>,
    pub self_param_item: Option<(Symbol, Ownership<PretypeSymbol>)>,
    pub param_items: Vec<(Symbol, Ownership<PretypeSymbol>)>,
    pub return_pretype: Option<Ownership<PretypeSymbol>>,
}

pub struct Implementation {
    pub ident: Symbol,
    pub op_ident: Symbol,
    pub self_type: Symbol,
    pub generic_items: Vec<(Symbol, Symbol)>,
    pub associate_items: Vec<(Symbol, Symbol)>,
    pub self_param_item: Option<(Symbol, Ownership<Symbol>)>,
    pub param_items: Vec<(Symbol, Ownership<Symbol>)>,
    pub return_type: Option<Ownership<Symbol>>,
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

pub struct OperationSignature<
    Type,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> {
    pub generics: [&'static str; GENERICS],
    pub associates: [&'static str; ASSOCIATES],
    pub self_param_item: [(&'static str, Ownership<Pretype<Type>>); SELF],
    pub param_items: [(&'static str, Ownership<Pretype<Type>>); PARAMS],
    pub return_pretype: [Ownership<Pretype<Type>>; RETURN],
}

pub struct OperationHandle<
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> {
    ident: Symbol,
    op_ident: Symbol,
}

impl<const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationHandle<GENERICS, ASSOCIATES, 0, PARAMS, 1>
{
    pub fn call<Type>(
        &self,
        self_type: Type,
        generic_types: [Type; GENERICS],
        param_exprs: [Expr; PARAMS],
    ) -> Expr
    where
        Type: ToSymbol,
    {
        ExprRepr::CallFunction {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self_type.to_symbol(),
            generic_types: Vec::from(generic_types.each_ref().map(ToSymbol::to_symbol)),
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationHandle<GENERICS, ASSOCIATES, 1, PARAMS, 1>
{
    pub fn call<Type>(
        &self,
        self_type: Type,
        generic_types: [Type; GENERICS],
        self_expr: Expr,
        param_exprs: [Expr; PARAMS],
    ) -> Expr
    where
        Type: ToSymbol,
    {
        ExprRepr::CallMethod {
            ident: self.ident,
            op_ident: self.op_ident,
            self_type: self_type.to_symbol(),
            generic_types: Vec::from(generic_types.each_ref().map(ToSymbol::to_symbol)),
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
