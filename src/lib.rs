use std::rc::Rc;

/// Count of negative signs.
// type Sign = u32;

// type SpaceIndex = usize;

// #[repr(transparent)]
// #[derive(Clone, Copy)]
// struct BladeIndex {
//     generator_bits: usize,
// }

// struct Blade {
//     name: String,
//     gray_inv: usize,
//     intrinsic_sign: Sign,
// }

// #[repr(transparent)]
// #[derive(Clone, Copy)]
// struct SpaceIndex {
//     grade_bits: usize,
// }

// struct Space {
//     name: String,
// }

// trait GrayCodeInv<const DIM: u32> {
//     fn gray_code_inv(self) -> Self;
// }

// impl GrayCodeInv<0> for BladeIndex {
//     fn gray_code_inv(self) -> Self {
//         0
//     }
// }

// impl<const DIM: u32> GrayCodeInv<DIM> for BladeIndex
// where
//     BladeIndex: GrayCodeInv<{ DIM - 1 }>,
// {
//     fn gray_code_inv(self) -> Self {
//         self >> DIM ^ <BladeIndex as GrayCodeInv<{ DIM - 1 }>>::gray_code_inv(self)
//     }
// }

#[derive(Clone, Copy)]
struct GeometricAlgebraRenames {
    blades: &'static [(usize, &'static str)],
    spaces: &'static [(usize, &'static str)],
}

struct GeometricAlgebraBuilder {
    dim: usize,
    generator_squares: Vec<i8>,
    blades: Vec<String>,
    spaces: Vec<String>,
}

fn generator_squares<V>(generator_squares: V) -> GeometricAlgebraBuilder
where
    V: Into<Vec<i8>>,
{
    let generator_squares = generator_squares.into();
    let dim = generator_squares.len();
    GeometricAlgebraBuilder {
        dim,
        generator_squares: generator_squares,
        blades: (0..1 << dim)
            .map(|blade_index| format!("e{blade_index}"))
            .collect(),
        spaces: (0..1 << (dim + 1))
            .map(|space_index| format!("Space{space_index}"))
            .collect(),
    }
}

impl GeometricAlgebraBuilder {
    fn rename_blade<V>(mut self, index: usize, name: V) -> Self
    where
        V: Into<String>,
    {
        self.blades[index] = name.into();
        self
    }

    fn rename_space<V>(mut self, index: usize, name: V) -> Self
    where
        V: Into<String>,
    {
        self.spaces[index] = name.into();
        self
    }

    fn rename(mut self, renames: GeometricAlgebraRenames) -> Self {
        renames
            .blades
            .into_iter()
            .for_each(|&(index, name)| self.blades[index] = name.into());
        renames
            .spaces
            .into_iter()
            .for_each(|&(index, name)| self.spaces[index] = name.into());
        self
    }
}

impl From<GeometricAlgebraBuilder> for GeometricAlgebra {
    fn from(value: GeometricAlgebraBuilder) -> Self {
        let dim = value.dim;
        let ref generator_squares = value.generator_squares;
        let (blades, ref intrinsic_signs): (Vec<_>, Vec<_>) = value
            .blades
            .into_iter()
            .map(|blade| match blade.strip_prefix('-') {
                None => (blade, 1i8),
                Some(blade) => (blade.to_string(), -1i8),
            })
            .unzip();
        let spaces = value.spaces;
        let metric = (0usize..1 << dim)
            .flat_map(|index_0| {
                let gray_inv = (0..dim)
                    .map(|grade| index_0 >> grade)
                    .fold(0, std::ops::BitXor::bitxor);
                (0usize..1 << dim).map(move |index_1| {
                    generator_squares
                        .iter()
                        .enumerate()
                        .filter_map(|(generator, generator_square)| {
                            (index_0 & index_1 & 1 << generator != 0).then_some(generator_square)
                        })
                        .product::<i8>()
                        * (-1i8).pow(1 & (gray_inv >> 1 & index_1).count_ones())
                        * intrinsic_signs[index_0]
                        * intrinsic_signs[index_1]
                        * intrinsic_signs[index_0 ^ index_1]
                })
            })
            .collect();
        Self {
            dim,
            metric,
            blades,
            spaces,
        }
    }
}

struct Term<T> {
    value: T,
    coeff: i8,
}

struct Terms<T>(std::collections::BTreeMap<T, i8>);

impl<T: std::cmp::Ord> FromIterator<Term<T>> for Terms<T> {
    fn from_iter<I: IntoIterator<Item = Term<T>>>(iter: I) -> Self {
        let mut map = std::collections::BTreeMap::new();
        iter.into_iter().for_each(|Term { value, coeff }| {
            *map.entry(value).or_default() += coeff;
        });
        Self(map)
    }
}

impl<T> IntoIterator for Terms<T> {
    type Item = Term<T>;
    type IntoIter = std::iter::Map<
        <std::collections::BTreeMap<T, i8> as IntoIterator>::IntoIter,
        fn((T, i8)) -> Term<T>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(value, coeff)| Term { value, coeff })
    }
}

struct GeometricAlgebra {
    dim: usize,
    metric: Vec<i8>,
    blades: Vec<String>,
    spaces: Vec<String>,
}

impl GeometricAlgebra {
    // fn grade(index: BladeIndex) -> u32 {
    //     index.count_ones()
    // }

    // fn gray_code_inv(index: BladeIndex, DIM: u32) -> usize {
    //     (0..DIM)
    //         .map(|grade| index >> grade)
    //         .fold(0, std::ops::BitXor::bitxor)
    // }

    fn space_blades(&self, space: usize) -> Vec<usize> {
        (0usize..1 << self.dim)
            .filter(|&blade| space & 1 << blade.count_ones() != 0)
            .collect()
    }

    fn blade_mul(&self, blade_0: usize, blade_1: usize) -> (usize, i8) {
        // blades[blade_0] * blades[blade_1] = blades[blade] * coeff
        (
            blade_0 ^ blade_1,
            self.metric[blade_0 << self.dim | blade_1],
        )
    }

    // fn space_mul(&self, space_0: usize, space_1: usize) -> Terms<(usize, usize)> {
    //     // spaces[space_0] * spaces[space_1] = sum(blades[blade_0] * blades[blade_1] * coeff)
    //     self.space_blades(space_0)
    // }

    // fn geometric_product_impls(&self) -> Vec<OperationImpl<1, 1>> {
    //     (0usize..1 << (self.dim + 1)).zip((0usize..1 << (self.dim + 1))).map(|(space_0, space_1)| {
    //         OperationImpl {
    //             trait_ident: "GeometricProduct".into(),
    //             generics: [Type::Struct(space_0)],
    //             self_variable: ,
    //             arg_variables: ,
    //             output_type: ,
    //             statements: ,
    //             return_expression: ,
    //         }
    //     }).collect()
    // }
}

// struct Signature<Generics, Args, ReturnType> {
//     generics: Generics,
//     args: Args,
//     return_type: ReturnType,
// }

// struct Return<Type> {
//     ty: Type,
// }

// trait ReturnType {
//     type Expr;
// }

// impl ReturnType for () {
//     type Expr = ();
// }

// impl<Type> ReturnType for Return<Type> {
//     type Expr = TypedExpr<Type>;
// }

struct OperationSignature<Operation, Generics, Args, ReturnType> {
    operation: Operation,
    generics: Generics,
    args: Args,
    return_type: ReturnType,
}

struct OperationBody<ReturnExpr> {
    statements: Vec<Box<dyn Statement>>,
    return_expr: ReturnExpr,
}

// struct AssignmentImpl<Generics, Args> {
//     trait_ident: Rc<str>,
//     generics: Generics,
//     arg_variables: Args,
//     statements: Vec<Statement>,
// }

trait Operation<Generics> {
    const NAME: &'static str;

    type Signature;
    type Body;

    fn signatures(&self, alg: &GeometricAlgebra) -> Vec<Self::Signature>;
    fn body(&self, signature: &Self::Signature) -> Self::Body;
}

// trait Assignment<Generics, Args> {
//     const NAME: &'static str;

//     type Generics;
//     type Args;

//     fn impls(&self, alg: &GeometricAlgebra) -> Vec<OperationImpl<Self::Generics, Self::Args>>;
// }

// trait Operation<Generics, Vars> {
//     type OutputType: Type;

//     fn generics(&self) ->
//     fn output_type(&self, generics: &Generics) -> Self::OutputType;
//     fn statements(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
//     fn expression(&self, generics: &Generics, vars: &Vars) -> Expression;
// }

// trait InplaceOperation<Generics, Vars> {
//     fn statements(&self, generics: &Generics, vars: &Vars) -> Vec<Statement>;
// }

// enum Type {
//     Float,
//     Space(usize),
//     PtrSpace(usize),
// }

#[derive(Clone, Copy, PartialEq)]
struct TypeAtom;
#[derive(Clone, Copy, PartialEq)]
struct TypeSpace(usize);
#[derive(Clone, Copy, PartialEq)]
struct TypeSpacePtr(usize);

struct Variable<Type> {
    name: Rc<str>,
    ty: Type,
}

struct TypedExpr<Type> {
    ty: Type,
    expression: Box<dyn Expression<Type = Type>>,
}

impl<Type, Expr: Expression<Type = Type> + 'static> From<Expr> for TypedExpr<Type> {
    fn from(expr: Expr) -> Self {
        Self {
            ty: expr.ty(),
            expression: Box::new(expr),
        }
    }
}

trait Expression {
    type Type;

    fn ty(&self) -> Self::Type;
}

struct ExprVariable<Type> {
    variable: Variable<Type>,
}

impl<Type: Copy> Expression for ExprVariable<Type> {
    type Type = Type;

    fn ty(&self) -> Self::Type {
        self.variable.ty
    }
}

struct ExprFieldAccess {
    expr: TypedExpr<TypeSpace>,
    blade_index: usize,
}

impl Expression for ExprFieldAccess {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprPtrFieldAccess {
    expr: TypedExpr<TypeSpacePtr>,
    blade_index: usize,
}

impl Expression for ExprPtrFieldAccess {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprLiteral {
    value: u8,
}

impl Expression for ExprLiteral {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprStruct {
    name: Rc<str>,
    space: usize,
    fields: Vec<(Rc<str>, TypedExpr<TypeAtom>)>,
}

impl Expression for ExprStruct {
    type Type = TypeSpace;

    fn ty(&self) -> Self::Type {
        TypeSpace(self.space)
    }
}

struct ExprGroup<Type> {
    expr: TypedExpr<Type>,
}

impl<Type: Copy> Expression for ExprGroup<Type> {
    type Type = Type;

    fn ty(&self) -> Self::Type {
        self.expr.ty
    }
}

struct ExprNeg {
    expr: TypedExpr<TypeAtom>,
}

impl Expression for ExprNeg {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprAdd {
    lhs: TypedExpr<TypeAtom>,
    rhs: TypedExpr<TypeAtom>,
}

impl Expression for ExprAdd {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprSub {
    lhs: TypedExpr<TypeAtom>,
    rhs: TypedExpr<TypeAtom>,
}

impl Expression for ExprSub {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprMul {
    lhs: TypedExpr<TypeAtom>,
    rhs: TypedExpr<TypeAtom>,
}

impl Expression for ExprMul {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprDiv {
    lhs: TypedExpr<TypeAtom>,
    rhs: TypedExpr<TypeAtom>,
}

impl Expression for ExprDiv {
    type Type = TypeAtom;

    fn ty(&self) -> Self::Type {
        TypeAtom
    }
}

struct ExprCall1<Operation, Generics, Type0, Type> {
    signature: OperationSignature<Operation, Generics, (Variable<Type0>,), Type>,
    arg_0: TypedExpr<Type0>,
}

impl<Operation, Generics, Type0: PartialEq + std::fmt::Debug, Type: Copy> Expression
    for ExprCall1<Operation, Generics, Type0, Type>
{
    type Type = Type;

    fn ty(&self) -> Self::Type {
        assert_eq!(self.arg_0.ty, self.signature.args.0.ty);
        self.signature.return_type
    }
}

struct ExprCall2<Operation, Generics, Type0, Type1, Type> {
    signature: OperationSignature<Operation, Generics, (Variable<Type0>, Variable<Type1>), Type>,
    arg_0: TypedExpr<Type0>,
    arg_1: TypedExpr<Type1>,
}

impl<
        Operation,
        Generics,
        Type0: PartialEq + std::fmt::Debug,
        Type1: PartialEq + std::fmt::Debug,
        Type: Copy,
    > Expression for ExprCall2<Operation, Generics, Type0, Type1, Type>
{
    type Type = Type;

    fn ty(&self) -> Self::Type {
        assert_eq!(self.arg_0.ty, self.signature.args.0.ty);
        assert_eq!(self.arg_1.ty, self.signature.args.1.ty);
        self.signature.return_type
    }
}

// enum Expression {
//     Variable {
//         binding: usize,
//     },
//     FieldAccess {
//         binding: usize,
//         blade_index: usize,
//     },
//     Literal {
//         value: i8,
//     },
//     Struct {
//         name: Rc<str>,
//         fields: Vec<(Rc<str>, Self)>,
//     },
//     Group {
//         expression: Box<Self>,
//     },
//     Neg {
//         expression: Box<Self>,
//     },
//     Add {
//         lhs: Box<Self>,
//         rhs: Box<Self>,
//     },
//     Sub {
//         lhs: Box<Self>,
//         rhs: Box<Self>,
//     },
//     Mul {
//         lhs: Box<Self>,
//         rhs: Box<Self>,
//     },
//     Div {
//         lhs: Box<Self>,
//         rhs: Box<Self>,
//     },
//     CallOperation {
//         qualname: Rc<str>,
//         self_input: Box<Self>,
//         arg_inputs: Vec<Self>,
//     },
// }

trait Statement {
    fn ty(&self) {}
}

struct StmtAddAssign {
    expr: TypedExpr<TypeSpacePtr>,
    blade_index: usize,
    rhs: TypedExpr<TypeAtom>,
}

impl Statement for StmtAddAssign {}

struct StmtSubAssign {
    expr: TypedExpr<TypeSpacePtr>,
    blade_index: usize,
    rhs: TypedExpr<TypeAtom>,
}

impl Statement for StmtSubAssign {}

struct StmtMulAssign {
    expr: TypedExpr<TypeSpacePtr>,
    blade_index: usize,
    rhs: TypedExpr<TypeAtom>,
}

impl Statement for StmtMulAssign {}

struct StmtDivAssign {
    expr: TypedExpr<TypeSpacePtr>,
    blade_index: usize,
    rhs: TypedExpr<TypeAtom>,
}

impl Statement for StmtDivAssign {}

struct StmtCall1<Operation, Generics, Type0> {
    signature: OperationSignature<Operation, Generics, (Variable<Type0>,), ()>,
    arg_0: TypedExpr<Type0>,
}

impl<Operation, Generics, Type0: PartialEq + std::fmt::Debug> Statement
    for StmtCall1<Operation, Generics, Type0>
{
    fn ty(&self) {
        assert_eq!(self.arg_0.ty, self.signature.args.0.ty);
    }
}

struct StmtCall2<Operation, Generics, Type0, Type1> {
    signature: OperationSignature<Operation, Generics, (Variable<Type0>, Variable<Type1>), ()>,
    arg_0: TypedExpr<Type0>,
    arg_1: TypedExpr<Type1>,
}

impl<
        Operation,
        Generics,
        Type0: PartialEq + std::fmt::Debug,
        Type1: PartialEq + std::fmt::Debug,
    > Statement for StmtCall2<Operation, Generics, Type0, Type1>
{
    fn ty(&self) {
        assert_eq!(self.arg_0.ty, self.signature.args.0.ty);
        assert_eq!(self.arg_1.ty, self.signature.args.1.ty);
    }
}

// enum Statement {
//     AddAssign {
//         expr: TypedExpr<TypeSpacePtr>,
//         blade_index: usize,
//         rhs: TypedExpr<TypeAtom>,
//     },
//     SubAssign {
//         expr: TypedExpr<TypeSpacePtr>,
//         blade_index: usize,
//         rhs: TypedExpr<TypeAtom>,
//     },
//     MulAssign {
//         expr: TypedExpr<TypeSpacePtr>,
//         blade_index: usize,
//         rhs: TypedExpr<TypeAtom>,
//     },
//     DivAssign {
//         expr: TypedExpr<TypeSpacePtr>,
//         blade_index: usize,
//         rhs: TypedExpr<TypeAtom>,
//     },
//     CallAssignment {
//         qualname: Rc<str>,
//         mut_self_input: Rc<str>,
//         arg_inputs: Vec<Expression>,
//     },
// }

const PGA1D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[(0b00, "s"), (0b01, "e0"), (0b10, "e1"), (0b11, "e01")],
    spaces: &[
        (0b001, "scalar"),
        (0b010, "point"),
        (0b101, "motor"),
        (0b010, "flector"),
        (0b111, "multivector"),
    ],
};
const PGA2D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[
        (0b000, "s"),
        (0b001, "e0"),
        (0b010, "e1"),
        (0b011, "e01"),
        (0b100, "e2"),
        (0b101, "-e20"),
        (0b110, "e12"),
        (0b111, "e012"),
    ],
    spaces: &[
        (0b0001, "scalar"),
        (0b0010, "line"),
        (0b0010, "point"),
        (0b0101, "motor"),
        (0b1010, "flector"),
        (0b1111, "multivector"),
    ],
};
const PGA3D_RENAMES: GeometricAlgebraRenames = GeometricAlgebraRenames {
    blades: &[
        (0b0000, "s"),
        (0b0001, "e0"),
        (0b0010, "e1"),
        (0b0011, "e01"),
        (0b0100, "e2"),
        (0b0101, "e02"),
        (0b0110, "e12"),
        (0b0111, "-e021"),
        (0b1000, "e3"),
        (0b1001, "e03"),
        (0b1010, "-e31"),
        (0b1011, "e013"),
        (0b1100, "e23"),
        (0b1101, "-e032"),
        (0b1110, "e123"),
        (0b1111, "e0123"),
    ],
    spaces: &[
        (0b00001, "scalar"),
        (0b00010, "plane"),
        (0b00100, "line"),
        (0b01000, "point"),
        (0b10101, "motor"),
        (0b01010, "flector"),
        (0b11111, "multivector"),
    ],
};

fn algebras() -> Vec<(&'static str, GeometricAlgebra)> {
    [
        (
            "epga1d",
            generator_squares([1, -1]).rename(PGA1D_RENAMES).into(),
        ),
        (
            "ppga1d",
            generator_squares([1, 0]).rename(PGA1D_RENAMES).into(),
        ),
        (
            "hpga1d",
            generator_squares([1, 1]).rename(PGA1D_RENAMES).into(),
        ),
        (
            "epga2d",
            generator_squares([1, 1, -1]).rename(PGA2D_RENAMES).into(),
        ),
        (
            "ppga2d",
            generator_squares([1, 1, 0]).rename(PGA2D_RENAMES).into(),
        ),
        (
            "hpga2d",
            generator_squares([1, 1, 1]).rename(PGA2D_RENAMES).into(),
        ),
        (
            "epga3d",
            generator_squares([1, 1, 1, -1])
                .rename(PGA3D_RENAMES)
                .into(),
        ),
        (
            "ppga3d",
            generator_squares([1, 1, 1, 0]).rename(PGA3D_RENAMES).into(),
        ),
        (
            "hpga3d",
            generator_squares([1, 1, 1, 1]).rename(PGA3D_RENAMES).into(),
        ),
    ]
    .into()
}

// struct GeometricAlgebra {
//     name: &'static str,
//     generator_squares: &'static str,
//     signed_blades: &'static str,
//     structures: &'static [Structure],
// }

// struct Structure {
//     ty: &'static str,
//     fields: &'static str,
// }
