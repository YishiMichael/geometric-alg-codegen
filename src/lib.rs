use itertools::Itertools;
use symbol::Symbol;

trait EvalType<Type, const GENERICS: usize, const ASSOCIATES: usize> {
    type OutputType;

    fn eval_type(
        &self,
        generic_types: &[Type; GENERICS],
        associate_types: &[Type; ASSOCIATES],
    ) -> Self::OutputType;
}

struct Context<Pretype, Type> {
    function: <Self as std::ops::Index<ProtocolFunction>>::Output,
    method_expr: <Self as std::ops::Index<ProtocolMethodExpr>>::Output,
    method_stmt: <Self as std::ops::Index<ProtocolMethodStmt>>::Output,
}

struct OperationGroup<Protocol, Pretype, ReturnPretype, Type, ReturnType, Body> {
    signatures: Vec<OperationSignatureErased<Protocol, Pretype, ReturnPretype>>,
    implementations: Vec<(
        ImplementationSignatureErased<Protocol, Type, ReturnType>,
        Body,
    )>,
}

// struct ImplementationBody {
//     stmts: Vec<Stmt>,
//     return_expr: Option<Expr>,
// }

// struct Struct<Type> {
//     ident: Symbol,
//     r#type: Type,
//     fields: Vec<Field<Type>>,
// }

// struct Field<Type> {
//     ident: Symbol,
//     r#type: Type,
// }

trait OperationProtocol<Pretype, Type> {
    type ReturnPretype;
    type ReturnType;
    type Body;
}

#[derive(Clone, Copy)]
struct ProtocolFunction;

impl<Pretype, Type> OperationProtocol<Pretype, Type> for ProtocolFunction {
    type ReturnPretype = Pretype;
    type ReturnType = Type;
    type Body = Expr;
}

impl<Pretype, Type> std::ops::Index<ProtocolFunction> for Context<Pretype, Type> {
    type Output = OperationGroup<
        ProtocolFunction,
        Pretype,
        <ProtocolFunction as OperationProtocol<Pretype, Type>>::ReturnPretype,
        Type,
        <ProtocolFunction as OperationProtocol<Pretype, Type>>::ReturnType,
        <ProtocolFunction as OperationProtocol<Pretype, Type>>::Body,
    >;

    fn index(&self, _index: ProtocolFunction) -> &Self::Output {
        &self.function
    }
}

impl<Pretype, Type> std::ops::IndexMut<ProtocolFunction> for Context<Pretype, Type> {
    fn index_mut(&mut self, _index: ProtocolFunction) -> &mut Self::Output {
        &mut self.function
    }
}

#[derive(Clone, Copy)]
struct ProtocolMethodExpr;

impl<Pretype, Type> OperationProtocol<Pretype, Type> for ProtocolMethodExpr {
    type ReturnPretype = Pretype;
    type ReturnType = Type;
    type Body = Expr;
}

impl<Pretype, Type> std::ops::Index<ProtocolMethodExpr> for Context<Pretype, Type> {
    type Output = OperationGroup<
        ProtocolMethodExpr,
        Pretype,
        <ProtocolMethodExpr as OperationProtocol<Pretype, Type>>::ReturnPretype,
        Type,
        <ProtocolMethodExpr as OperationProtocol<Pretype, Type>>::ReturnType,
        <ProtocolMethodExpr as OperationProtocol<Pretype, Type>>::Body,
    >;

    fn index(&self, _index: ProtocolMethodExpr) -> &Self::Output {
        &self.method_expr
    }
}

impl<Pretype, Type> std::ops::IndexMut<ProtocolMethodExpr> for Context<Pretype, Type> {
    fn index_mut(&mut self, _index: ProtocolMethodExpr) -> &mut Self::Output {
        &mut self.method_expr
    }
}

#[derive(Clone, Copy)]
struct ProtocolMethodStmt;

impl<Pretype, Type> OperationProtocol<Pretype, Type> for ProtocolMethodStmt {
    type ReturnPretype = ();
    type ReturnType = ();
    type Body = Vec<Stmt>;
}

impl<Pretype, Type> std::ops::Index<ProtocolMethodStmt> for Context<Pretype, Type> {
    type Output = OperationGroup<
        ProtocolMethodStmt,
        Pretype,
        <ProtocolMethodStmt as OperationProtocol<Pretype, Type>>::ReturnPretype,
        Type,
        <ProtocolMethodStmt as OperationProtocol<Pretype, Type>>::ReturnType,
        <ProtocolMethodStmt as OperationProtocol<Pretype, Type>>::Body,
    >;

    fn index(&self, _index: ProtocolMethodStmt) -> &Self::Output {
        &self.method_stmt
    }
}

impl<Pretype, Type> std::ops::IndexMut<ProtocolMethodStmt> for Context<Pretype, Type> {
    fn index_mut(&mut self, _index: ProtocolMethodStmt) -> &mut Self::Output {
        &mut self.method_stmt
    }
}

struct OperationSignatureErased<Protocol, Pretype, ReturnPretype> {
    protocol: Protocol,
    ident: Symbol,
    op_ident: Symbol,
    generics: Vec<Symbol>,
    associates: Vec<Symbol>,
    param_items: Vec<(Symbol, Pretype)>,
    return_pretype: ReturnPretype,
    // implementations: Vec<Implementation<Type>>,
    // implementor: Box<dyn Fn(&Context<Pretype, Type>, Vec<Expr>, Type) -> Option<Expr>>, // Lazily evaluated; None = builtin
    // call_expr_repr: Box<dyn Fn(Symbol, Vec<Expr>, &Implementation<Type>) -> ExprRepr>,
}

#[derive(Clone)]
struct OperationSignature<
    Protocol,
    Pretype,
    ReturnPretype,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const PARAMS: usize,
> {
    protocol: Protocol,
    ident: Symbol,
    op_ident: Symbol,
    generics: [Symbol; GENERICS],
    associates: [Symbol; ASSOCIATES],
    param_items: [(Symbol, Pretype); PARAMS],
    return_pretype: ReturnPretype,
}

impl<
        Protocol,
        Pretype,
        ReturnPretype,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const PARAMS: usize,
    > OperationSignature<Protocol, Pretype, ReturnPretype, GENERICS, ASSOCIATES, PARAMS>
{
    fn erase(self) -> OperationSignatureErased<Protocol, Pretype, ReturnPretype> {
        OperationSignatureErased {
            protocol: self.protocol,
            ident: self.ident,
            op_ident: self.op_ident,
            generics: Vec::from(self.generics),
            associates: Vec::from(self.associates),
            param_items: Vec::from(self.param_items),
            return_pretype: self.return_pretype,
        }
    }
}

struct ImplementationSignatureErased<Protocol, Type, ReturnType> {
    protocol: Protocol,
    ident: Symbol,
    op_ident: Symbol,
    generic_items: Vec<(Symbol, Type)>,
    associate_items: Vec<(Symbol, Type)>,
    param_items: Vec<(Symbol, Type)>,
    return_type: ReturnType,
}

#[derive(Clone)]
struct ImplementationSignature<
    Protocol,
    Type,
    ReturnType,
    const GENERICS: usize,
    const ASSOCIATES: usize,
    const PARAMS: usize,
> {
    protocol: Protocol,
    ident: Symbol,
    op_ident: Symbol,
    generic_items: [(Symbol, Type); GENERICS],
    associate_items: [(Symbol, Type); ASSOCIATES],
    param_items: [(Symbol, Type); PARAMS],
    return_type: ReturnType,
}

impl<
        Protocol,
        Type,
        ReturnType,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const PARAMS: usize,
    > ImplementationSignature<Protocol, Type, ReturnType, GENERICS, ASSOCIATES, PARAMS>
{
    fn erase(self) -> ImplementationSignatureErased<Protocol, Type, ReturnType> {
        ImplementationSignatureErased {
            protocol: self.protocol,
            ident: self.ident,
            op_ident: self.op_ident,
            generic_items: Vec::from(self.generic_items),
            associate_items: Vec::from(self.associate_items),
            param_items: Vec::from(self.param_items),
            return_type: self.return_type,
        }
    }
}

impl<Pretype, Type> Context<Pretype, Type> {
    // fn register_struct<const FIELDS: usize>(
    //     &mut self,
    //     ident: impl AsRef<str>,
    //     r#type: Type,
    //     field_items: [(impl AsRef<str>, Type); FIELDS],
    // ) -> &mut Self {
    //     self.structs.push(Struct {
    //         ident: Symbol::from(ident),
    //         r#type,
    //         fields: Vec::from(field_items.map(|(field_ident, field_type)| Field {
    //             ident: Symbol::from(field_ident),
    //             r#type: field_type,
    //         })),
    //     });
    //     self
    // }

    fn register_operation<
        Protocol,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const PARAMS: usize,
    >(
        &mut self,
        protocol: Protocol,
        ident: impl AsRef<str>,
        op_ident: impl AsRef<str>,
        generics: [impl AsRef<str>; GENERICS],
        associates: [impl AsRef<str>; ASSOCIATES],
        param_items: [(impl AsRef<str>, Pretype); PARAMS],
        return_pretype: Protocol::ReturnPretype,
    ) -> OperationSignature<Protocol, Pretype, Protocol::ReturnPretype, GENERICS, ASSOCIATES, PARAMS>
    where
        Protocol: Clone + OperationProtocol<Pretype, Type>,
        Pretype: Clone,
        Protocol::ReturnPretype: Clone,
        Self: std::ops::IndexMut<
            Protocol,
            Output = OperationGroup<
                Protocol,
                Pretype,
                Protocol::ReturnPretype,
                Type,
                Protocol::ReturnType,
                Protocol::Body,
            >,
        >,
    {
        let operation_signature = OperationSignature {
            protocol,
            ident: Symbol::from(ident),
            op_ident: Symbol::from(op_ident),
            generics: generics.map(Symbol::from),
            associates: associates.map(Symbol::from),
            param_items: param_items
                .map(|(param_ident, param_pretype)| (Symbol::from(param_ident), param_pretype)),
            return_pretype,
        };
        self[operation_signature.protocol.clone()]
            .signatures
            .push(operation_signature.clone().erase());
        operation_signature
    }

    fn register_implementation<
        Protocol,
        const GENERICS: usize,
        const ASSOCIATES: usize,
        const PARAMS: usize,
    >(
        &mut self,
        operation_signature: &OperationSignature<
            Protocol,
            Pretype,
            Protocol::ReturnPretype,
            GENERICS,
            ASSOCIATES,
            PARAMS,
        >,
        generic_types: [Type; GENERICS],
        associate_types: [Type; ASSOCIATES],
        body: Protocol::Body,
    ) -> ImplementationSignature<Protocol, Type, Protocol::ReturnType, GENERICS, ASSOCIATES, PARAMS>
    where
        Protocol: Clone + OperationProtocol<Pretype, Type>,
        Pretype: EvalType<Type, GENERICS, ASSOCIATES, OutputType = Type>,
        Protocol::ReturnPretype:
            EvalType<Type, GENERICS, ASSOCIATES, OutputType = Protocol::ReturnType>,
        Type: Clone,
        Protocol::ReturnType: Clone,
        Self: std::ops::IndexMut<
            Protocol,
            Output = OperationGroup<
                Protocol,
                Pretype,
                Protocol::ReturnPretype,
                Type,
                Protocol::ReturnType,
                Protocol::Body,
            >,
        >,
    {
        let param_items =
            operation_signature
                .param_items
                .each_ref()
                .map(|(param_ident, param_pretype)| {
                    (
                        *param_ident,
                        param_pretype.eval_type(&generic_types, &associate_types),
                    )
                });
        let return_type = operation_signature
            .return_pretype
            .eval_type(&generic_types, &associate_types);
        let implementation_signature = ImplementationSignature {
            protocol: operation_signature.protocol.clone(),
            ident: operation_signature.ident,
            op_ident: operation_signature.op_ident,
            generic_items: operation_signature
                .generics
                .iter()
                .cloned()
                .zip_eq(generic_types)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap(),
            associate_items: operation_signature
                .associates
                .iter()
                .cloned()
                .zip_eq(associate_types)
                .collect::<Vec<_>>()
                .try_into()
                .ok()
                .unwrap(),
            param_items,
            return_type,
        };
        self[implementation_signature.protocol.clone()]
            .implementations
            .push((implementation_signature.clone().erase(), body));
        implementation_signature
    }
}

// trait Subspace<Basis, Type> {
//     fn get_type(&self) -> Type;
//     fn get_coeff_expr(&self, basis: &Basis, expr: Expr) -> Option<Expr>;
// }

enum ExprRepr {
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
        generic_types: Vec<Symbol>,
        param_exprs: Vec<Expr>,
    },
    CallMethod {
        self_expr: Expr,
        ident: Symbol,
        op_ident: Symbol,
        generic_types: Vec<Symbol>,
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

struct Expr {
    repr: Box<ExprRepr>,
}

impl From<ExprRepr> for Expr {
    fn from(repr: ExprRepr) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}

// trait TypeSymbol {
//     fn to_symbol(&self) -> Symbol
// }

impl Expr {
    fn variable(ident: Symbol) -> Self {
        ExprRepr::Variable { ident }.into()
    }

    fn literal(value: u32) -> Self {
        ExprRepr::Literal { value }.into()
    }

    fn r#struct(ident: Symbol, fields: Vec<(Symbol, Self)>) -> Self {
        ExprRepr::Struct { ident, fields }.into()
    }

    fn field(self, ident: Symbol) -> Self {
        ExprRepr::Field { expr: self, ident }.into()
    }

    // fn call_function<Type, const GENERICS: usize, const ASSOCIATES: usize, const PARAMS: usize>(
    //     implementation_signature: &ImplementationSignature<
    //         ProtocolFunction,
    //         Type,
    //         Type,
    //         GENERICS,
    //         ASSOCIATES,
    //         PARAMS,
    //     >,
    //     param_exprs: [Self; PARAMS],
    // ) -> Self {
    //     ExprRepr::CallFunction {
    //         ident: implementation_signature.ident,
    //         op_ident: implementation_signature.op_ident,
    //         generic_types: implementation_signature.generic_items,
    //         param_exprs: (),
    //     }
    // }
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

enum StmtRepr {
    CallMethod {
        self_expr: Expr,
        ident: Symbol,
        op_ident: Symbol,
        generic_types: Vec<Symbol>,
        param_exprs: Vec<Expr>,
    },
    AddAssign {
        lhs: Expr,
        rhs: Expr,
    },
    SubAssign {
        lhs: Expr,
        rhs: Expr,
    },
    MulAssign {
        lhs: Expr,
        rhs: Expr,
    },
    DivAssign {
        lhs: Expr,
        rhs: Expr,
    },
}

struct Stmt {
    repr: Box<StmtRepr>,
}

impl From<StmtRepr> for Stmt {
    fn from(repr: StmtRepr) -> Self {
        Self {
            repr: Box::new(repr),
        }
    }
}

// impl<Pretype, Type> Context<Pretype, Type>
// where
//     Type: TypeTrait,
// {
//     fn variable(&self, ident: Symbol, r#type: Type) -> Expr {
//         Expr {
//             repr: Box::new(ExprRepr::Variable { ident }),
//             r#type,
//         }
//     }

//     fn literal(&self, value: u8) -> Expr {
//         Expr {
//             repr: Box::new(ExprRepr::Literal { value }),
//             r#type: Type::literal_type(),
//         }
//     }

//     fn r#struct(&self, ident: Symbol, field_expr_fn: impl Fn(Symbol) -> Expr) -> Expr {
//         let r#struct = self
//             .structs
//             .iter()
//             .filter(|r#struct| r#struct.ident == ident)
//             .exactly_one()
//             .ok()
//             .unwrap();
//         Expr {
//             repr: Box::new(ExprRepr::Struct {
//                 ident,
//                 fields: r#struct
//                     .fields
//                     .iter()
//                     .map(|field| {
//                         let field_expr = field_expr_fn(field.ident);
//                         (field.ident, field_expr)
//                     })
//                     .collect(),
//             }),
//             r#type: r#struct.r#type.clone(),
//         }
//     }

//     fn field(&self, expr: Expr, ident: Symbol) -> Expr {
//         let r#struct = self
//             .structs
//             .iter()
//             .filter(|r#struct| r#struct.r#type == expr.r#type)
//             .exactly_one()
//             .ok()
//             .unwrap();
//         let field = r#struct
//             .fields
//             .iter()
//             .filter(|field| field.ident == ident)
//             .exactly_one()
//             .ok()
//             .unwrap();
//         Expr {
//             repr: Box::new(ExprRepr::Field { expr, ident }),
//             r#type: field.r#type.clone(),
//         }
//     }

//     fn call<const PARAMS: usize>(
//         &self,
//         op_ident: Symbol,
//         param_exprs: [Expr; PARAMS],
//     ) -> Expr {
//         let r#trait = self
//             .traits
//             .iter()
//             .filter(|r#trait| r#trait.op_ident == op_ident)
//             .exactly_one()
//             .ok()
//             .unwrap();
//         let implementation = r#trait
//             .implementations
//             .iter()
//             .filter(|implementation| {
//                 implementation
//                     .param_types
//                     .iter()
//                     .zip_eq(param_exprs.each_ref())
//                     .all(|(param_type, param_expr)| param_type == &param_expr.r#type)
//             })
//             .exactly_one()
//             .ok()
//             .unwrap();
//         Expr {
//             repr: Box::new((r#trait.call_expr_repr)(
//                 op_ident,
//                 Vec::from(param_exprs),
//                 implementation,
//             )),
//             r#type: implementation.return_type.clone(),
//         }
//     }
// }

// struct Context {
//     map: std::collections::BTreeMap<()>
// }

//
type Coefficient = i32;

#[derive(Clone, Copy)]
enum Sign {
    POS,
    NEG,
}

impl std::ops::BitXor for Sign {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::POS, Self::POS) => Self::POS,
            (Self::POS, Self::NEG) => Self::NEG,
            (Self::NEG, Self::POS) => Self::NEG,
            (Self::NEG, Self::NEG) => Self::POS,
        }
    }
}

impl From<usize> for Sign {
    fn from(value: usize) -> Self {
        if 1 & value == 0 {
            Self::POS
        } else {
            Self::NEG
        }
    }
}

impl From<Sign> for Coefficient {
    fn from(value: Sign) -> Self {
        match value {
            Sign::POS => 1,
            Sign::NEG => -1,
        }
    }
}

struct Blade {
    ident: Symbol,
    generator_bits: usize,
    intrinsic_sign: Sign,
}

struct GeometricAlgebraData {
    ident: &'static str,
    signs: &'static str,
    blades: &'static str,
}

struct GeometricAlgebra {
    dimension: usize,
    signs: Vec<Coefficient>,
    blades: Vec<Blade>,
}

enum GAType {
    Primitive,
    Compound(Vec<Blade>),
}

enum GAPretype<const GENERICS: usize, const ASSOCIATES: usize> {
    Primitive,
    GenericBinding(usize),
    AssociateBinding(usize),
}

const ALGEBRAS: &[GeometricAlgebraData] = &[
    GeometricAlgebraData {
        ident: "epga1d",
        signs: "+ -",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraData {
        ident: "ppga1d",
        signs: "+ 0",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraData {
        ident: "hpga1d",
        signs: "+ +",
        blades: "s e0 e1 e01",
    },
    GeometricAlgebraData {
        ident: "epga2d",
        signs: "+ + -",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraData {
        ident: "ppga2d",
        signs: "+ + 0",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraData {
        ident: "hpga2d",
        signs: "+ + +",
        blades: "s e0 e1 e01 e2 -e20 e12 e012",
    },
    GeometricAlgebraData {
        ident: "epga3d",
        signs: "+ + + -",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraData {
        ident: "ppga3d",
        signs: "+ + + 0",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
    GeometricAlgebraData {
        ident: "hpga3d",
        signs: "+ + + +",
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123",
    },
];

trait Emitter<Pretype, Type> {
    const SUFFIX: &str;

    fn emit_function_operation(
        &self,
        buf: &mut String,
        operation: &OperationSignatureErased<ProtocolFunction, Pretype, Pretype>,
    );
    fn emit_method_expr_operation(
        &self,
        buf: &mut String,
        operation: &OperationSignatureErased<ProtocolMethodExpr, Pretype, Pretype>,
    );
    fn emit_method_stmt_operation(
        &self,
        buf: &mut String,
        operation: &OperationSignatureErased<ProtocolMethodStmt, Pretype, ()>,
    );
    // fn emit_
}

struct StructSignature<Type> {
    ident: Symbol,
    definition: Vec<(Symbol, Type)>,
}

fn f() {
    let alg = GeometricAlgebra {
        generator_squares: Vec::from([1, 1, 1, -1]),
        blades: "s e0 e1 e01 e2 e02 e12 -e021 e3 e03 -e31 e013 e23 -e032 e123 e0123"
            .split_whitespace()
            .enumerate()
            .map(|(generator_bits, name)| match name.strip_prefix('-') {
                None => Blade {
                    ident: Symbol::from(name),
                    generator_bits,
                    intrinsic_sign: Sign::POS,
                },
                Some(name) => Blade {
                    ident: Symbol::from(name),
                    generator_bits,
                    intrinsic_sign: Sign::NEG,
                },
            })
            .collect(),
        grade_idents: "Plane Line Point PseudoScalar"
            .split_whitespace()
            .map(Symbol::from)
            .collect(),
        even_ident: Symbol::from("Motor"),
        odd_ident: Symbol::from("Flector"),
        entire_ident: Symbol::from("Multivector"),
    };
    // let geometric_product = OperationSignature {
    //     name: "GeometricProduct",
    //     method_name: "geometric_product",
    //     generics: ["Self", "T"],
    //     associates: ["Output"],
    //     params: [
    //         ("self", GAPretype::GenericBinding(0)),
    //         ("other", GAPretype::GenericBinding(1)),
    //     ],
    //     return_pretype: GAPretype::AssociateBinding(0),
    // };
}
