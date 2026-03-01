pub trait Ast: Sized {
    type Type: Clone;
    type Template: Clone;
    type Field: Clone;
    type Operation: Clone + OperationName;
    type Generic: Clone + std::fmt::Display;
    type Associate: Clone + std::fmt::Display;
    type Param: Clone + std::fmt::Display;
    type Literal: Clone + std::fmt::Display;

    fn structures(&self) -> impl Iterator<Item = Structure<Self>>;
    fn implementations(&self) -> impl Iterator<Item = Implementation<Self>>;

    fn record(&self) -> Record<Self> {
        Record {
            structures: self.structures().collect(),
            implementations: self.implementations().collect(),
        }
    }
}

pub trait OperationName {
    fn trait_name(&self) -> &str;
    fn fn_name(&self) -> &str;
    fn is_builtin(&self) -> bool;
}

pub trait Stringifier<A: Ast> {
    fn stringify_type(&self, r#type: &A::Type) -> &str;
    fn stringify_template(&self, template: &A::Template) -> &str;
    fn stringify_field(&self, field: &A::Field) -> &str;
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TypeBinding<Type> {
    SelfBining,
    GenericBinding(usize),
    AssociateBinding(usize),
    Fixed(Type),
}

impl<Type: Clone> TypeBinding<Type> {
    fn eval_type<const GENERICS: usize, const ASSOCIATES: usize>(
        &self,
        self_type: &Type,
        generic_types: &[Type; GENERICS],
        associate_types: &[Type; ASSOCIATES],
    ) -> Type {
        match self {
            Self::SelfBining => self_type.clone(),
            Self::GenericBinding(index) => generic_types[*index].clone(),
            Self::AssociateBinding(index) => associate_types[*index].clone(),
            Self::Fixed(r#type) => r#type.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Ownership<T> {
    Owned(T),
    Borrowed(T),
    BorrowedMut(T),
}

impl<T> Ownership<T> {
    fn as_ref(&self) -> Ownership<&T> {
        match self {
            Self::Owned(t) => Ownership::Owned(t),
            Self::Borrowed(t) => Ownership::Borrowed(t),
            Self::BorrowedMut(t) => Ownership::BorrowedMut(t),
        }
    }

    fn map<U>(self, f: impl FnOnce(T) -> U) -> Ownership<U> {
        match self {
            Self::Owned(t) => Ownership::Owned(f(t)),
            Self::Borrowed(t) => Ownership::Borrowed(f(t)),
            Self::BorrowedMut(t) => Ownership::BorrowedMut(f(t)),
        }
    }
}

pub struct Record<A: Ast> {
    pub structures: Vec<Structure<A>>,
    pub implementations: Vec<Implementation<A>>,
}

pub struct Item<Key, Value> {
    pub key: Key,
    pub value: Value,
}

pub struct TemplateSignature<A: Ast> {
    pub template: A::Template,
}

impl<A: Ast> TemplateSignature<A> {
    pub fn structure(
        &self,
        fields: impl IntoIterator<Item = Item<A::Field, A::Type>>,
    ) -> Structure<A> {
        Structure {
            template: self.template.clone(),
            fields: fields.into_iter().collect(),
        }
    }

    pub fn construct(&self, fields: impl IntoIterator<Item = Item<A::Field, Expr<A>>>) -> Expr<A> {
        ExprRepr::Struct {
            template: self.template.clone(),
            fields: fields.into_iter().collect(),
        }
        .into()
    }

    pub fn access(&self, expr: Expr<A>, field: A::Field) -> Expr<A> {
        ExprRepr::Field { expr, field }.into()
    }
}

pub struct Structure<A: Ast> {
    pub template: A::Template,
    pub fields: Vec<Item<A::Field, A::Type>>,
}

pub struct OperationSignature<
    A: Ast,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> {
    pub operation: A::Operation,
    pub generics: [A::Generic; GENERICS],
    pub associates: [A::Associate; ASSOCIATES],
    pub self_param_item: [Item<A::Param, Ownership<TypeBinding<A::Type>>>; SELF],
    pub param_items: [Item<A::Param, Ownership<TypeBinding<A::Type>>>; PARAMS],
    pub return_type_binding: [TypeBinding<A::Type>; RETURN],
}

impl<
    A: Ast,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const SELF: usize,
    const PARAMS: usize,
    const RETURN: usize,
> OperationSignature<A, GENERICS, ASSOCIATES, SELF, PARAMS, RETURN>
{
    fn implementation<Body>(
        &self,
        self_type: A::Type,
        generic_types: [A::Type; GENERICS],
        associate_types: [A::Type; ASSOCIATES],
        body_fn: impl FnOnce([A::Param; SELF], [A::Param; PARAMS]) -> Body,
    ) -> Implementation<A>
    where
        Body: Into<ImplementationBody<A>>,
    {
        Implementation {
            operation: self.operation.clone(),
            self_param_item: self.self_param_item.first().map(
                |Item {
                     key: self_param,
                     value: self_param_type_binding,
                 }| {
                    Item {
                        key: self_param.clone(),
                        value: self_param_type_binding
                            .as_ref()
                            .map(|self_param_type_binding| {
                                self_param_type_binding.eval_type(
                                    &self_type,
                                    &generic_types,
                                    &associate_types,
                                )
                            }),
                    }
                },
            ),
            param_items: Vec::from(self.param_items.each_ref().map(
                |Item {
                     key: param,
                     value: param_type_binding,
                 }| {
                    Item {
                        key: param.clone(),
                        value: param_type_binding.as_ref().map(|param_type_binding| {
                            param_type_binding.eval_type(
                                &self_type,
                                &generic_types,
                                &associate_types,
                            )
                        }),
                    }
                },
            )),
            return_type: self.return_type_binding.first().map(|return_type_binding| {
                return_type_binding.eval_type(&self_type, &generic_types, &associate_types)
            }),
            self_type,
            generic_items: self
                .generics
                .iter()
                .cloned()
                .zip(generic_types)
                .map(|(key, value)| Item { key, value })
                .collect(),
            associate_items: self
                .associates
                .iter()
                .cloned()
                .zip(associate_types)
                .map(|(key, value)| Item { key, value })
                .collect(),
            body: body_fn(
                self.self_param_item.each_ref().map(
                    |Item {
                         key: self_param,
                         value: _,
                     }| self_param.clone(),
                ),
                self.param_items.each_ref().map(
                    |Item {
                         key: param,
                         value: _,
                     }| param.clone(),
                ),
            )
            .into(),
        }
    }

    pub fn implementations<SelfTypeIter, GenericTypesIter, Body>(
        &self,
        self_type: SelfTypeIter,
        generic_types: impl Fn(A::Type) -> GenericTypesIter,
        associate_types: impl Fn(A::Type, [A::Type; GENERICS]) -> Option<[A::Type; ASSOCIATES]>,
        body_fn: impl Fn(
            A::Type,
            [A::Type; GENERICS],
            [A::Type; ASSOCIATES],
            [A::Param; SELF],
            [A::Param; PARAMS],
        ) -> Body,
    ) -> Vec<Implementation<A>>
    where
        SelfTypeIter: IntoIterator<Item = A::Type>,
        GenericTypesIter: IntoIterator<Item = [A::Type; GENERICS]>,
        Body: Into<ImplementationBody<A>>,
    {
        self_type
            .into_iter()
            .flat_map(|ref self_type| {
                generic_types(self_type.clone())
                    .into_iter()
                    .filter_map(|ref generic_types| {
                        associate_types(self_type.clone(), generic_types.clone()).map(
                            |ref associate_types| {
                                self.implementation(
                                    self_type.clone(),
                                    generic_types.clone(),
                                    associate_types.clone(),
                                    |self_param, params| {
                                        body_fn(
                                            self_type.clone(),
                                            generic_types.clone(),
                                            associate_types.clone(),
                                            self_param,
                                            params,
                                        )
                                    },
                                )
                            },
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl<A: Ast, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<A, GENERICS, ASSOCIATES, 0, PARAMS, 1>
{
    #[allow(dead_code)]
    pub fn call_expr(
        &self,
        self_type: A::Type,
        generic_types: [A::Type; GENERICS],
        param_exprs: [Expr<A>; PARAMS],
    ) -> Expr<A> {
        ExprRepr::Call {
            operation: self.operation.clone(),
            self_type,
            generic_types: Vec::from(generic_types),
            self_expr: None,
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<A: Ast, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<A, GENERICS, ASSOCIATES, 1, PARAMS, 1>
{
    #[allow(dead_code)]
    pub fn call_expr(
        &self,
        self_type: A::Type,
        generic_types: [A::Type; GENERICS],
        self_expr: Expr<A>,
        param_exprs: [Expr<A>; PARAMS],
    ) -> Expr<A> {
        ExprRepr::Call {
            operation: self.operation.clone(),
            self_type,
            generic_types: Vec::from(generic_types),
            self_expr: Some(self_expr),
            param_exprs: Vec::from(param_exprs),
        }
        .into()
    }
}

impl<A: Ast, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<A, GENERICS, ASSOCIATES, 0, PARAMS, 0>
{
    #[allow(dead_code)]
    pub fn call_stmt(
        &self,
        self_type: A::Type,
        generic_types: [A::Type; GENERICS],
        param_exprs: [Expr<A>; PARAMS],
    ) -> Stmt<A> {
        StmtRepr::Expr {
            expr: ExprRepr::Call {
                operation: self.operation.clone(),
                self_type,
                generic_types: Vec::from(generic_types),
                self_expr: None,
                param_exprs: Vec::from(param_exprs),
            }
            .into(),
        }
        .into()
    }
}

impl<A: Ast, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>
    OperationSignature<A, GENERICS, ASSOCIATES, 1, PARAMS, 0>
{
    #[allow(dead_code)]
    pub fn call_stmt(
        &self,
        self_type: A::Type,
        generic_types: [A::Type; GENERICS],
        self_expr: Expr<A>,
        param_exprs: [Expr<A>; PARAMS],
    ) -> Stmt<A> {
        StmtRepr::Expr {
            expr: ExprRepr::Call {
                operation: self.operation.clone(),
                self_type,
                generic_types: Vec::from(generic_types),
                self_expr: Some(self_expr),
                param_exprs: Vec::from(param_exprs),
            }
            .into(),
        }
        .into()
    }
}

pub struct Implementation<A: Ast> {
    pub operation: A::Operation,
    pub self_type: A::Type,
    pub generic_items: Vec<Item<A::Generic, A::Type>>,
    pub associate_items: Vec<Item<A::Associate, A::Type>>,
    pub self_param_item: Option<Item<A::Param, Ownership<A::Type>>>,
    pub param_items: Vec<Item<A::Param, Ownership<A::Type>>>,
    pub return_type: Option<A::Type>,
    pub body: ImplementationBody<A>,
}

pub struct ImplementationBody<A: Ast> {
    pub stmts: Vec<Stmt<A>>,
    pub expr: Option<Expr<A>>,
}

impl<A: Ast> From<Vec<Stmt<A>>> for ImplementationBody<A> {
    fn from(stmts: Vec<Stmt<A>>) -> Self {
        Self { stmts, expr: None }
    }
}

impl<A: Ast> From<Expr<A>> for ImplementationBody<A> {
    fn from(expr: Expr<A>) -> Self {
        Self {
            stmts: Vec::new(),
            expr: Some(expr),
        }
    }
}

pub enum ExprRepr<A: Ast> {
    Variable {
        param: A::Param,
    },
    Literal {
        value: A::Literal,
    },
    Struct {
        template: A::Template,
        fields: Vec<Item<A::Field, Expr<A>>>,
    },
    Field {
        expr: Expr<A>,
        field: A::Field,
    },
    Borrow {
        expr: Expr<A>,
    },
    BorrowMut {
        expr: Expr<A>,
    },
    Deref {
        expr: Expr<A>,
    },
    Call {
        operation: A::Operation,
        self_type: A::Type,
        generic_types: Vec<A::Type>,
        self_expr: Option<Expr<A>>,
        param_exprs: Vec<Expr<A>>,
    },
    Neg {
        expr: Expr<A>,
    },
    Add {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    Sub {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    Mul {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    Div {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
}

pub struct Expr<A: Ast> {
    pub repr: Box<ExprRepr<A>>,
}

impl<A: Ast> From<ExprRepr<A>> for Expr<A> {
    fn from(repr: ExprRepr<A>) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}

impl<A: Ast> Expr<A> {
    pub fn param(param: A::Param) -> Self {
        ExprRepr::Variable { param }.into()
    }

    pub fn literal(value: A::Literal) -> Self {
        ExprRepr::Literal { value }.into()
    }

    pub fn borrow(self) -> Self {
        ExprRepr::Borrow { expr: self }.into()
    }

    pub fn borrow_mut(self) -> Self {
        ExprRepr::BorrowMut { expr: self }.into()
    }

    pub fn deref(self) -> Self {
        ExprRepr::Deref { expr: self }.into()
    }
}

impl<A: Ast> std::ops::Neg for Expr<A> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ExprRepr::Neg { expr: self }.into()
    }
}

impl<A: Ast> std::ops::Add for Expr<A> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ExprRepr::Add { lhs: self, rhs }.into()
    }
}

impl<A: Ast> std::ops::Sub for Expr<A> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        ExprRepr::Sub { lhs: self, rhs }.into()
    }
}

impl<A: Ast> std::ops::Mul for Expr<A> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        ExprRepr::Mul { lhs: self, rhs }.into()
    }
}

impl<A: Ast> std::ops::Div for Expr<A> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        ExprRepr::Div { lhs: self, rhs }.into()
    }
}

impl<A: Ast> Expr<A> {
    pub fn bind(param: A::Param, param_type: A::Type, expr: Expr<A>) -> Stmt<A> {
        StmtRepr::Let {
            param,
            param_type,
            expr,
        }
        .into()
    }

    pub fn bind_mut(param: A::Param, param_type: A::Type, expr: Expr<A>) -> Stmt<A> {
        StmtRepr::LetMut {
            param,
            param_type,
            expr,
        }
        .into()
    }

    pub fn assign(self, rhs: Expr<A>) -> Stmt<A> {
        StmtRepr::Assign { lhs: self, rhs }.into()
    }

    pub fn add_assign(self, rhs: Expr<A>) -> Stmt<A> {
        StmtRepr::AddAssign { lhs: self, rhs }.into()
    }

    pub fn sub_assign(self, rhs: Expr<A>) -> Stmt<A> {
        StmtRepr::SubAssign { lhs: self, rhs }.into()
    }

    pub fn mul_assign(self, rhs: Expr<A>) -> Stmt<A> {
        StmtRepr::MulAssign { lhs: self, rhs }.into()
    }

    pub fn div_assign(self, rhs: Expr<A>) -> Stmt<A> {
        StmtRepr::DivAssign { lhs: self, rhs }.into()
    }
}

pub enum StmtRepr<A: Ast> {
    Expr {
        expr: Expr<A>,
    },
    Let {
        param: A::Param,
        param_type: A::Type,
        expr: Expr<A>,
    },
    LetMut {
        param: A::Param,
        param_type: A::Type,
        expr: Expr<A>,
    },
    Assign {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    AddAssign {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    SubAssign {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    MulAssign {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
    DivAssign {
        lhs: Expr<A>,
        rhs: Expr<A>,
    },
}

pub struct Stmt<A: Ast> {
    pub repr: Box<StmtRepr<A>>,
}

impl<A: Ast> From<StmtRepr<A>> for Stmt<A> {
    fn from(repr: StmtRepr<A>) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}
